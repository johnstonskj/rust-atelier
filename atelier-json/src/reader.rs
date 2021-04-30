use crate::syntax::*;
use crate::FILE_EXTENSION;
use atelier_core::error::{ErrorKind, Result as ModelResult, ResultExt};
use atelier_core::io::ModelReader;
use atelier_core::model::shapes::{
    AppliedTraits, HasTraits, ListOrSet, Map as MapShape, MemberShape, ShapeKind, Simple,
    StructureOrUnion, TopLevelShape,
};
use atelier_core::model::values::{Value as NodeValue, ValueMap};
use atelier_core::model::{Identifier, Model, NamespaceID, ShapeID};
use atelier_core::Version;
use serde_json::{from_reader, Map, Value};
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
                FILE_EXTENSION.to_string(),
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
            let version = self.version(vs.get(K_SMITHY))?;

            let metadata = self.metadata(vs.get(K_METADATA))?;

            let shapes = self.shapes(vs.get(K_SHAPES))?;
            if !shapes.is_empty() {
                let mut model = Model::new(version);

                for shape in shapes.into_iter().map(|(_, s)| s) {
                    let _ = model.add_shape(shape);
                }

                for (key, value) in metadata {
                    let _ = model.add_metadata(key, value);
                }
                return Ok(model);
            }
        }
        Err(ErrorKind::Deserialization(
            FILE_EXTENSION.to_string(),
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
                FILE_EXTENSION.to_string(),
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
                let inner = self.shape(&id, v)?;
                let mut shape = TopLevelShape::new(id.clone(), inner);

                if let Some(Value::Object(vs)) = v.get(K_TRAITS) {
                    shape.append_traits(&self.traits(vs)?)?;
                };

                shapes.push((id.namespace().clone(), shape))
            }
        }
        Ok(shapes)
    }

    fn shape(&self, id: &ShapeID, outer: &Value) -> ModelResult<ShapeKind> {
        if let Some(Value::String(s)) = outer.get(K_TYPE) {
            let s = s.as_str();
            return if let Ok(st) = Simple::from_str(s) {
                Ok(ShapeKind::Simple(st))
            } else if s == V_APPLY {
                Ok(ShapeKind::Unresolved)
            } else if s == V_LIST {
                Ok(ShapeKind::List(ListOrSet::new(
                    &id,
                    self.target(outer.get(K_MEMBER))?,
                )))
            } else if s == V_SET {
                Ok(ShapeKind::Set(ListOrSet::new(
                    &id,
                    self.target(outer.get(K_MEMBER))?,
                )))
            } else if s == V_MAP {
                Ok(ShapeKind::Map(MapShape::new(
                    &id,
                    self.target(outer.get(K_KEY))?,
                    self.target(outer.get(K_VALUE))?,
                )))
            } else if s == V_STRUCTURE {
                let members = if let Some(Value::Object(vs)) = outer.get(K_MEMBERS) {
                    self.members(&id, vs)?
                } else {
                    Default::default()
                };
                Ok(ShapeKind::Structure(StructureOrUnion::with_members(
                    members.as_slice(),
                )))
            } else if s == V_UNION {
                let members = if let Some(Value::Object(vs)) = outer.get(K_MEMBERS) {
                    self.members(&id, vs)?
                } else {
                    Default::default()
                };
                Ok(ShapeKind::Union(StructureOrUnion::with_members(
                    members.as_slice(),
                )))
            } else {
                return Err(ErrorKind::Deserialization(
                    FILE_EXTENSION.to_string(),
                    "JsonReader::shape/type".to_string(),
                    Some(format!("{:#?}", outer)),
                )
                .into());
            };
        }
        Err(ErrorKind::Deserialization(
            FILE_EXTENSION.to_string(),
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

    fn members(
        &self,
        parent_id: &ShapeID,
        json: &Map<String, Value>,
    ) -> ModelResult<Vec<MemberShape>> {
        let mut members: Vec<MemberShape> = Default::default();
        for (k, v) in json {
            if let Value::Object(obj) = v {
                let target = if let Some(Value::String(target)) = obj.get(K_TARGET) {
                    ShapeID::from_str(target)?
                } else {
                    return Err(ErrorKind::Deserialization(
                        FILE_EXTENSION.to_string(),
                        "JsonReader::members/target".to_string(),
                        Some(format!("{:#?}", obj)),
                    )
                    .into());
                };
                let mut member =
                    MemberShape::new(parent_id.make_member(Identifier::from_str(k)?), target);
                if let Some(Value::Object(traits)) = obj.get(K_TRAITS) {
                    member.append_traits(&self.traits(traits)?)?;
                }
                members.push(member);
            } else {
                return Err(ErrorKind::Deserialization(
                    FILE_EXTENSION.to_string(),
                    "JsonReader::members".to_string(),
                    Some(format!("{:#?}", v)),
                )
                .into());
            }
        }
        Ok(members)
    }

    fn target(&self, member: Option<&Value>) -> ModelResult<ShapeID> {
        if let Some(Value::Object(ms)) = member {
            if let Some(Value::String(member_id)) = ms.get(K_TARGET) {
                return ShapeID::from_str(member_id);
            }
        }
        Err(ErrorKind::Deserialization(
            FILE_EXTENSION.to_string(),
            "JsonReader::target".to_string(),
            Some(format!("{:#?}", member)),
        )
        .into())
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
                        FILE_EXTENSION.to_string(),
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
