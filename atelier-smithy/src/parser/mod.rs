use atelier_core::error::{Error, ErrorKind, Result, ResultExt};
use atelier_core::model::shapes::{
    HasMembers, ListOrSet, Map, Member, Operation, Resource, Service, Shape, ShapeBody, SimpleType,
    StructureOrUnion, Trait, Valued,
};
use atelier_core::model::values::{Key, NodeValue, Number};
use atelier_core::model::{Annotated, Identifier, Model, Named, Namespace, ShapeID};
use atelier_core::Version;
use pest::error::Error as PestError;
use pest::iterators::Pair;
use pest::Parser;
use std::collections::HashMap;
use std::io::Write;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Parser)]
#[grammar = "smithy.pest"]
struct SmithyParser;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub(crate) fn parse(input: &str) -> Result<Model> {
    let mut parsed = SmithyParser::parse(Rule::idl, input).map_err(from_pest_error)?;
    let top_node = parsed.next().unwrap();
    parse_idl(top_node)
}

#[cfg(feature = "debug")]
#[allow(dead_code)]
pub(crate) fn parse_and_debug(w: &mut impl Write, input: &str) -> Result<()> {
    let parsed = SmithyParser::parse(Rule::idl, input).map_err(from_pest_error)?;
    writeln!(w, "{}", pest_ascii_tree::into_ascii_tree(parsed).unwrap())?;
    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Macros
// ------------------------------------------------------------------------------------------------

macro_rules! unexpected {
    () => {
        unreachable!("unexpected rule")
    };
    ($rule:expr) => {
        unreachable!("unexpected rule {:#?}", $rule)
    };
    ($fn_name:expr, $rule:expr) => {
        unreachable!("unexpected rule {:#?} ({:#?})", $fn_name, $rule)
    };
}
macro_rules! match_eoi_rule {
    ($pair:expr, $( $rule:ident => $match_expr:expr ),+) => {
        match $pair.as_rule() {
            $( Rule::$rule => $match_expr ),+
            , Rule::EOI => {},
            _ => unexpected!($pair),
        }
    };
}

macro_rules! match_rule {
    ($pair:expr, $( $rule:ident => $match_expr:expr ),+) => {
        match $pair.as_rule() {
            $( Rule::$rule => $match_expr ),+
            _ => unexpected!($pair),
        }
    };
}
//
// macro_rules! match_eoi_loop {
//     ($pair:expr, $( $rule:ident => $match_expr:expr ),+) => {
//         for inner in $pair.into_inner() {
//             match_eoi_rule! ( inner, $( $rule => $match_expr ),+ )
//         }
//     };
// }
//
// macro_rules! match_loop {
//     ($pair:expr, $( $rule:ident => $match_expr:expr ),+) => {
//         for inner in $pair.into_inner() {
//             match_rule! ( inner, $( $rule => $match_expr ),+ )
//         }
//     };
// }

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn parse_idl(input_pair: Pair<'_, Rule>) -> Result<Model> {
    match input_pair.as_rule() {
        Rule::idl => {
            let mut control_data: HashMap<Key, NodeValue> = Default::default();
            let mut meta_data: HashMap<Key, NodeValue> = Default::default();
            let mut shape_section: Option<(Namespace, Vec<ShapeID>, Vec<Shape>)> = None;
            for inner in input_pair.into_inner() {
                match_eoi_rule!(inner,
                    control_section => {
                        control_data = parse_control_section(inner)?;
                    },
                    metadata_section => {
                        meta_data = parse_metadata_section(inner)?;
                    },
                    shape_section => {
                        shape_section = Some(parse_shape_section(inner)?);
                    }
                );
            }
            let version = if let Some(NodeValue::String(version)) =
                control_data.get(&Key::String("version".to_string()))
            {
                Version::from_str(version)?
            } else {
                Version::default()
            };
            let shape_section = shape_section.unwrap();
            let mut model = Model::new(shape_section.0, Some(version));
            for (k, v) in control_data {
                model.add_control_data(k, v);
            }
            model.append_references(shape_section.1.as_slice());
            model.append_shapes(shape_section.2.as_slice());
            for (k, v) in meta_data {
                model.add_metadata(k, v);
            }
            Ok(model)
        }
        _ => unexpected!("parse_idl", input_pair),
    }
}

fn parse_control_section(input_pair: Pair<'_, Rule>) -> Result<HashMap<Key, NodeValue>> {
    let mut map: HashMap<Key, NodeValue> = Default::default();
    for inner in input_pair.into_inner() {
        match_rule!(inner,
            control_statement => {
                let (key, value) = parse_node_object_kvp(inner)?;
                let _ = map.insert(key, value);
            }
        );
    }
    Ok(map)
}

fn parse_metadata_section(input_pair: Pair<'_, Rule>) -> Result<HashMap<Key, NodeValue>> {
    let mut map: HashMap<Key, NodeValue> = Default::default();
    for inner in input_pair.into_inner() {
        match_rule!(inner,
            metadata_statement => {
                let (key, value) = parse_node_object_kvp(inner)?;
                let _ = map.insert(key, value);
            }
        );
    }
    Ok(map)
}

fn parse_shape_section(
    input_pair: Pair<'_, Rule>,
) -> Result<(Namespace, Vec<ShapeID>, Vec<Shape>)> {
    let mut namespace: Option<Namespace> = None;
    let mut uses: Vec<ShapeID> = Default::default();
    let mut shapes: Vec<Shape> = Default::default();
    for inner in input_pair.into_inner() {
        match_rule!(inner,
            namespace_statement => {
                namespace = Some(parse_namespace_statement(inner)?);
            },
            use_section => {
                uses = parse_use_section(inner)?;
            },
            shape_statements => {
                shapes = parse_shape_statements(inner)?;
            }
        );
    }
    Ok((namespace.unwrap(), uses, shapes))
}

fn parse_namespace_statement(input_pair: Pair<'_, Rule>) -> Result<Namespace> {
    let namespace: Pair<'_, Rule> = input_pair.into_inner().next().unwrap();
    if let Rule::namespace = namespace.as_rule() {
        Namespace::from_str(namespace.as_str())
    } else {
        Err(ErrorKind::Deserialization(
            "Smithy".to_string(),
            "parser::parse_namespace".to_string(),
            Some(namespace.to_string()),
        )
        .into())
    }
}

fn parse_use_section(input_pair: Pair<'_, Rule>) -> Result<Vec<ShapeID>> {
    let mut uses: Vec<ShapeID> = Default::default();
    for use_statement in input_pair.into_inner() {
        match_rule!(use_statement,
           use_statement => {
               let absolute_root_shape_id: Pair<'_, Rule> =
                   use_statement.into_inner().next().unwrap();
               if let Rule::absolute_root_shape_id = absolute_root_shape_id.as_rule() {
                   uses.push(ShapeID::from_str(absolute_root_shape_id.as_str())?);
               } else {
                   unreachable!("parse_uses > unreachable! {:#?}", absolute_root_shape_id)
               }
           }
        );
    }
    Ok(uses)
}

fn parse_shape_statements(input_pair: Pair<'_, Rule>) -> Result<Vec<Shape>> {
    let mut shapes: Vec<Shape> = Default::default();
    for shape_statement in input_pair.into_inner() {
        match_eoi_rule!(shape_statement,
            shape_statement => {
                shapes.push(parse_shape_statement(shape_statement)?);
            }
        );
    }
    Ok(shapes)
}

fn parse_shape_statement(input_pair: Pair<'_, Rule>) -> Result<Shape> {
    let mut documentation: Vec<String> = Default::default();
    let mut traits: Vec<Trait> = Default::default();
    let mut inner: Option<(Identifier, ShapeBody)> = None;
    let mut applies: Option<(ShapeID, Trait)> = None;
    for shape_statement in input_pair.into_inner() {
        match_eoi_rule!(shape_statement,
            documentation_text => {
                documentation.push(parse_documentation_text(shape_statement)?);
            },
            trait_statements => {
                traits = parse_trait_statements(shape_statement)?;
            },
            simple_shape_statement => {
                inner = Some(parse_simple_shape_statement(shape_statement)?);
            },
            list_statement => {
                inner = Some(parse_list_statement(shape_statement)?);
            },
            set_statement => {
                inner = Some(parse_set_statement(shape_statement)?);
            },
            map_statement => {
                inner = Some(parse_map_statement(shape_statement)?);
            },
            structure_statement => {
                inner = Some(parse_structure_statement(shape_statement)?);
            },
            union_statement => {
                inner = Some(parse_union_statement(shape_statement)?);
            },
            service_statement => {
                inner = Some(parse_service_statement(shape_statement)?);
            },
            operation_statement => {
                inner = Some(parse_operation_statement(shape_statement)?);
            },
            resource_statement => {
                inner = Some(parse_resource_statement(shape_statement)?);
            },
            apply_statement => {
                applies = Some(parse_apply_statement(shape_statement)?);
            }
        );
    }
    if let Some((id, inner)) = inner {
        let mut shape = Shape::local(id, inner);
        if !documentation.is_empty() {
            shape.documentation(&documentation.join("\n"));
        }
        for a_trait in traits {
            shape.add_trait(a_trait);
        }
        Ok(shape)
    } else if let Some((id, a_trait)) = applies {
        let inner = ShapeBody::Apply;
        let mut shape = Shape::new(id, inner);
        shape.add_trait(a_trait);
        Ok(shape)
    } else {
        Err(ErrorKind::Deserialization(
            "Smithy".to_string(),
            "parser::parse_shape_statement".to_string(),
            None,
        )
        .into())
    }
}

fn parse_documentation_text(input_pair: Pair<'_, Rule>) -> Result<String> {
    if let Rule::documentation_text = input_pair.as_rule() {
        Ok(input_pair.as_str().to_string())
    } else {
        Err(ErrorKind::Deserialization(
            "Smithy".to_string(),
            "parser::parse_shape_documentation_comments".to_string(),
            Some(input_pair.to_string()),
        )
        .into())
    }
}

fn parse_trait_statements(input_pair: Pair<'_, Rule>) -> Result<Vec<Trait>> {
    let mut traits: Vec<Trait> = Default::default();
    for a_trait in input_pair.into_inner() {
        match_rule!(a_trait,
            a_trait => {
                traits.push(parse_a_trait(a_trait)?);
            }
        );
    }
    Ok(traits)
}

fn parse_a_trait(input_pair: Pair<'_, Rule>) -> Result<Trait> {
    let mut id: Option<ShapeID> = None;
    let mut node_value: Option<NodeValue> = None;
    for inner in input_pair.into_inner() {
        match_rule!(inner,
            shape_id => {
                id = Some(ShapeID::from_str(inner.as_str())?);
            },
            node_value => {
                node_value = Some(parse_node_value(inner)?);
            },
            trait_structure_kvp => {}
        );
    }
    Ok(match (id, node_value) {
        (Some(id), None) => Trait::new(id),
        (Some(id), Some(node_value)) => Trait::with_value(id, node_value),
        _ => unexpected!("a_trait"),
    })
}

fn parse_simple_shape_statement(input_pair: Pair<'_, Rule>) -> Result<(Identifier, ShapeBody)> {
    let mut id: Option<Identifier> = None;
    let mut simple_type: Option<SimpleType> = None;
    for inner in input_pair.into_inner() {
        match_rule!(inner,
            identifier => {
                id = Some(Identifier::from_str(inner.as_str())?);
            },
            type_blob => {simple_type = Some(SimpleType::Blob)},
            type_boolean => {simple_type = Some(SimpleType::Boolean)},
            type_document => {simple_type = Some(SimpleType::Document)},
            type_string => {simple_type = Some(SimpleType::String)},
            type_byte => {simple_type = Some(SimpleType::Byte)},
            type_short => {simple_type = Some(SimpleType::Short)},
            type_integer => {simple_type = Some(SimpleType::Integer)},
            type_long => {simple_type = Some(SimpleType::Long)},
            type_float => {simple_type = Some(SimpleType::Float)},
            type_double => {simple_type = Some(SimpleType::Double)},
            type_big_integer => {simple_type = Some(SimpleType::BigInteger)},
            type_big_decimal => {simple_type = Some(SimpleType::BigDecimal)},
            type_timestamp => {simple_type = Some(SimpleType::Timestamp)}
        );
    }
    Ok(match (id, simple_type) {
        (Some(id), Some(simple_type)) => (id, ShapeBody::SimpleType(simple_type)),
        _ => unexpected!("simple_shape_statement"),
    })
}

fn parse_list_statement(input_pair: Pair<'_, Rule>) -> Result<(Identifier, ShapeBody)> {
    let (id, members) = parse_membered_statement(input_pair)?;
    for member in members {
        if member.id() == &Identifier::from_str("member").unwrap() {
            return Ok((
                id,
                ShapeBody::List(ListOrSet::new(
                    member
                        .value()
                        .as_ref()
                        .unwrap()
                        .as_shape_id()
                        .unwrap()
                        .clone(),
                )),
            ));
        }
    }
    unexpected!("list_statement")
}

fn parse_set_statement(input_pair: Pair<'_, Rule>) -> Result<(Identifier, ShapeBody)> {
    let (id, members) = parse_membered_statement(input_pair)?;
    for member in members {
        if member.id() == &Identifier::from_str("member").unwrap() {
            return Ok((
                id,
                ShapeBody::Set(ListOrSet::new(
                    member
                        .value()
                        .as_ref()
                        .unwrap()
                        .as_shape_id()
                        .unwrap()
                        .clone(),
                )),
            ));
        }
    }
    unexpected!("set_statement")
}

fn parse_map_statement(statement: Pair<'_, Rule>) -> Result<(Identifier, ShapeBody)> {
    let (id, members) = parse_membered_statement(statement)?;
    let mut key: Option<ShapeID> = None;
    let mut value: Option<ShapeID> = None;
    for member in members {
        if member.id() == &Identifier::from_str("key").unwrap() {
            key = Some(
                member
                    .value()
                    .as_ref()
                    .unwrap()
                    .as_shape_id()
                    .unwrap()
                    .clone(),
            )
        } else if member.id() == &Identifier::from_str("value").unwrap() {
            value = Some(
                member
                    .value()
                    .as_ref()
                    .unwrap()
                    .as_shape_id()
                    .unwrap()
                    .clone(),
            )
        } else {
            unexpected!("map_statement", member)
        }
    }
    match (key, value) {
        (Some(k), Some(v)) => Ok((id, ShapeBody::Map(Map::new(k, v)))),
        _ => unexpected!("map_statement"),
    }
}

fn parse_structure_statement(statement: Pair<'_, Rule>) -> Result<(Identifier, ShapeBody)> {
    let (id, members) = parse_membered_statement(statement)?;
    Ok((
        id,
        ShapeBody::Structure(StructureOrUnion::with_members(members.as_slice())),
    ))
}

fn parse_union_statement(statement: Pair<'_, Rule>) -> Result<(Identifier, ShapeBody)> {
    let (id, members) = parse_membered_statement(statement)?;
    Ok((
        id,
        ShapeBody::Union(StructureOrUnion::with_members(members.as_slice())),
    ))
}

fn parse_membered_statement(statement: Pair<'_, Rule>) -> Result<(Identifier, Vec<Member>)> {
    let mut id: Option<Identifier> = None;
    let mut members: Vec<Member> = Default::default();
    for inner in statement.into_inner() {
        match_rule!(inner,
            identifier => {
                id = Some(Identifier::from_str(inner.as_str())?);
            },
            empty_shape_members => {},
            populated_shape_members => {
                members = parse_populated_shape_members(inner)?;
            }
        );
    }
    if let Some(id) = id {
        Ok((id, members))
    } else {
        unexpected!("membered_statement")
    }
}

fn parse_populated_shape_members(populated_shape_members: Pair<'_, Rule>) -> Result<Vec<Member>> {
    let mut members = Vec::default();
    for inner in populated_shape_members.into_inner() {
        match_rule!(inner,
            shape_member_kvp => {
                members.push(parse_shape_member_kvp(inner)?);
            }
        );
    }
    Ok(members)
}

fn parse_shape_member_kvp(shape_member_kvp: Pair<'_, Rule>) -> Result<Member> {
    let mut documentation: Vec<String> = Default::default();
    let mut traits: Vec<Trait> = Default::default();
    let mut id: Option<Identifier> = None;
    let mut shape_id: Option<ShapeID> = None;
    for inner in shape_member_kvp.into_inner() {
        match_rule!(inner,
            documentation_text => {
                documentation.push(parse_documentation_text(inner)?);
            },
            trait_statements => {
                traits = parse_trait_statements(inner)?;
            },
            identifier => {
                id = Some(Identifier::from_str(inner.as_str())?);
            },
            shape_id => {
                shape_id = Some(ShapeID::from_str(inner.as_str())?);
            }
        );
    }
    match (id, shape_id) {
        (Some(id), Some(shape_id)) => {
            let mut member = Member::new(id);
            member.set_value(NodeValue::ShapeID(shape_id));
            for doc in documentation {
                member.documentation(&doc);
            }
            for a_trait in traits {
                member.add_trait(a_trait);
            }
            Ok(member)
        }
        _ => unexpected!("shape_member_kvp"),
    }
}

fn parse_service_statement(statement: Pair<'_, Rule>) -> Result<(Identifier, ShapeBody)> {
    let (id, object) = parse_id_and_object(statement)?;
    if let NodeValue::Object(object) = object {
        let mut service = Service::default();
        for (key, value) in object {
            match &key {
                Key::Identifier(id) => {
                    if id.to_string() == "version"
                        || id.to_string() == "operations"
                        || id.to_string() == "resources"
                    {
                        service.set_member(Member::with_value(id.clone(), value))?;
                    } else {
                        return Err(ErrorKind::UnknownMember(key.to_string()).into());
                    }
                }
                _ => unexpected!("service_statement", key),
            }
        }
        Ok((id, ShapeBody::Service(service)))
    } else {
        unexpected!("service_statement", object)
    }
}

fn parse_operation_statement(statement: Pair<'_, Rule>) -> Result<(Identifier, ShapeBody)> {
    let (id, object) = parse_id_and_object(statement)?;
    if let NodeValue::Object(object) = object {
        let mut operation = Operation::default();
        for (key, value) in object {
            match &key {
                Key::Identifier(id) => {
                    if id.to_string() == "input"
                        || id.to_string() == "output"
                        || id.to_string() == "errors"
                    {
                        operation.set_member(Member::with_value(id.clone(), value))?;
                    } else {
                        return Err(ErrorKind::UnknownMember(key.to_string()).into());
                    }
                }
                _ => unexpected!("operation_statement", key),
            }
        }
        Ok((id, ShapeBody::Operation(operation)))
    } else {
        unexpected!("operation_statement", object)
    }
}

fn parse_resource_statement(statement: Pair<'_, Rule>) -> Result<(Identifier, ShapeBody)> {
    let (id, object) = parse_id_and_object(statement)?;
    if let NodeValue::Object(object) = object {
        let mut resource = Resource::default();
        for (key, value) in object {
            match &key {
                Key::Identifier(id) => {
                    if id.to_string() == "identifiers"
                        || id.to_string() == "create"
                        || id.to_string() == "put"
                        || id.to_string() == "read"
                        || id.to_string() == "update"
                        || id.to_string() == "delete"
                        || id.to_string() == "list"
                        || id.to_string() == "operations"
                        || id.to_string() == "collection_operations"
                        || id.to_string() == "resources"
                    {
                        resource.set_member(Member::with_value(id.clone(), value))?;
                    } else {
                        return Err(ErrorKind::UnknownMember(key.to_string()).into());
                    }
                }
                _ => unexpected!("resource_statement", key),
            }
        }
        Ok((id, ShapeBody::Resource(resource)))
    } else {
        unexpected!("resource_statement", object)
    }
}

fn parse_id_and_object(statement: Pair<'_, Rule>) -> Result<(Identifier, NodeValue)> {
    let mut id: Option<Identifier> = None;
    let mut node_value: Option<NodeValue> = None;
    for inner in statement.into_inner() {
        match_rule!(inner,
            identifier => {
                id = Some(Identifier::from_str(inner.as_str())?);
            },
            empty_node_object => {node_value = Some(empty_node_object())},
            populated_node_object => {node_value = Some(parse_populated_node_object(inner)?)}
        );
    }
    Ok(match (id, node_value) {
        (Some(id), Some(node_value)) => (id, node_value),
        _ => unreachable!("id_and_object"),
    })
}

fn parse_apply_statement(statement: Pair<'_, Rule>) -> Result<(ShapeID, Trait)> {
    let mut id: Option<ShapeID> = None;
    let mut a_trait: Option<Trait> = None;
    for inner in statement.into_inner() {
        match_rule!(inner,
            identifier => {
                id = Some(ShapeID::from_str(inner.as_str())?);
            },
            a_trait => {a_trait = Some(parse_a_trait(inner)?)}
        );
    }
    Ok(match (id, a_trait) {
        (Some(id), Some(a_trait)) => (id, a_trait),
        _ => unreachable!("apply_statement"),
    })
}

fn parse_node_value(node_value: Pair<'_, Rule>) -> Result<NodeValue> {
    let inner: Pair<'_, Rule> = node_value.into_inner().next().unwrap();
    Ok(match inner.as_rule() {
        Rule::empty_node_array => empty_node_array(),
        Rule::populated_node_array => parse_populated_node_array(inner)?,
        Rule::empty_node_object => empty_node_object(),
        Rule::populated_node_object => parse_populated_node_object(inner)?,
        Rule::number => {
            if inner.as_str().contains('.') {
                NodeValue::Number(Number::Float(
                    inner.as_str().parse().chain_err(|| "number format error")?,
                ))
            } else {
                NodeValue::Number(Number::Integer(
                    inner.as_str().parse().chain_err(|| "number format error")?,
                ))
            }
        }
        Rule::kw_true => NodeValue::Boolean(true),
        Rule::kw_false => NodeValue::Boolean(false),
        Rule::kw_null => NodeValue::None,
        Rule::shape_id => NodeValue::ShapeID(ShapeID::from_str(inner.as_str())?),
        Rule::text_block => NodeValue::TextBlock(inner.as_str().to_string()),
        Rule::quoted_text => NodeValue::String(inner.as_str().to_string()),
        _ => unreachable!("parse_node_value > unreachable! {:#?}", inner),
    })
}

fn empty_node_array() -> NodeValue {
    NodeValue::Array(Default::default())
}

fn parse_populated_node_array(populated_node_array: Pair<'_, Rule>) -> Result<NodeValue> {
    let mut array: Vec<NodeValue> = Default::default();
    for inner in populated_node_array.into_inner() {
        match_rule!(inner,
            node_value => {array.push(parse_node_value(inner)?)}
        );
    }
    Ok(NodeValue::Array(array))
}

fn empty_node_object() -> NodeValue {
    NodeValue::Object(Default::default())
}

fn parse_populated_node_object(populated_node_object: Pair<'_, Rule>) -> Result<NodeValue> {
    let mut object: HashMap<Key, NodeValue> = Default::default();
    for inner in populated_node_object.into_inner() {
        match_rule!(inner,
           node_object_kvp => {
               let (key, value) = parse_node_object_kvp(inner)?;
               let _ = object.insert(key, value);
           }
        );
    }
    Ok(NodeValue::Object(object))
}

fn parse_node_object_kvp(node_object_kvp: Pair<'_, Rule>) -> Result<(Key, NodeValue)> {
    let mut key: Option<Key> = None;
    let mut value: Option<NodeValue> = None;
    for inner in node_object_kvp.into_inner() {
        match_rule!(inner,
            identifier => {
                key = Some(Key::Identifier(Identifier::from_str(inner.as_str())?));
            },
            quoted_text => {
                key = Some(Key::String(inner.as_str().to_string()));
            },
            node_value => {value = Some(parse_node_value(inner)?)}
        );
    }
    Ok(match (key, value) {
        (Some(key), Some(value)) => (key, value),
        _ => unreachable!("parse_node_object_kvp > unreachable!"),
    })
}

fn from_pest_error(e: PestError<Rule>) -> Error {
    Error::with_chain(
        e,
        ErrorKind::Deserialization("Smithy".to_string(), "pest".to_string(), None),
    )
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    const SMITHY: &str = r#"$version: 1.0

namespace example.weather // from spec

use aws.sdk#AShape

/// Provides weather forecasts.
/// Well, only a chance of rain really.
@paginated(inputToken: "nextToken", outputToken: "nextToken",
           pageSize: "pageSize")
service Weather {
    version: "2006-03-01",
    resources: [City],
    operations: [GetCurrentTime]
}

resource City {
    identifiers: { cityId: CityId },
    read: GetCity,
    list: ListCities,
    resources: [Forecast],
}

resource Forecast {
    identifiers: { cityId: CityId },
    read: GetForecast,
}

// "pattern" is a trait.
@pattern("^[A-Za-z0-9 ]+$")
string CityId

@readonly
operation GetCity {
    input: GetCityInput,
    output: GetCityOutput,
    errors: [NoSuchResource]
}

structure GetCityInput {
    // "cityId" provides the identifier for the resource and
    // has to be marked as required.
    @required
    cityId: CityId
}

structure GetCityOutput {
    // "required" is used on output to indicate if the service
    // will always provide a value for the member.
    @required
    name: String,

    @required
    coordinates: CityCoordinates,
}

// This structure is nested within GetCityOutput.
structure CityCoordinates {
    @required
    latitude: Float,

    @required
    longitude: Float,
}

// "error" is a trait that is used to specialize
// a structure as an error.
@error("client")
structure NoSuchResource {
    @required
    resourceType: String
}

// The paginated trait indicates that the operation may
// return truncated results.
@readonly
@paginated(items: "items")
operation ListCities {
    input: ListCitiesInput,
    output: ListCitiesOutput
}

structure ListCitiesInput {
    nextToken: String,
    pageSize: Integer
}

structure ListCitiesOutput {
    nextToken: String,

    @required
    items: CitySummaries,
}

// CitySummaries is a list of CitySummary structures.
list CitySummaries {
    member: CitySummary
}

// CitySummary contains a reference to a City.
@references([{resource: City}])
structure CitySummary {
    @required
    cityId: CityId,

    @required
    name: String,
}

@readonly
operation GetCurrentTime {
    output: GetCurrentTimeOutput
}

structure GetCurrentTimeOutput {
    @required
    time: Timestamp
}

@readonly
operation GetForecast {
    input: GetForecastInput,
    output: GetForecastOutput
}

// "cityId" provides the only identifier for the resource since
// a Forecast doesn't have its own.
structure GetForecastInput {
    @required
    cityId: CityId
}

structure GetForecastOutput {
    chanceOfRain: Float
}"#;

    #[cfg(feature = "debug")]
    #[test]
    fn test_low_level_parser() {
        hr(40);
        match SmithyParser::parse(Rule::idl, SMITHY) {
            Ok(parsed) => print!("{:#?}", parsed),
            Err(err) => panic!(err.to_string()),
        }
    }

    #[cfg(feature = "debug")]
    #[test]
    fn test_api_level_parser() {
        hr(40);
        match parse(SMITHY) {
            Ok(parsed) => print!("{:#?}", parsed),
            Err(err) => panic!(err.to_string()),
        }
    }

    fn hr(w: usize) {
        println!("\n{:-<width$}", "-", width = w);
    }
}
