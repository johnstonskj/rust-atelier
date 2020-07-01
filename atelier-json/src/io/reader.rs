use crate::io::syntax::*;
use atelier_core::error::{ErrorKind, Result, ResultExt};
use atelier_core::io::ModelReader;
use atelier_core::model::shapes::{
    ListOrSet, Map as MapShape, Member, Shape, ShapeBody, SimpleType, StructureOrUnion, Trait,
};
use atelier_core::model::values::{Key, NodeValue};
use atelier_core::model::{Annotated, Identifier, Model, Namespace, ShapeID};
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
    fn representation(&self) -> &'static str {
        "JSON"
    }

    fn read(&mut self, r: &mut impl Read) -> Result<Model> {
        let json: Value = from_reader(r).chain_err(|| {
            ErrorKind::Deserialization(
                self.representation().to_string(),
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
            self.representation().to_string(),
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
                self.representation().to_string(),
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
                let mut shape = Shape::new(id.clone(), inner);

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
                let members = if let Some(Value::Object(vs)) = outer.get(K_MEMBERS) {
                    self.members(vs)?
                } else {
                    Default::default()
                };
                Ok(ShapeBody::Structure(StructureOrUnion::with_members(
                    members.as_slice(),
                )))
            } else if s == V_UNION {
                let members = if let Some(Value::Object(vs)) = outer.get(K_MEMBERS) {
                    self.members(vs)?
                } else {
                    Default::default()
                };
                Ok(ShapeBody::Union(StructureOrUnion::with_members(
                    members.as_slice(),
                )))
            } else {
                return Err(ErrorKind::Deserialization(
                    self.representation().to_string(),
                    "JsonReader::shape/type".to_string(),
                    Some(format!("{:#?}", outer)),
                )
                .into());
            };
        }
        Err(ErrorKind::Deserialization(
            self.representation().to_string(),
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

    fn members(&self, json: &Map<String, Value>) -> Result<Vec<Member>> {
        let mut members: Vec<Member> = Default::default();
        for (k, v) in json {
            if let Value::Object(obj) = v {
                let target = if let Some(Value::String(target)) = obj.get(K_TARGET) {
                    ShapeID::from_str(target)?
                } else {
                    return Err(ErrorKind::Deserialization(
                        self.representation().to_string(),
                        "JsonReader::members/target".to_string(),
                        Some(format!("{:#?}", obj)),
                    )
                    .into());
                };
                let mut member =
                    Member::with_value(Identifier::from_str(k)?, NodeValue::from(target));
                if let Some(Value::Object(traits)) = obj.get(K_TRAITS) {
                    member.append_traits(self.traits(traits)?.as_slice());
                }
                members.push(member);
            } else {
                return Err(ErrorKind::Deserialization(
                    self.representation().to_string(),
                    "JsonReader::members".to_string(),
                    Some(format!("{:#?}", v)),
                )
                .into());
            }
        }
        Ok(members)
    }

    fn target(&self, member: Option<&Value>) -> Result<ShapeID> {
        if let Some(Value::Object(ms)) = member {
            if let Some(Value::String(member_id)) = ms.get(K_TARGET) {
                return Ok(ShapeID::from_str(member_id)?);
            }
        }
        Err(ErrorKind::Deserialization(
            self.representation().to_string(),
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
                        self.representation().to_string(),
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
                    let _ = object.insert(Key::from(k.to_string()), self.value(v)?);
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
