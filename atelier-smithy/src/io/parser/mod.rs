use crate::io::parser::error::ParserError;
use atelier_core::error::{Error, ErrorKind, Result as CoreResult, ResultExt};
use atelier_core::model::shapes::{
    AppliedTrait, ListOrSet, Map, Member, Operation, Resource, Service, TopLevelShape, ShapeKind, Simple,
    StructureOrUnion,
};
use atelier_core::model::values::{Number, Value as NodeValue, ValueMap};
use atelier_core::model::{Identifier, Model, NamespaceID, ShapeID};
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
    ($fn_name:expr, $pair:expr) => {
        return ParserError::unexpected($fn_name, &$pair).into();
    };
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn parse_idl(input_pair: Pair<'_, Rule>) -> CoreResult<Model> {
    match input_pair.as_rule() {
        Rule::idl => {
            let mut control_data: ValueMap = Default::default();
            let mut meta_data: ValueMap = Default::default();
            let mut shape_section: Option<(NamespaceID, Vec<ShapeID>, Vec<TopLevelShape>)> = None;
            for inner in input_pair.into_inner() {
                match inner.as_rule() {
                    Rule::control_section => {
                        control_data = parse_control_section(inner)?;
                    }
                    Rule::metadata_section => {
                        meta_data = parse_metadata_section(inner)?;
                    }
                    Rule::shape_section => {
                        shape_section = Some(parse_shape_section(inner)?);
                    }
                    _ => unexpected!("parse_idl", inner),
                }
            }
            let version = if let Some(NodeValue::String(version)) = control_data.get("version") {
                Version::from_str(version)?
            } else {
                Version::default()
            };
            let shape_section = shape_section.unwrap();
            let mut model = Model::new(version);
            let namespace = shape_section.0;
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

fn parse_control_section(input_pair: Pair<'_, Rule>) -> CoreResult<ValueMap> {
    let mut map: ValueMap = Default::default();
    for inner in input_pair.into_inner() {
        match inner.as_rule() {
            Rule::control_statement => {
                let (key, value) = parse_node_object_kvp(inner)?;
                let _ = map.insert(key, value);
            }
            _ => unexpected!("parse_control_section", inner),
        }
    }
    Ok(map)
}

fn parse_metadata_section(input_pair: Pair<'_, Rule>) -> CoreResult<ValueMap> {
    let mut map: ValueMap = Default::default();
    for inner in input_pair.into_inner() {
        match inner.as_rule() {
            Rule::metadata_statement => {
                let (key, value) = parse_node_object_kvp(inner)?;
                let _ = map.insert(key, value);
            }
            _ => unexpected!("parse_metadata_section", inner),
        }
    }
    Ok(map)
}

fn parse_shape_section(
    input_pair: Pair<'_, Rule>,
) -> CoreResult<(NamespaceID, Vec<ShapeID>, Vec<TopLevelShape>)> {
    let mut namespace: Option<NamespaceID> = None;
    let mut uses: Vec<ShapeID> = Default::default();
    let mut shapes: Vec<TopLevelShape> = Default::default();
    for inner in input_pair.into_inner() {
        match inner.as_rule() {
            Rule::namespace_statement => {
                namespace = Some(parse_namespace_statement(inner)?);
            }
            Rule::use_section => {
                uses = parse_use_section(inner)?;
            }
            Rule::shape_statements => {
                shapes = parse_shape_statements(inner)?;
            }
            _ => unexpected!("parse_shape_section", inner),
        }
    }
    Ok((namespace.unwrap(), uses, shapes))
}

fn parse_namespace_statement(input_pair: Pair<'_, Rule>) -> CoreResult<NamespaceID> {
    let namespace: Pair<'_, Rule> = input_pair.into_inner().next().unwrap();
    if let Rule::namespace = namespace.as_rule() {
        NamespaceID::from_str(namespace.as_str())
    } else {
        ParserError::new("parse_namespace_statement")
            .context(&namespace)
            .into()
    }
}

fn parse_use_section(input_pair: Pair<'_, Rule>) -> CoreResult<Vec<ShapeID>> {
    let mut uses: Vec<ShapeID> = Default::default();
    for inner in input_pair.into_inner() {
        match inner.as_rule() {
            Rule::use_statement => {
                let absolute_root_shape_id: Pair<'_, Rule> = inner.into_inner().next().unwrap();
                if let Rule::absolute_root_shape_id = absolute_root_shape_id.as_rule() {
                    uses.push(ShapeID::from_str(absolute_root_shape_id.as_str())?);
                } else {
                    return ParserError::unreachable("parse_use_section")
                        .context(&absolute_root_shape_id)
                        .into();
                }
            }
            _ => unexpected!("parse_use_section", inner),
        }
    }
    Ok(uses)
}

fn parse_shape_statements(input_pair: Pair<'_, Rule>) -> CoreResult<Vec<TopLevelShape>> {
    let mut shapes: Vec<TopLevelShape> = Default::default();
    for shape_statement in input_pair.into_inner() {
        match shape_statement.as_rule() {
            Rule::shape_statement => {
                shapes.push(parse_shape_statement(shape_statement)?);
            }
            Rule::EOI => {}
            _ => unexpected!("parse_shape_statements", shape_statement),
        }
    }
    Ok(shapes)
}

fn parse_shape_statement(input_pair: Pair<'_, Rule>) -> CoreResult<TopLevelShape> {
    let mut documentation: Vec<String> = Default::default();
    let mut traits: Vec<AppliedTrait> = Default::default();
    let mut inner: Option<(Identifier, ShapeKind)> = None;
    let mut applies: Option<(ShapeID, AppliedTrait)> = None;
    for shape_statement in input_pair.into_inner() {
        match shape_statement.as_rule() {
            Rule::documentation_text => {
                documentation.push(parse_documentation_text(shape_statement)?);
            }
            Rule::trait_statements => {
                traits = parse_trait_statements(shape_statement)?;
            }
            Rule::simple_shape_statement => {
                inner = Some(parse_simple_shape_statement(shape_statement)?);
            }
            Rule::list_statement => {
                inner = Some(parse_list_statement(shape_statement)?);
            }
            Rule::set_statement => {
                inner = Some(parse_set_statement(shape_statement)?);
            }
            Rule::map_statement => {
                inner = Some(parse_map_statement(shape_statement)?);
            }
            Rule::structure_statement => {
                inner = Some(parse_structure_statement(shape_statement)?);
            }
            Rule::union_statement => {
                inner = Some(parse_union_statement(shape_statement)?);
            }
            Rule::service_statement => {
                inner = Some(parse_service_statement(shape_statement)?);
            }
            Rule::operation_statement => {
                inner = Some(parse_operation_statement(shape_statement)?);
            }
            Rule::resource_statement => {
                inner = Some(parse_resource_statement(shape_statement)?);
            }
            Rule::apply_statement => {
                applies = Some(parse_apply_statement(shape_statement)?);
            }
            _ => unexpected!("parse_shape_statement", shape_statement),
        }
    }
    if let Some((id, inner)) = inner {
        let mut shape = TopLevelShape::local(id, inner);
        if !documentation.is_empty() {
            shape.documentation(&documentation.join("\n"));
        }
        for a_trait in traits {
            shape.add_trait(a_trait);
        }
        Ok(shape)
    } else if let Some((id, a_trait)) = applies {
        let inner = ShapeKind::Apply;
        let mut shape = TopLevelShape::new(id, inner);
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

fn parse_trait_statements(input_pair: Pair<'_, Rule>) -> CoreResult<Vec<AppliedTrait>> {
    let mut traits: Vec<AppliedTrait> = Default::default();
    for a_trait in input_pair.into_inner() {
        match a_trait.as_rule() {
            Rule::a_trait => {
                traits.push(parse_a_trait(a_trait)?);
            }
            _ => unexpected!("parse_trait_statements", a_trait),
        }
    }
    Ok(traits)
}

fn parse_a_trait(input_pair: Pair<'_, Rule>) -> CoreResult<AppliedTrait> {
    let mut id: Option<ShapeID> = None;
    let mut node_value: Option<NodeValue> = None;
    let mut members: HashMap<Key, NodeValue> = Default::default();
    for inner in input_pair.into_inner() {
        match inner.as_rule() {
            Rule::shape_id => {
                id = Some(ShapeID::from_str(inner.as_str())?);
            }
            Rule::node_value => {
                node_value = Some(parse_node_value(inner)?);
            }
            Rule::trait_structure_kvp => {
                let (id, value) = parse_trait_structure_kvp(inner)?;
                let _ = members.insert(id, value);
            }
            _ => unexpected!("parse_a_trait", inner),
        }
    }
    if node_value.is_some() && !members.is_empty() {
        return ParserError::unreachable("parse_a_trait")
            .debug_context(&members)
            .into();
    } else if node_value.is_none() && !members.is_empty() {
        node_value = Some(NodeValue::Object(members));
    }
    match (id, node_value) {
        (Some(id), None) => Ok(AppliedTrait::new(id)),
        (Some(id), Some(node_value)) => Ok(AppliedTrait::with_value(id, node_value)),
        _ => ParserError::unreachable("parse_a_trait").into(),
    }
}

#[allow(unused_assignments)]
fn parse_trait_structure_kvp(input_pair: Pair<'_, Rule>) -> CoreResult<(Key, NodeValue)> {
    let mut id: Option<Key> = None;
    let mut node_value: Option<NodeValue> = None;
    for inner in input_pair.into_inner() {
        match inner.as_rule() {
            Rule::quoted_text => {
                for inner in inner.into_inner() {
                    match inner.as_rule() {
                        Rule::quoted_chars => {
                            // erroneous: value assigned to `id` is never read
                            id = Some(Key::String(inner.as_str().to_string()))
                        }
                        _ => unexpected!("parse_trait_structure_kvp", inner),
                    }
                }
                if id.is_none() {
                    return ParserError::unreachable("parse_trait_structure_kvp")
                        .in_rule("quoted_text")
                        .into();
                }
            }
            Rule::identifier => id = Some(Key::Identifier(Identifier::from_str(inner.as_str())?)),
            Rule::node_value => {
                node_value = Some(parse_node_value(inner)?);
            }
            _ => unexpected!("parse_trait_structure_kvp", inner),
        }
    }
    match (id, node_value) {
        (Some(id), Some(node_value)) => Ok((id, node_value)),
        _ => ParserError::unreachable("parse_a_trait").into(),
    }
}

fn parse_simple_shape_statement(input_pair: Pair<'_, Rule>) -> CoreResult<(Identifier, ShapeKind)> {
    let mut id: Option<Identifier> = None;
    let mut simple_type: Option<Simple> = None;
    for inner in input_pair.into_inner() {
        match inner.as_rule() {
            Rule::identifier => {
                id = Some(Identifier::from_str(inner.as_str())?);
            }
            Rule::type_blob => simple_type = Some(Simple::Blob),
            Rule::type_boolean => simple_type = Some(Simple::Boolean),
            Rule::type_document => simple_type = Some(Simple::Document),
            Rule::type_string => simple_type = Some(Simple::String),
            Rule::type_byte => simple_type = Some(Simple::Byte),
            Rule::type_short => simple_type = Some(Simple::Short),
            Rule::type_integer => simple_type = Some(Simple::Integer),
            Rule::type_long => simple_type = Some(Simple::Long),
            Rule::type_float => simple_type = Some(Simple::Float),
            Rule::type_double => simple_type = Some(Simple::Double),
            Rule::type_big_integer => simple_type = Some(Simple::BigInteger),
            Rule::type_big_decimal => simple_type = Some(Simple::BigDecimal),
            Rule::type_timestamp => simple_type = Some(Simple::Timestamp),
            _ => unexpected!("parse_simple_shape_statement", inner),
        }
    }
    match (id, simple_type) {
        (Some(id), Some(simple_type)) => Ok((id, ShapeKind::Simple(simple_type))),
        _ => ParserError::unreachable("parse_simple_shape_statement").into(),
    }
}

fn parse_list_statement(input_pair: Pair<'_, Rule>) -> CoreResult<(Identifier, ShapeKind)> {
    let (id, members) = parse_membered_statement(input_pair)?;
    for member in members {
        if member.id() == &Identifier::from_str("member").unwrap() {
            return Ok((
                id,
                ShapeKind::List(ListOrSet::new(
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

fn parse_set_statement(input_pair: Pair<'_, Rule>) -> CoreResult<(Identifier, ShapeKind)> {
    let (id, members) = parse_membered_statement(input_pair)?;
    for member in members {
        if member.id() == &Identifier::from_str("member").unwrap() {
            return Ok((
                id,
                ShapeKind::Set(ListOrSet::new(
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

fn parse_map_statement(input_pair: Pair<'_, Rule>) -> CoreResult<(Identifier, ShapeKind)> {
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
        (Some(k), Some(v)) => Ok((id, ShapeKind::Map(Map::new(k, v)))),
        _ => ParserError::unreachable("parse_map_statement").into(),
    }
}

fn parse_structure_statement(input_pair: Pair<'_, Rule>) -> CoreResult<(Identifier, ShapeKind)> {
    let (id, members) = parse_membered_statement(input_pair)?;
    Ok((
        id,
        ShapeKind::Structure(StructureOrUnion::with_members(members.as_slice())),
    ))
}

fn parse_union_statement(input_pair: Pair<'_, Rule>) -> CoreResult<(Identifier, ShapeKind)> {
    let (id, members) = parse_membered_statement(input_pair)?;
    Ok((
        id,
        ShapeKind::Union(StructureOrUnion::with_members(members.as_slice())),
    ))
}

fn parse_membered_statement(input_pair: Pair<'_, Rule>) -> CoreResult<(Identifier, Vec<Member>)> {
    let mut id: Option<Identifier> = None;
    let mut members: Vec<Member> = Default::default();
    for inner in input_pair.into_inner() {
        match inner.as_rule() {
            Rule::identifier => {
                id = Some(Identifier::from_str(inner.as_str())?);
            }
            Rule::empty_shape_members => {}
            Rule::populated_shape_members => {
                members = parse_populated_shape_members(inner)?;
            }
            _ => unexpected!("parse_membered_statement", inner),
        }
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
        match inner.as_rule() {
            Rule::hape_member_kvp => {
                members.push(parse_shape_member_kvp(inner)?);
            }
            _ => unexpected!("parse_populated_shape_members", inner),
        }
    }
    Ok(members)
}

fn parse_shape_member_kvp(input_pair: Pair<'_, Rule>) -> CoreResult<Member> {
    let mut documentation: Vec<String> = Default::default();
    let mut traits: Vec<AppliedTrait> = Default::default();
    let mut id: Option<Identifier> = None;
    let mut shape_id: Option<ShapeID> = None;
    for inner in input_pair.into_inner() {
        match inner.as_rule() {
            Rule::documentation_text => {
                documentation.push(parse_documentation_text(inner)?);
            }
            Rule::trait_statements => {
                traits = parse_trait_statements(inner)?;
            }
            Rule::identifier => {
                id = Some(Identifier::from_str(inner.as_str())?);
            }
            Rule::shape_id => {
                shape_id = Some(ShapeID::from_str(inner.as_str())?);
            }
            _ => unexpected!("parse_shape_member_kvp", inner),
        }
    }
    match (id, shape_id) {
        (Some(id), Some(shape_id)) => {
            let mut member = Member::new(id, shape_id);
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

fn parse_service_statement(input_pair: Pair<'_, Rule>) -> CoreResult<(Identifier, ShapeKind)> {
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
        Ok((id, ShapeKind::Service(service)))
    } else {
        ParserError::unreachable("parse_service_statement")
            .context(&object)
            .into()
    }
}

fn parse_operation_statement(input_pair: Pair<'_, Rule>) -> CoreResult<(Identifier, ShapeKind)> {
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
        Ok((id, ShapeKind::Operation(operation)))
    } else {
        ParserError::unreachable("parse_operation_statement")
            .context(&object)
            .into()
    }
}

fn parse_resource_statement(input_pair: Pair<'_, Rule>) -> CoreResult<(Identifier, ShapeKind)> {
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
        Ok((id, ShapeKind::Resource(resource)))
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
        match inner.as_rule() {
            Rule::identifier => {
                id = Some(Identifier::from_str(inner.as_str())?);
            }
            Rule::empty_node_object => node_value = Some(empty_node_object()),
            Rule::populated_node_object => node_value = Some(parse_populated_node_object(inner)?),
            _ => unexpected!("parse_id_and_object", inner),
        }
    }
    match (id, node_value) {
        (Some(id), Some(node_value)) => Ok((id, node_value)),
        _ => ParserError::unreachable("parse_id_and_object").into(),
    }
}

fn parse_apply_statement(input_pair: Pair<'_, Rule>) -> CoreResult<(ShapeID, AppliedTrait)> {
    let mut id: Option<ShapeID> = None;
    let mut a_trait: Option<AppliedTrait> = None;
    for inner in input_pair.into_inner() {
        match inner.as_rule() {
            Rule::identifier => {
                id = Some(ShapeID::from_str(inner.as_str())?);
            }
            Rule::a_trait => a_trait = Some(parse_a_trait(inner)?),
            _ => unexpected!("parse_apply_statement", inner),
        }
    }
    match (id, a_trait) {
        (Some(id), Some(a_trait)) => Ok((id, a_trait)),
        _ => ParserError::unreachable("parse_apply_statement").into(),
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
        Rule::shape_id => NodeValue::String(ShapeID::from_str(inner.as_str())?.to_string()),
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
        match inner.as_rule() {
            Rule::quoted_chars => return Ok(NodeValue::String(inner.as_str().to_string())),
            _ => unexpected!("parse_quoted_text", inner),
        }
    }
}

fn parse_text_block(input_pair: Pair<'_, Rule>) -> CoreResult<NodeValue> {
    for inner in input_pair.into_inner() {
        match inner.as_rule() {
            Rule::block_quoted_chars => return Ok(NodeValue::String(inner.as_str().to_string())),
            _ => unexpected!("parse_text_block", inner),
        }
    }
}

fn empty_node_array() -> NodeValue {
    NodeValue::Array(Default::default())
}

fn parse_populated_node_array(input_pair: Pair<'_, Rule>) -> CoreResult<NodeValue> {
    let mut array: Vec<NodeValue> = Default::default();
    for inner in input_pair.into_inner() {
        match inner.as_rule() {
            Rule::node_value => array.push(parse_node_value(inner)?),
            _ => unexpected!("parse_populated_node_array", inner),
        }
    }
    Ok(NodeValue::Array(array))
}

fn empty_node_object() -> NodeValue {
    NodeValue::Object(Default::default())
}

fn parse_populated_node_object(input_pair: Pair<'_, Rule>) -> CoreResult<NodeValue> {
    let mut object: ValueMap = Default::default();
    for inner in input_pair.into_inner() {
        match inner.as_rule() {
            Rule::node_object_kvp => {
                let (key, value) = parse_node_object_kvp(inner)?;
                let _ = object.insert(key, value);
            }
            _ => unexpected!("parse_populated_node_object", inner),
        }
    }
    Ok(NodeValue::Object(object))
}

fn parse_node_object_kvp(input_pair: Pair<'_, Rule>) -> CoreResult<(Key, NodeValue)> {
    let mut key: Option<Key> = None;
    let mut value: Option<NodeValue> = None;
    for inner in input_pair.into_inner() {
        match inner.as_rule() {
            Rule::identifier => {
                key = Some(Key::Identifier(Identifier::from_str(inner.as_str())?));
            }
            Rule::quoted_text => {
                key = Some(Key::String(inner.as_str().to_string()));
            }
            Rule::node_value => value = Some(parse_node_value(inner)?),
            _ => unexpected!("parse_node_object_kvp", inner),
        }
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
