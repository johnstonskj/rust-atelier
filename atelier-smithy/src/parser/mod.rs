use crate::parser::error::ParserError;
use atelier_core::error::{Error, ErrorKind, Result as CoreResult, ResultExt};
use atelier_core::model::shapes::{
    HasMembers, ListOrSet, Map, Member, Operation, Resource, Service, Shape, ShapeBody, SimpleType,
    StructureOrUnion, Trait, Valued,
};
use atelier_core::model::values::{Key, NodeValue, Number};
use atelier_core::model::{Annotated, Identifier, Model, Named, Namespace, ShapeID};
use atelier_core::syntax::{
    MEMBER_COLLECTION_OPERATIONS, MEMBER_CREATE, MEMBER_DELETE, MEMBER_ERRORS, MEMBER_IDENTIFIERS,
    MEMBER_INPUT, MEMBER_LIST, MEMBER_OPERATIONS, MEMBER_OUTPUT, MEMBER_PUT, MEMBER_READ,
    MEMBER_RESOURCES, MEMBER_UPDATE, MEMBER_VERSION,
};
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

pub(crate) fn parse(input: &str) -> CoreResult<Model> {
    let mut parsed = SmithyParser::parse(Rule::idl, input).map_err(from_pest_error)?;
    let top_node = parsed.next().unwrap();
    parse_idl(top_node)
}

#[cfg(feature = "debug")]
#[allow(dead_code)]
pub(crate) fn parse_and_debug(w: &mut impl Write, input: &str) -> CoreResult<()> {
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

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn parse_idl(input_pair: Pair<'_, Rule>) -> CoreResult<Model> {
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
        _ => ParserError::unexpected("parse_idl", &input_pair).into(),
    }
}

