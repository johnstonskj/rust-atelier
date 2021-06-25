use crate::syntax::ADD_MODEL_SMITHY_VERSION;
use crate::syntax::*;
use crate::REPRESENTATION_NAME;
use atelier_core::error::{ErrorKind, Result as ModelResult, ResultExt};
use atelier_core::io::ModelReader;
use atelier_core::model::shapes::{
    AppliedTraits, HasTraits, ListOrSet, Map as MapShape, MemberShape, Operation, Resource,
    Service, ShapeKind, Simple, StructureOrUnion, TopLevelShape,
};
use atelier_core::model::values::{Value as NodeValue, ValueMap};
use atelier_core::model::{Identifier, Model, NamespaceID, ShapeID};
use atelier_core::syntax::{
    MEMBER_COLLECTION_OPERATIONS, MEMBER_CREATE, MEMBER_DELETE, MEMBER_ERRORS, MEMBER_IDENTIFIERS,
    MEMBER_INPUT, MEMBER_KEY, MEMBER_LIST, MEMBER_MEMBER, MEMBER_OPERATIONS, MEMBER_OUTPUT,
    MEMBER_PUT, MEMBER_READ, MEMBER_RENAME, MEMBER_RESOURCES, MEMBER_UPDATE, MEMBER_VALUE,
    MEMBER_VERSION, MODEL_METADATA, MODEL_SHAPES, SHAPE_APPLY, SHAPE_LIST, SHAPE_MAP,
    SHAPE_OPERATION, SHAPE_RESOURCE, SHAPE_SERVICE, SHAPE_SET, SHAPE_STRUCTURE, SHAPE_UNION,
};
use atelier_core::Version;
use serde_json::{from_reader, Map, Value};
use std::collections::HashMap;
use std::io::Read;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Read a [Model](../atelier_core/model/struct.Model.html) from the JSON AST representation.
///
#[allow(missing_debug_implementations)]
pub struct JsonReader;

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl<'a> Default for JsonReader {
    fn default() -> Self {
        Self {}
    }
}

impl ModelReader for JsonReader {
    fn read(&mut self, r: &mut impl Read) -> ModelResult<Model> {
        let json: Value = from_reader(r).chain_err(|| {
            ErrorKind::Deserialization(
                REPRESENTATION_NAME.to_string(),
                "ModelReader::read".to_string(),
                None,
            )
            .to_string()
        })?;
        self.model(json)
    }
}

impl JsonReader {
    fn model(&self, json: Value) -> ModelResult<Model> {
        if let Value::Object(vs) = json {
            let version = self.version(vs.get(ADD_MODEL_SMITHY_VERSION))?;
            let mut model = Model::new(version);

            let metadata = self.metadata(vs.get(MODEL_METADATA))?;
            for (key, value) in metadata {
                let _ = model.add_metadata(key, value)?;
            }

            let shapes = self.shapes(vs.get(MODEL_SHAPES))?;
            for shape in shapes.into_iter().map(|(_, s)| s) {
                model.add_shape(shape)?;
            }

            return Ok(model);
        }
        Err(ErrorKind::Deserialization(
            REPRESENTATION_NAME.to_string(),
            "JsonReader::model".to_string(),
            None,
        )
        .into())
    }

    fn version(&self, json: Option<&Value>) -> ModelResult<Version> {
        if let Some(Value::String(version)) = json {
            Ok(Version::from_str(version)?)
        } else {
            Err(ErrorKind::Deserialization(
                REPRESENTATION_NAME.to_string(),
                "JsonReader::version".to_string(),
                Some(format!("{:#?}", json)),
            )
            .into())
        }
    }

    fn metadata(&self, json: Option<&Value>) -> ModelResult<ValueMap> {
        let mut metadata: ValueMap = Default::default();
        if let Some(Value::Object(vs)) = json {
            for (k, v) in vs {
                let _ = metadata.insert(k.clone(), self.value(v)?);
            }
        }
        Ok(metadata)
    }

    fn shapes(&self, json: Option<&Value>) -> ModelResult<Vec<(NamespaceID, TopLevelShape)>> {
        let mut shapes: Vec<(NamespaceID, TopLevelShape)> = Default::default();
        if let Some(Value::Object(vs)) = json {
            for (k, v) in vs {
                let id = ShapeID::from_str(k)?;
                let inner = self.shape(v)?;
                let mut shape = TopLevelShape::new(id.clone(), inner);

                if let Some(Value::Object(vs)) = v.get(ADD_SHAPE_KEY_TRAITS) {
                    shape.append_traits(&self.traits(vs)?)?;
                };

                shapes.push((id.namespace().clone(), shape))
            }
        }
        Ok(shapes)
    }

