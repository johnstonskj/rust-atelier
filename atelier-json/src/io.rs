use crate::syntax::*;
use atelier_core::error::{AndPanic, ErrorKind, Result, ResultExt};
use atelier_core::io::{ModelReader, ModelWriter};
use atelier_core::model::shapes::{
    ListOrSet, Map as MapShape, Member, Shape, ShapeBody, SimpleType, StructureOrUnion, Trait,
    Valued,
};
use atelier_core::model::values::{Key, NodeValue, Number};
use atelier_core::model::{Annotated, Identifier, Model, Named, Namespace, ShapeID};
use atelier_core::Version;
use serde_json::{from_reader, to_writer, to_writer_pretty, Map, Number as JsonNumber, Value};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Write a [Model](../atelier_core/model/struct.Model.html) in the JSON AST representation.
///
#[allow(missing_debug_implementations)]
pub struct JsonWriter {
    pretty_print: bool,
}

///
/// Read a [Model](../atelier_core/model/struct.Model.html) from the JSON AST representation.
///
#[allow(missing_debug_implementations)]
pub struct JsonReader;

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl<'a> Default for JsonWriter {
    fn default() -> Self {
        Self {
            pretty_print: false,
        }
    }
}

impl<'a> ModelWriter<'a> for JsonWriter {
    const REPRESENTATION: &'static str = "JSON";

    fn write(&mut self, w: &mut impl Write, model: &'a Model) -> Result<()> {
        let mut top: Map<String, Value> = Default::default();

        let _ = top.insert(
            K_SMITHY.to_string(),
            Value::String(model.version().to_string()),
        );

        let _ = top.insert(K_SHAPES.to_string(), self.shapes(model));

        if self.pretty_print {
            to_writer_pretty(w, &Value::Object(top)).chain_err(|| {
                ErrorKind::Serialization(Self::REPRESENTATION.to_string()).to_string()
            })
        } else {
            to_writer(w, &Value::Object(top)).chain_err(|| {
                ErrorKind::Serialization(Self::REPRESENTATION.to_string()).to_string()
            })
        }
    }
}