fn parse_control_section(input_pair: Pair<'_, Rule>) -> CoreResult<HashMap<Key, NodeValue>> {
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

fn parse_metadata_section(input_pair: Pair<'_, Rule>) -> CoreResult<HashMap<Key, NodeValue>> {
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
) -> CoreResult<(Namespace, Vec<ShapeID>, Vec<Shape>)> {
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

fn parse_namespace_statement(input_pair: Pair<'_, Rule>) -> CoreResult<Namespace> {
    let namespace: Pair<'_, Rule> = input_pair.into_inner().next().unwrap();
    if let Rule::namespace = namespace.as_rule() {
        Namespace::from_str(namespace.as_str())
    } else {
        ParserError::new("parse_namespace_statement")
            .context(&namespace)
            .into()
    }
}

fn parse_use_section(input_pair: Pair<'_, Rule>) -> CoreResult<Vec<ShapeID>> {
    let mut uses: Vec<ShapeID> = Default::default();
    for use_statement in input_pair.into_inner() {
        match_rule!(use_statement,
           use_statement => {
               let absolute_root_shape_id: Pair<'_, Rule> =
                   use_statement.into_inner().next().unwrap();
               if let Rule::absolute_root_shape_id = absolute_root_shape_id.as_rule() {
                   uses.push(ShapeID::from_str(absolute_root_shape_id.as_str())?);
               } else {
                   return ParserError::unreachable("parse_use_section").context(&absolute_root_shape_id).into()
               }
           }
        );
    }
    Ok(uses)
}

fn parse_shape_statements(input_pair: Pair<'_, Rule>) -> CoreResult<Vec<Shape>> {
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

fn parse_shape_statement(input_pair: Pair<'_, Rule>) -> CoreResult<Shape> {
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
        ParserError::new("parse_shape_statement").into()
    }
}

fn parse_documentation_text(input_pair: Pair<'_, Rule>) -> CoreResult<String> {
    if let Rule::documentation_text = input_pair.as_rule() {
        Ok(input_pair.as_str().to_string())
    } else {
        ParserError::new("parse_documentation_text")
            .context(&input_pair)
            .into()
    }
}

fn parse_trait_statements(input_pair: Pair<'_, Rule>) -> CoreResult<Vec<Trait>> {
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

fn parse_a_trait(input_pair: Pair<'_, Rule>) -> CoreResult<Trait> {
    let mut id: Option<ShapeID> = None;
    let mut node_value: Option<NodeValue> = None;
    let mut members: HashMap<Key, NodeValue> = Default::default();
    for inner in input_pair.into_inner() {
        match_rule!(inner,
            shape_id => {
                id = Some(ShapeID::from_str(inner.as_str())?);
            },
            node_value => {
                node_value = Some(parse_node_value(inner)?);
            },
            trait_structure_kvp => {
                let (id, value) = parse_trait_structure_kvp(inner)?;
                let _ = members.insert(id, value);
            }
        );
    }
    if node_value.is_some() && !members.is_empty() {
        return ParserError::unreachable("parse_a_trait")
            .debug_context(&members)
            .into();
    } else if node_value.is_none() && !members.is_empty() {
        node_value = Some(NodeValue::Object(members));
    }
    match (id, node_value) {
        (Some(id), None) => Ok(Trait::new(id)),
        (Some(id), Some(node_value)) => Ok(Trait::with_value(id, node_value)),
        _ => ParserError::unreachable("parse_a_trait").into(),
    }
}

#[allow(unused_assignments)]
fn parse_trait_structure_kvp(input_pair: Pair<'_, Rule>) -> CoreResult<(Key, NodeValue)> {
    let mut id: Option<Key> = None;
    let mut node_value: Option<NodeValue> = None;
    for inner in input_pair.into_inner() {
        match_rule!(inner,
            quoted_text => {
                for inner in inner.into_inner() {
                    match_rule!(inner,
                        quoted_chars => {
                            // erroneous: value assigned to `id` is never read
                            id = Some(Key::String(inner.as_str().to_string()))
                        }
                    );
                }
                if id.is_none() {
                    return ParserError::unreachable("parse_trait_structure_kvp").in_rule("quoted_text").into()
                }
            },
            identifier => {
                id = Some(Key::Identifier(Identifier::from_str(inner.as_str())?))
            },
            node_value => {
                node_value = Some(parse_node_value(inner)?);
            }
        );
    }
    match (id, node_value) {
        (Some(id), Some(node_value)) => Ok((id, node_value)),
        _ => ParserError::unreachable("parse_a_trait").into(),
    }
}

fn parse_simple_shape_statement(input_pair: Pair<'_, Rule>) -> CoreResult<(Identifier, ShapeBody)> {
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
    match (id, simple_type) {
        (Some(id), Some(simple_type)) => Ok((id, ShapeBody::SimpleType(simple_type))),
        _ => ParserError::unreachable("parse_simple_shape_statement").into(),
    }
}

fn parse_list_statement(input_pair: Pair<'_, Rule>) -> CoreResult<(Identifier, ShapeBody)> {
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
    ParserError::unreachable("parse_list_statement").into()
}

fn parse_set_statement(input_pair: Pair<'_, Rule>) -> CoreResult<(Identifier, ShapeBody)> {
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
    ParserError::unreachable("parse_set_statement").into()
}

fn parse_map_statement(input_pair: Pair<'_, Rule>) -> CoreResult<(Identifier, ShapeBody)> {
    let (id, members) = parse_membered_statement(input_pair)?;
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
            return ParserError::new("parse_map_statement")
                .unreachable_rule()
                .debug_context(&member)
                .into();
        }
    }
    match (key, value) {
        (Some(k), Some(v)) => Ok((id, ShapeBody::Map(Map::new(k, v)))),
        _ => ParserError::unreachable("parse_map_statement").into(),
    }
}

fn parse_structure_statement(input_pair: Pair<'_, Rule>) -> CoreResult<(Identifier, ShapeBody)> {
    let (id, members) = parse_membered_statement(input_pair)?;
    Ok((
        id,
        ShapeBody::Structure(StructureOrUnion::with_members(members.as_slice())),
    ))
}

fn parse_union_statement(input_pair: Pair<'_, Rule>) -> CoreResult<(Identifier, ShapeBody)> {
    let (id, members) = parse_membered_statement(input_pair)?;
    Ok((
        id,
        ShapeBody::Union(StructureOrUnion::with_members(members.as_slice())),
    ))
}

fn parse_membered_statement(input_pair: Pair<'_, Rule>) -> CoreResult<(Identifier, Vec<Member>)> {
    let mut id: Option<Identifier> = None;
    let mut members: Vec<Member> = Default::default();
    for inner in input_pair.into_inner() {
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
        ParserError::unreachable("parse_membered_statement").into()
    }
}

fn parse_populated_shape_members(input_pair: Pair<'_, Rule>) -> CoreResult<Vec<Member>> {
    let mut members = Vec::default();
    for inner in input_pair.into_inner() {
        match_rule!(inner,
            shape_member_kvp => {
                members.push(parse_shape_member_kvp(inner)?);
            }
        );
    }
    Ok(members)
}

fn parse_shape_member_kvp(input_pair: Pair<'_, Rule>) -> CoreResult<Member> {
    let mut documentation: Vec<String> = Default::default();
    let mut traits: Vec<Trait> = Default::default();
    let mut id: Option<Identifier> = None;
    let mut shape_id: Option<ShapeID> = None;
    for inner in input_pair.into_inner() {
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
        _ => ParserError::unreachable("parse_shape_member_kvp").into(),
    }
}

fn parse_service_statement(input_pair: Pair<'_, Rule>) -> CoreResult<(Identifier, ShapeBody)> {
    let (id, object) = parse_id_and_object(input_pair)?;
    if let NodeValue::Object(object) = object {
        let mut service = Service::default();
        for (key, value) in object {
            match &key {
                Key::Identifier(id) => {
                    if id.to_string() == MEMBER_VERSION
                        || id.to_string() == MEMBER_OPERATIONS
                        || id.to_string() == MEMBER_RESOURCES
                    {
                        service.set_member(Member::with_value(id.clone(), value))?;
                    } else {
                        return Err(ErrorKind::UnknownMember(key.to_string()).into());
                    }
                }
                _ => {
                    return ParserError::unreachable("parse_service_statement")
                        .context(&key)
                        .into()
                }
            }
        }
        Ok((id, ShapeBody::Service(service)))
    } else {
        ParserError::unreachable("parse_service_statement")
            .context(&object)
            .into()
    }
}

fn parse_operation_statement(input_pair: Pair<'_, Rule>) -> CoreResult<(Identifier, ShapeBody)> {
    let (id, object) = parse_id_and_object(input_pair)?;
    if let NodeValue::Object(object) = object {
        let mut operation = Operation::default();
        for (key, value) in object {
            match &key {
                Key::Identifier(id) => {
                    if id.to_string() == MEMBER_INPUT
                        || id.to_string() == MEMBER_OUTPUT
                        || id.to_string() == MEMBER_ERRORS
                    {
                        operation.set_member(Member::with_value(id.clone(), value))?;
                    } else {
                        return Err(ErrorKind::UnknownMember(key.to_string()).into());
                    }
                }
                _ => {
                    return ParserError::unreachable("parse_operation_statement")
                        .context(&key)
                        .into()
                }
            }
        }
        Ok((id, ShapeBody::Operation(operation)))
    } else {
        ParserError::unreachable("parse_operation_statement")
            .context(&object)
            .into()
    }
}

fn parse_resource_statement(input_pair: Pair<'_, Rule>) -> CoreResult<(Identifier, ShapeBody)> {
    let (id, object) = parse_id_and_object(input_pair)?;
    if let NodeValue::Object(object) = object {
        let mut resource = Resource::default();
        for (key, value) in object {
            match &key {
                Key::Identifier(id) => {
                    if id.to_string() == MEMBER_IDENTIFIERS
                        || id.to_string() == MEMBER_CREATE
                        || id.to_string() == MEMBER_PUT
                        || id.to_string() == MEMBER_READ
                        || id.to_string() == MEMBER_UPDATE
                        || id.to_string() == MEMBER_DELETE
                        || id.to_string() == MEMBER_LIST
                        || id.to_string() == MEMBER_OPERATIONS
                        || id.to_string() == MEMBER_COLLECTION_OPERATIONS
                        || id.to_string() == MEMBER_RESOURCES
                    {
                        resource.set_member(Member::with_value(id.clone(), value))?;
                    } else {
                        return Err(ErrorKind::UnknownMember(key.to_string()).into());
                    }
                }
                _ => {
                    return ParserError::unreachable("parse_resource_statement")
                        .context(&key)
                        .into()
                }
            }
        }
        Ok((id, ShapeBody::Resource(resource)))
    } else {
        ParserError::unreachable("parse_resource_statement")
            .context(&object)
            .into()
    }
}

fn parse_id_and_object(input_pair: Pair<'_, Rule>) -> CoreResult<(Identifier, NodeValue)> {
    let mut id: Option<Identifier> = None;
    let mut node_value: Option<NodeValue> = None;
    for inner in input_pair.into_inner() {
        match_rule!(inner,
            identifier => {
                id = Some(Identifier::from_str(inner.as_str())?);
            },
            empty_node_object => {node_value = Some(empty_node_object())},
            populated_node_object => {node_value = Some(parse_populated_node_object(inner)?)}
        );
    }
    match (id, node_value) {
        (Some(id), Some(node_value)) => Ok((id, node_value)),
        _ => ParserError::unreachable("parse_id_and_object").into(),
    }
}

fn parse_apply_statement(input_pair: Pair<'_, Rule>) -> CoreResult<(ShapeID, Trait)> {
    let mut id: Option<ShapeID> = None;
    let mut a_trait: Option<Trait> = None;
    for inner in input_pair.into_inner() {
        match_rule!(inner,
            identifier => {
                id = Some(ShapeID::from_str(inner.as_str())?);
            },
            a_trait => {a_trait = Some(parse_a_trait(inner)?)}
        );
    }
    match (id, a_trait) {
        (Some(id), Some(a_trait)) => Ok((id, a_trait)),
        _ => ParserError::unreachable("parse_id_and_object").into(),
    }
}

fn parse_node_value(input_pair: Pair<'_, Rule>) -> CoreResult<NodeValue> {
    let inner: Pair<'_, Rule> = input_pair.into_inner().next().unwrap();
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
        Rule::text_block => parse_text_block(inner)?,
        Rule::quoted_text => parse_quoted_text(inner)?,
        _ => {
            return ParserError::unreachable("parse_node_value")
                .debug_context(&inner)
                .into()
        }
    })
}