    fn shape(&self, outer: &Value) -> ModelResult<ShapeKind> {
        if let Some(Value::String(s)) = outer.get(ADD_SHAPE_KEY_TYPE) {
            let s = s.as_str();
            return if let Ok(st) = Simple::from_str(s) {
                Ok(ShapeKind::Simple(st))
            } else if s == SHAPE_APPLY {
                Ok(ShapeKind::Unresolved)
            } else if s == SHAPE_LIST {
                Ok(ShapeKind::List(ListOrSet::from(
                    self.member(outer.get(MEMBER_MEMBER))?,
                )))
            } else if s == SHAPE_SET {
                Ok(ShapeKind::Set(ListOrSet::from(
                    self.member(outer.get(MEMBER_MEMBER))?,
                )))
            } else if s == SHAPE_MAP {
                Ok(ShapeKind::Map(MapShape::from(
                    self.member(outer.get(MEMBER_KEY))?,
                    self.member(outer.get(MEMBER_VALUE))?,
                )))
            } else if s == SHAPE_STRUCTURE {
                let members = if let Some(Value::Object(vs)) = outer.get(ADD_SHAPE_KEY_MEMBERS) {
                    self.members(vs)?
                } else {
                    Default::default()
                };
                Ok(ShapeKind::Structure(StructureOrUnion::with_members(
                    members.as_slice(),
                )))
            } else if s == SHAPE_UNION {
                let members = if let Some(Value::Object(vs)) = outer.get(ADD_SHAPE_KEY_MEMBERS) {
                    self.members(vs)?
                } else {
                    Default::default()
                };
                Ok(ShapeKind::Union(StructureOrUnion::with_members(
                    members.as_slice(),
                )))
            } else if s == SHAPE_SERVICE {
                let version = if let Some(Value::String(value)) = outer.get(MEMBER_VERSION) {
                    value.clone()
                } else {
                    return Err(ErrorKind::InvalidVersionNumber(None).into());
                };
                let mut service = Service::new(&version);
                service.append_operations(&self.target_list(outer.get(MEMBER_OPERATIONS))?);
                service.append_resources(&self.target_list(outer.get(MEMBER_RESOURCES))?);
                service.set_renames(self.renames_hash(outer.get(MEMBER_RENAME))?);
                Ok(ShapeKind::Service(service))
            } else if s == SHAPE_OPERATION {
                let mut operation = Operation::default();
                if let Some(input) = self.optional_target(outer.get(MEMBER_INPUT))? {
                    operation.set_input(input);
                }
                if let Some(output) = self.optional_target(outer.get(MEMBER_OUTPUT))? {
                    operation.set_output(output);
                }
                operation.append_errors(&self.target_list(outer.get(MEMBER_ERRORS))?);
                Ok(ShapeKind::Operation(operation))
            } else if s == SHAPE_RESOURCE {
                let mut resource = Resource::default();
                resource.set_identifiers(self.identifiers_hash(outer.get(MEMBER_IDENTIFIERS))?);
                if let Some(create) = self.optional_target(outer.get(MEMBER_CREATE))? {
                    resource.set_create(create);
                }
                if let Some(put) = self.optional_target(outer.get(MEMBER_PUT))? {
                    resource.set_put(put);
                }
                if let Some(read) = self.optional_target(outer.get(MEMBER_READ))? {
                    resource.set_read(read);
                }
                if let Some(update) = self.optional_target(outer.get(MEMBER_UPDATE))? {
                    resource.set_update(update);
                }
                if let Some(delete) = self.optional_target(outer.get(MEMBER_DELETE))? {
                    resource.set_delete(delete);
                }
                if let Some(list) = self.optional_target(outer.get(MEMBER_LIST))? {
                    resource.set_list(list);
                }
                resource.append_operations(&self.target_list(outer.get(MEMBER_OPERATIONS))?);
                resource.append_collection_operations(
                    &self.target_list(outer.get(MEMBER_COLLECTION_OPERATIONS))?,
                );
                resource.append_resources(&self.target_list(outer.get(MEMBER_RESOURCES))?);
                Ok(ShapeKind::Resource(resource))
            } else {
                return Err(ErrorKind::Deserialization(
                    REPRESENTATION_NAME.to_string(),
                    "JsonReader::shape/type".to_string(),
                    Some(format!("{:#?}", outer)),
                )
                .into());
            };
        }
        Err(ErrorKind::Deserialization(
            REPRESENTATION_NAME.to_string(),
            "JsonReader::shape".to_string(),
            Some(format!("{:#?}", outer)),
        )
        .into())
    }