impl<'a> JsonWriter {
    pub fn new(pretty_print: bool) -> Self {
        Self { pretty_print }
    }

    fn shapes(&self, model: &'a Model) -> Value {
        let mut shape_map: Map<String, Value> = Default::default();
        let namespace = model.namespace().clone();
        for shape in model.shapes() {
            let _ = shape_map.insert(
                shape.id().to_absolute(namespace.clone()).to_string(),
                self.shape(shape),
            );
        }
        if model.has_metadata() {
            let mut meta_map: Map<String, Value> = Default::default();
            for (key, value) in model.metadata() {
                let _ = meta_map.insert(key.to_string(), self.value(value));
            }
            let _ = shape_map.insert(K_METADATA.to_string(), Value::Object(meta_map));
        }
        Value::Object(shape_map)
    }

    fn shape(&self, shape: &'a Shape) -> Value {
        let mut shape_map: Map<String, Value> = Default::default();
        if shape.has_traits() {
            let _ = shape_map.insert(K_TRAITS.to_string(), self.traits(shape.traits()));
        }
        match shape.body() {
            ShapeBody::SimpleType(v) => {
                let _ = shape_map.insert(K_TYPE.to_string(), Value::String(v.to_string()));
            }
            ShapeBody::List(v) => {
                let _ = shape_map.insert(K_TYPE.to_string(), Value::String(V_LIST.to_string()));
                let _ = shape_map.insert(K_MEMBER.to_string(), self.reference(v.member()));
            }
            ShapeBody::Set(v) => {
                let _ = shape_map.insert(K_TYPE.to_string(), Value::String(V_SET.to_string()));
                let _ = shape_map.insert(K_MEMBER.to_string(), self.reference(v.member()));
            }
            ShapeBody::Map(v) => {
                let _ = shape_map.insert(K_TYPE.to_string(), Value::String(V_MAP.to_string()));
                let _ = shape_map.insert(K_KEY.to_string(), self.reference(v.key()));
                let _ = shape_map.insert(K_VALUE.to_string(), self.reference(v.value()));
            }
            ShapeBody::Structure(v) => {
                let _ =
                    shape_map.insert(K_TYPE.to_string(), Value::String(V_STRUCTURE.to_string()));
                if v.has_members() {
                    let _ = shape_map.insert(K_MEMBERS.to_string(), self.members(v.members()));
                }
            }
            ShapeBody::Union(v) => {
                let _ = shape_map.insert(K_TYPE.to_string(), Value::String(V_UNION.to_string()));
                if v.has_members() {
                    let _ = shape_map.insert(K_MEMBERS.to_string(), self.members(v.members()));
                }
            }
            ShapeBody::Service(v) => {
                let _ = shape_map.insert(K_TYPE.to_string(), Value::String(V_SERVICE.to_string()));
                let _ = shape_map.insert(
                    K_VERSION.to_string(),
                    Value::String(v.version().to_string()),
                );
                if v.has_operations() {
                    let _ = shape_map.insert(
                        K_OPERATIONS.to_string(),
                        Value::Array(v.operations().map(|o| self.reference(o)).collect()),
                    );
                }
                if v.has_resources() {
                    let _ = shape_map.insert(
                        K_RESOURCES.to_string(),
                        Value::Array(v.resources().map(|o| self.reference(o)).collect()),
                    );
                }
            }
            ShapeBody::Operation(v) => {
                let _ =
                    shape_map.insert(K_TYPE.to_string(), Value::String(V_OPERATION.to_string()));
                if let Some(v) = v.input() {
                    let _ = shape_map.insert(K_INPUT.to_string(), self.reference(v));
                }
                if let Some(v) = v.output() {
                    let _ = shape_map.insert(K_OUTPUT.to_string(), self.reference(v));
                }
                if v.has_errors() {
                    let _ = shape_map.insert(
                        K_ERRORS.to_string(),
                        Value::Array(v.errors().map(|o| self.reference(o)).collect()),
                    );
                }
            }
            ShapeBody::Resource(v) => {
                let _ = shape_map.insert(K_TYPE.to_string(), Value::String(V_RESOURCE.to_string()));
                if v.has_identifiers() {
                    let mut id_map: Map<String, Value> = Default::default();
                    for (id, ref_id) in v.identifiers() {
                        let _ = id_map.insert(id.to_string(), Value::String(ref_id.to_string()));
                    }
                    let _ = shape_map.insert(K_IDENTIFIERS.to_string(), Value::Object(id_map));
                }
                if let Some(v) = v.create() {
                    let _ = shape_map.insert(K_CREATE.to_string(), self.reference(v));
                }
                if let Some(v) = v.put() {
                    let _ = shape_map.insert(K_PUT.to_string(), self.reference(v));
                }
                if let Some(v) = v.read() {
                    let _ = shape_map.insert(K_READ.to_string(), self.reference(v));
                }
                if let Some(v) = v.update() {
                    let _ = shape_map.insert(K_UPDATE.to_string(), self.reference(v));
                }
                if let Some(v) = v.delete() {
                    let _ = shape_map.insert(K_DELETE.to_string(), self.reference(v));
                }
                if let Some(v) = v.list() {
                    let _ = shape_map.insert(K_LIST.to_string(), self.reference(v));
                }
                if v.has_operations() {
                    let _ = shape_map.insert(
                        K_OPERATIONS.to_string(),
                        Value::Array(v.operations().map(|o| self.reference(o)).collect()),
                    );
                }
                if v.has_collection_operations() {
                    let _ = shape_map.insert(
                        K_OPERATIONS.to_string(),
                        Value::Array(
                            v.collection_operations()
                                .map(|o| self.reference(o))
                                .collect(),
                        ),
                    );
                }
                if v.has_resources() {
                    let _ = shape_map.insert(
                        K_COLLECTION_OPERATIONS.to_string(),
                        Value::Array(v.resources().map(|o| self.reference(o)).collect()),
                    );
                }
            }
            ShapeBody::Apply => {
                let _ = shape_map.insert(K_TYPE.to_string(), Value::String(V_APPLY.to_string()));
            }
        }
        Value::Object(shape_map)
    }

    fn traits(&self, traits: &[Trait]) -> Value {
        let mut trait_map: Map<String, Value> = Default::default();
        for a_trait in traits {
            let _ = trait_map.insert(
                a_trait.id().to_string(),
                match a_trait.value() {
                    None => Value::Object(Default::default()),
                    Some(value) => self.value(value),
                },
            );
        }
        Value::Object(trait_map)
    }

    fn members(&self, members: impl Iterator<Item = &'a Member>) -> Value {
        let mut members_map: Map<String, Value> = Default::default();
        for member in members {
            let mut member_map: Map<String, Value> = Default::default();
            if member.has_traits() {
                let _ = member_map.insert(K_TRAITS.to_string(), self.traits(member.traits()));
            }
            if let Some(NodeValue::ShapeID(id)) = member.value() {
                let _ = member_map.insert(K_TARGET.to_string(), Value::String(id.to_string()));
            } else {
                ErrorKind::InvalidValueVariant("ShapeID".to_string()).panic();
            }
            let _ = members_map.insert(member.id().to_string(), Value::Object(member_map));
        }
        Value::Object(members_map)
    }

    fn value(&self, value: &NodeValue) -> Value {
        match value {
            NodeValue::None => Value::Null,
            NodeValue::Array(v) => Value::Array(v.iter().map(|v| self.value(v)).collect()),
            NodeValue::Object(v) => {
                let mut object: Map<String, Value> = Default::default();
                for (k, v) in v {
                    let _ = object.insert(
                        match k {
                            Key::String(v) => v.clone(),
                            Key::Identifier(v) => v.to_string(),
                        },
                        self.value(v),
                    );
                }
                Value::Object(object)
            }
            NodeValue::Number(v) => match v {
                Number::Integer(v) => Value::Number((*v).into()),
                Number::Float(v) => Value::Number(JsonNumber::from_f64(*v).unwrap()),
            },
            NodeValue::Boolean(v) => Value::Bool(*v),
            NodeValue::ShapeID(v) => self.reference(v),
            NodeValue::TextBlock(v) => Value::String(v.clone()),
            NodeValue::String(v) => Value::String(v.clone()),
        }
    }

    fn reference(&self, id: &'a ShapeID) -> Value {
        let mut shape_map: Map<String, Value> = Default::default();
        let _ = shape_map.insert(K_TARGET.to_string(), Value::String(id.to_string()));
        Value::Object(shape_map)
    }
}