fn parse_quoted_text(input_pair: Pair<'_, Rule>) -> CoreResult<NodeValue> {
    for inner in input_pair.into_inner() {
        match_rule!(inner,
            quoted_chars => {return Ok(NodeValue::String(inner.as_str().to_string()))}
        );
    }
    ParserError::unreachable("parse_quoted_text").into()
}

fn parse_text_block(input_pair: Pair<'_, Rule>) -> CoreResult<NodeValue> {
    for inner in input_pair.into_inner() {
        match_rule!(inner,
            block_quoted_chars => {return Ok(NodeValue::TextBlock(inner.as_str().to_string()))}
        );
    }
    ParserError::unreachable("parse_text_block").into()
}

fn empty_node_array() -> NodeValue {
    NodeValue::Array(Default::default())
}

fn parse_populated_node_array(input_pair: Pair<'_, Rule>) -> CoreResult<NodeValue> {
    let mut array: Vec<NodeValue> = Default::default();
    for inner in input_pair.into_inner() {
        match_rule!(inner,
            node_value => {array.push(parse_node_value(inner)?)}
        );
    }
    Ok(NodeValue::Array(array))
}

fn empty_node_object() -> NodeValue {
    NodeValue::Object(Default::default())
}

fn parse_populated_node_object(input_pair: Pair<'_, Rule>) -> CoreResult<NodeValue> {
    let mut object: HashMap<Key, NodeValue> = Default::default();
    for inner in input_pair.into_inner() {
        match_rule!(inner,
           node_object_kvp => {
               let (key, value) = parse_node_object_kvp(inner)?;
               let _ = object.insert(key, value);
           }
        );
    }
    Ok(NodeValue::Object(object))
}

fn parse_node_object_kvp(input_pair: Pair<'_, Rule>) -> CoreResult<(Key, NodeValue)> {
    let mut key: Option<Key> = None;
    let mut value: Option<NodeValue> = None;
    for inner in input_pair.into_inner() {
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
    match (key, value) {
        (Some(key), Some(value)) => Ok((key, value)),
        _ => ParserError::unreachable("parse_node_value").into(),
    }
}

fn from_pest_error(e: PestError<Rule>) -> Error {
    Error::with_chain(
        e,
        ErrorKind::Deserialization("Smithy".to_string(), "pest".to_string(), None),
    )
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod error;

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