    fn traits(&self, json: &Map<String, Value>) -> ModelResult<AppliedTraits> {
        let mut traits: AppliedTraits = Default::default();
        for (k, v) in json {
            let id = ShapeID::from_str(k)?;
            let inner = self.value(v)?;
            let _ = traits.insert(id, Some(inner));
        }
        Ok(traits)
    }

    fn members(&self, json: &Map<String, Value>) -> ModelResult<Vec<MemberShape>> {
        let mut members: Vec<MemberShape> = Default::default();
        for (k, v) in json {
            if let Value::Object(obj) = v {
                let target = if let Some(Value::String(target)) = obj.get(ADD_SHAPE_KEY_TARGET) {
                    ShapeID::from_str(target)?
                } else {
                    return Err(ErrorKind::Deserialization(
                        REPRESENTATION_NAME.to_string(),
                        "JsonReader::members/target".to_string(),
                        Some(format!("{:#?}", obj)),
                    )
                    .into());
                };
                let mut member = MemberShape::new(Identifier::from_str(k)?, target);
                if let Some(Value::Object(traits)) = obj.get(ADD_SHAPE_KEY_TRAITS) {
                    member.append_traits(&self.traits(traits)?)?;
                }
                members.push(member);
            } else {
                return Err(ErrorKind::Deserialization(
                    REPRESENTATION_NAME.to_string(),
                    "JsonReader::members".to_string(),
                    Some(format!("{:#?}", v)),
                )
                .into());
            }
        }
        Ok(members)
    }

    fn renames_hash(&self, member: Option<&Value>) -> ModelResult<HashMap<ShapeID, Identifier>> {
        if let Some(member) = member {
            let mut hash: HashMap<ShapeID, Identifier> = Default::default();
            if let Value::Object(ms) = member {
                for (obj_key, obj) in ms {
                    let key = ShapeID::from_str(obj_key)?;
                    let value = Identifier::from_str(&self.string_value(obj)?)?;
                    let _ = hash.insert(key, value);
                }
                Ok(hash)
            } else {
                Err(ErrorKind::Deserialization(
                    REPRESENTATION_NAME.to_string(),
                    "JsonReader::renames_hash".to_string(),
                    Some(format!("{:#?}", member)),
                )
                .into())
            }
        } else {
            Ok(Default::default())
        }
    }

    fn identifiers_hash(
        &self,
        member: Option<&Value>,
    ) -> ModelResult<HashMap<Identifier, ShapeID>> {
        if let Some(member) = member {
            let mut hash: HashMap<Identifier, ShapeID> = Default::default();
            if let Value::Object(ms) = member {
                for (obj_key, obj) in ms {
                    println!("{:?}: {:?}", obj_key, obj);
                    let key = Identifier::from_str(obj_key)?;
                    let value = self.target(Some(obj))?;
                    let _ = hash.insert(key, value);
                }
                Ok(hash)
            } else {
                Err(ErrorKind::Deserialization(
                    REPRESENTATION_NAME.to_string(),
                    "JsonReader::renames_hash".to_string(),
                    Some(format!("{:#?}", member)),
                )
                .into())
            }
        } else {
            Ok(Default::default())
        }
    }

    fn target_list(&self, member: Option<&Value>) -> ModelResult<Vec<ShapeID>> {
        if let Some(member) = member {
            let mut targets: Vec<ShapeID> = Default::default();
            if let Value::Array(ms) = member {
                for obj in ms {
                    targets.push(self.target(Some(obj))?)
                }
                Ok(targets)
            } else {
                Err(ErrorKind::Deserialization(
                    REPRESENTATION_NAME.to_string(),
                    "JsonReader::target_list".to_string(),
                    Some(format!("{:#?}", member)),
                )
                .into())
            }
        } else {
            Ok(Default::default())
        }
    }