// ------------------------------------------------------------------------------------------------

impl<'a> Default for JsonReader {
    fn default() -> Self {
        Self {}
    }
}

impl ModelReader for JsonReader {
    const REPRESENTATION: &'static str = "JSON";

    fn read(&mut self, r: &mut impl Read) -> Result<Model> {
        let json: Value = from_reader(r).chain_err(|| {
            ErrorKind::Deserialization(
                Self::REPRESENTATION.to_string(),
                "ModelReader::read".to_string(),
                None,
            )
            .to_string()
        })?;
        self.model(json)
    }
}

impl JsonReader {
    fn model(&self, json: Value) -> Result<Model> {
        if let Value::Object(vs) = json {
            let version = self.version(vs.get(K_SMITHY))?;

            let metadata = self.metadata(vs.get(K_METADATA))?;

            let shapes = self.shapes(vs.get(K_SHAPES))?;
            if !shapes.is_empty() {
                let namespaces: Vec<&Namespace> = shapes.iter().map(|(ns, _)| ns).collect();
                let all_unique = namespaces
                    .get(0)
                    .map(|first| namespaces.iter().all(|x| x == first))
                    .unwrap_or(true);
                if all_unique {
                    let namespace = (*namespaces.first().unwrap()).clone();
                    let mut model = Model::new(namespace, Some(version));

                    for shape in shapes.into_iter().map(|(_, s)| s) {
                        model.add_shape(shape);
                    }

                    for (key, value) in metadata {
                        model.add_metadata(key, value);
                    }
                    return Ok(model);
                }
            }
        }
        Err(ErrorKind::Deserialization(
            Self::REPRESENTATION.to_string(),
            "JsonReader::model".to_string(),
            None,
        )
        .into())
    }

    fn version(&self, json: Option<&Value>) -> Result<Version> {
        if let Some(Value::String(version)) = json {
            Ok(Version::from_str(version)?)
        } else {
            Err(ErrorKind::Deserialization(
                Self::REPRESENTATION.to_string(),
                "JsonReader::version".to_string(),
                Some(format!("{:#?}", json)),
            )
            .into())
        }
    }

    fn metadata(&self, json: Option<&Value>) -> Result<HashMap<Key, NodeValue>> {
        let mut metadata: HashMap<Key, NodeValue> = Default::default();
        if let Some(Value::Object(vs)) = json {
            for (k, v) in vs {
                let _ = metadata.insert(Key::String(k.clone()), self.value(v)?);
            }
        }
        Ok(metadata)
    }