    fn optional_target(&self, member: Option<&Value>) -> ModelResult<Option<ShapeID>> {
        if member.is_some() {
            self.target(member).map(Some)
        } else {
            Ok(None)
        }
    }

    fn target(&self, member: Option<&Value>) -> ModelResult<ShapeID> {
        if let Some(Value::Object(ms)) = member {
            if let Some(Value::String(target_id)) = ms.get(ADD_SHAPE_KEY_TARGET) {
                return ShapeID::from_str(target_id);
            }
        }
        Err(ErrorKind::Deserialization(
            REPRESENTATION_NAME.to_string(),
            "JsonReader::target".to_string(),
            Some(format!("{:#?}", member)),
        )
        .into())
    }

    fn member(&self, member: Option<&Value>) -> ModelResult<MemberShape> {
        if let Some(Value::Object(ms)) = member {
            if let Some(Value::String(target_id)) = ms.get(ADD_SHAPE_KEY_TARGET) {
                let mut member_shape = MemberShape::new(
                    Identifier::new_unchecked(MEMBER_MEMBER),
                    ShapeID::from_str(target_id)?,
                );
                if let Some(Value::Object(vs)) = ms.get(ADD_SHAPE_KEY_TRAITS) {
                    member_shape.append_traits(&self.traits(vs)?)?;
                };
                return Ok(member_shape);
            }
        }
        Err(ErrorKind::Deserialization(
            REPRESENTATION_NAME.to_string(),
            "JsonReader::target".to_string(),
            Some(format!("{:#?}", member)),
        )
        .into())
    }

    fn string_value(&self, value: &Value) -> ModelResult<String> {
        if let Value::String(value) = value {
            Ok(value.clone())
        } else {
            Err(ErrorKind::Deserialization(
                REPRESENTATION_NAME.to_string(),
                "JsonReader::string_value".to_string(),
                Some(format!("{:#?}", value)),
            )
            .into())
        }
    }

    fn value(&self, json: &Value) -> ModelResult<NodeValue> {
        match json {
            Value::Null => Ok(NodeValue::None),
            Value::Bool(v) => Ok(NodeValue::from(*v)),
            Value::Number(v) => {
                if v.is_f64() {
                    Ok(NodeValue::from(v.as_f64().unwrap()))
                } else if v.is_i64() {
                    Ok(NodeValue::from(v.as_i64().unwrap()))
                } else if v.is_u64() {
                    Ok(NodeValue::from(v.as_u64().unwrap() as i64))
                } else {
                    Err(ErrorKind::Deserialization(
                        REPRESENTATION_NAME.to_string(),
                        "JsonReader::value".to_string(),
                        Some(format!("{:#?}", json)),
                    )
                    .into())
                }
            }
            Value::String(v) => Ok(NodeValue::from(v.to_string())),
            Value::Array(vs) => {
                let result: ModelResult<Vec<NodeValue>> =
                    vs.iter().map(|v| self.value(v)).collect();
                match result {
                    Err(e) => Err(e),
                    Ok(vs) => Ok(NodeValue::from(vs)),
                }
            }
            Value::Object(vs) => {
                let mut object: ValueMap = Default::default();
                for (k, v) in vs {
                    let _ = object.insert(k.clone(), self.value(v)?);
                }
                Ok(NodeValue::from(object))
            }
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use atelier_core::io::read_model_from_string;

    const JSON: &str = r#"{
    "smithy": "1.0",
    "metadata": {
        "authors": [
            "Simon"
        ]
    },
    "shapes": {
        "smithy.example#MyString": {
            "type": "string",
            "traits": {
                "smithy.api#documentation": "My documentation string",
                "smithy.api#tags": [
                    "a",
                    "b"
                ]
            }
        },
        "smithy.example#MyList": {
            "type": "list",
            "member": {
                "target": "smithy.api#String"
            }
        },
        "smithy.example#MyStructure": {
            "type": "structure",
            "members": {
                "stringMember": {
                    "target": "smithy.api#String",
                    "traits": {
                        "smithy.api#required": {}
                    }
                },
                "numberMember": {
                    "target": "smithy.api#Integer"
                }
            }
        }
    }
}"#;

    #[test]
    fn test_json_parser() {
        let mut reader = JsonReader::default();
        let result = read_model_from_string(&mut reader, JSON);
        if result.is_err() {
            println!("{:?}", result);
        }
        assert!(result.is_ok());
        println!("{:#?}", result.unwrap());
    }
}