    fn shapes(&self, json: Option<&Value>) -> Result<Vec<(Namespace, Shape)>> {
        let mut shapes: Vec<(Namespace, Shape)> = Default::default();
        if let Some(Value::Object(vs)) = json {
            for (k, v) in vs {
                let id = ShapeID::from_str(k)?;
                let inner = self.shape(v)?;
                let mut shape = Shape::local(id.shape_name().clone(), inner);

                if let Some(Value::Object(vs)) = v.get(K_TRAITS) {
                    shape.append_traits(self.traits(vs)?.as_ref())
                };

                shapes.push((id.namespace().as_ref().unwrap().clone(), shape))
            }
        }
        Ok(shapes)
    }

    fn shape(&self, outer: &Value) -> Result<ShapeBody> {
        if let Some(Value::String(s)) = outer.get(K_TYPE) {
            let s = s.as_str();
            return if let Ok(st) = SimpleType::from_str(s) {
                Ok(ShapeBody::SimpleType(st))
            } else if s == V_APPLY {
                Ok(ShapeBody::Apply)
            } else if s == V_LIST {
                Ok(ShapeBody::List(ListOrSet::new(
                    self.target(outer.get(K_MEMBER))?,
                )))
            } else if s == V_SET {
                Ok(ShapeBody::Set(ListOrSet::new(
                    self.target(outer.get(K_MEMBER))?,
                )))
            } else if s == V_MAP {
                Ok(ShapeBody::Map(MapShape::new(
                    self.target(outer.get(K_KEY))?,
                    self.target(outer.get(K_VALUE))?,
                )))
            } else if s == V_STRUCTURE {
                if let Some(Value::Object(vs)) = outer.get(K_MEMBERS) {
                    let mut members: Vec<Member> = Default::default();
                    for (k, v) in vs {
                        members.push(Member::with_value(
                            Identifier::from_str(k)?,
                            self.value(&v)?,
                        ));
                    }
                    Ok(ShapeBody::Structure(StructureOrUnion::with_members(
                        members.as_slice(),
                    )))
                } else {
                    return Err(ErrorKind::Deserialization(
                        Self::REPRESENTATION.to_string(),
                        "JsonReader::shape/structure".to_string(),
                        Some(format!("{:#?}", outer)),
                    )
                    .into());
                }
            } else {
                return Err(ErrorKind::Deserialization(
                    Self::REPRESENTATION.to_string(),
                    "JsonReader::shape/type".to_string(),
                    Some(format!("{:#?}", outer)),
                )
                .into());
            };
        }
        Err(ErrorKind::Deserialization(
            Self::REPRESENTATION.to_string(),
            "JsonReader::shape".to_string(),
            Some(format!("{:#?}", outer)),
        )
        .into())
    }

    fn traits(&self, json: &Map<String, Value>) -> Result<Vec<Trait>> {
        let mut traits: Vec<Trait> = Default::default();
        for (k, v) in json {
            let id = ShapeID::from_str(k)?;
            let inner = self.value(v)?;
            traits.push(Trait::with_value(id, inner))
        }
        Ok(traits)
    }

    fn target(&self, member: Option<&Value>) -> Result<ShapeID> {
        if let Some(Value::Object(ms)) = member {
            if let Some(Value::String(member_id)) = ms.get(K_TARGET) {
                return Ok(ShapeID::from_str(member_id)?);
            }
        }
        Err(ErrorKind::Deserialization(
            Self::REPRESENTATION.to_string(),
            "JsonReader::target".to_string(),
            Some(format!("{:#?}", member)),
        )
        .into())
    }

    fn value(&self, json: &Value) -> Result<NodeValue> {
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
                        Self::REPRESENTATION.to_string(),
                        "JsonReader::value".to_string(),
                        Some(format!("{:#?}", json)),
                    )
                    .into())
                }
            }
            Value::String(v) => Ok(NodeValue::from(v.to_string())),
            Value::Array(vs) => {
                let result: Result<Vec<NodeValue>> = vs.iter().map(|v| self.value(v)).collect();
                match result {
                    Err(e) => Err(e),
                    Ok(vs) => Ok(NodeValue::from(vs)),
                }
            }
            Value::Object(vs) => {
                let mut object: HashMap<Key, NodeValue> = Default::default();
                for (k, v) in vs {
                    let _ =
                        object.insert(Key::Identifier(Identifier::from_str(k)?), self.value(v)?);
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
