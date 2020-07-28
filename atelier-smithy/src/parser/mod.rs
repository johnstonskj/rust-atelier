use crate::parser::error::ParserError;
use atelier_core::builder::shapes::ShapeTraits;
use atelier_core::builder::{
    ListBuilder, MapBuilder, MemberBuilder, ModelBuilder, OperationBuilder, ReferenceBuilder,
    ResourceBuilder, ServiceBuilder, SimpleShapeBuilder, StructureBuilder, TraitBuilder,
};
use atelier_core::error::{Error, ErrorKind, Result as ModelResult, ResultExt};
use atelier_core::model::shapes::Simple;
use atelier_core::model::values::{Number, Value as NodeValue, ValueMap};
use atelier_core::model::Model;
use atelier_core::syntax::{
    MEMBER_COLLECTION_OPERATIONS, MEMBER_CREATE, MEMBER_DELETE, MEMBER_ERRORS, MEMBER_IDENTIFIERS,
    MEMBER_INPUT, MEMBER_KEY, MEMBER_LIST, MEMBER_MEMBER, MEMBER_OPERATIONS, MEMBER_OUTPUT,
    MEMBER_PUT, MEMBER_READ, MEMBER_RESOURCES, MEMBER_UPDATE, MEMBER_VALUE, MEMBER_VERSION,
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

pub(crate) fn parse(input: &str) -> ModelResult<Model> {
    let mut parsed = SmithyParser::parse(Rule::idl, input).map_err(from_pest_error)?;
    let top_node = parsed.next().unwrap();
    parse_idl(top_node)
}

#[cfg(feature = "debug")]
#[allow(dead_code)]
pub(crate) fn parse_and_debug(w: &mut impl Write, input: &str) -> ModelResult<()> {
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

fn parse_idl(input_pair: Pair<'_, Rule>) -> ModelResult<Model> {
    match input_pair.as_rule() {
        Rule::idl => {
            let mut control_data: ValueMap = Default::default();
            let mut meta_data: ValueMap = Default::default();
            let mut builder: Option<ModelBuilder> = None;
            for inner in input_pair.into_inner() {
                match inner.as_rule() {
                    Rule::control_section => {
                        control_data = parse_control_section(inner)?;
                    }
                    Rule::metadata_section => {
                        meta_data = parse_metadata_section(inner)?;
                    }
                    Rule::shape_section => {
                        let version =
                            if let Some(NodeValue::String(version)) = control_data.get("version") {
                                Version::from_str(version)?
                            } else {
                                Version::default()
                            };
                        builder = Some(parse_shape_section(inner, version)?);
                    }
                    Rule::EOI => {}
                    _ => unexpected!("parse_idl", inner),
                }
            }
            let mut builder = builder.unwrap();
            for (k, v) in meta_data {
                let _ = builder.meta_data(k, v);
            }
            Ok((&mut builder).into())
        }
        _ => unexpected!("parse_idl", input_pair),
    }
}

fn parse_control_section(input_pair: Pair<'_, Rule>) -> ModelResult<ValueMap> {
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

fn parse_metadata_section(input_pair: Pair<'_, Rule>) -> ModelResult<ValueMap> {
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

fn parse_shape_section(input_pair: Pair<'_, Rule>, version: Version) -> ModelResult<ModelBuilder> {
    let mut builder: Option<ModelBuilder> = None;
    for inner in input_pair.into_inner() {
        match inner.as_rule() {
            Rule::namespace_statement => {
                builder = Some(parse_namespace_statement(inner, version)?);
            }
            Rule::use_section => {
                parse_use_section(inner, builder.as_mut().unwrap())?;
            }
            Rule::shape_statements => {
                parse_shape_statements(inner, builder.as_mut().unwrap())?;
            }
            _ => unexpected!("parse_shape_section", inner),
        }
    }
    Ok(builder.unwrap())
}

fn parse_namespace_statement(
    input_pair: Pair<'_, Rule>,
    version: Version,
) -> ModelResult<ModelBuilder> {
    let namespace: Pair<'_, Rule> = input_pair.into_inner().next().unwrap();
    if let Rule::namespace = namespace.as_rule() {
        Ok(ModelBuilder::new(version, namespace.as_str()))
    } else {
        ParserError::new("parse_namespace_statement").into()
    }
}

fn parse_use_section(input_pair: Pair<'_, Rule>, builder: &mut ModelBuilder) -> ModelResult<()> {
    for inner in input_pair.into_inner() {
        match inner.as_rule() {
            Rule::use_statement => {
                let absolute_root_shape_id: Pair<'_, Rule> = inner.into_inner().next().unwrap();
                if let Rule::absolute_root_shape_id = absolute_root_shape_id.as_rule() {
                    let _ = builder.uses(absolute_root_shape_id.as_str());
                } else {
                    return ParserError::unreachable("parse_use_section")
                        .context(&absolute_root_shape_id)
                        .into();
                }
            }
            _ => unexpected!("parse_use_section", inner),
        }
    }
    Ok(())
}

fn parse_shape_statements(
    input_pair: Pair<'_, Rule>,
    builder: &mut ModelBuilder,
) -> ModelResult<()> {
    for shape_statement in input_pair.into_inner() {
        match shape_statement.as_rule() {
            Rule::shape_statement => {
                parse_shape_statement(shape_statement, builder)?;
            }
            Rule::EOI => {}
            _ => unexpected!("parse_shape_statements", shape_statement),
        }
    }
    Ok(())
}

fn parse_shape_statement(
    input_pair: Pair<'_, Rule>,
    builder: &mut ModelBuilder,
) -> ModelResult<()> {
    let mut documentation: Vec<String> = Default::default();
    let mut traits: Vec<TraitBuilder> = Default::default();
    for shape_statement in input_pair.into_inner() {
        match shape_statement.as_rule() {
            Rule::documentation_text => {
                documentation.push(parse_documentation_text(shape_statement)?);
            }
            Rule::trait_statements => {
                traits = parse_trait_statements(shape_statement)?;
            }
            Rule::simple_shape_statement => {
                let mut shape = parse_simple_shape_statement(shape_statement)?;
                apply_traits(&mut shape, &documentation, &traits);
                let _ = builder.simple_shape(shape);
            }
            Rule::list_statement => {
                let mut shape = parse_list_statement(shape_statement)?;
                apply_traits(&mut shape, &documentation, &traits);
                let _ = builder.list(shape);
            }
            Rule::set_statement => {
                let mut shape = parse_list_statement(shape_statement)?;
                apply_traits(&mut shape, &documentation, &traits);
                let _ = builder.set(shape);
            }
            Rule::map_statement => {
                let mut shape = parse_map_statement(shape_statement)?;
                apply_traits(&mut shape, &documentation, &traits);
                let _ = builder.map(shape);
            }
            Rule::structure_statement => {
                let mut shape = parse_structure_statement(shape_statement)?;
                apply_traits(&mut shape, &documentation, &traits);
                let _ = builder.structure(shape);
            }
            Rule::union_statement => {
                let mut shape = parse_union_statement(shape_statement)?;
                apply_traits(&mut shape, &documentation, &traits);
                let _ = builder.union(shape);
            }
            Rule::service_statement => {
                let mut shape = parse_service_statement(shape_statement)?;
                apply_traits(&mut shape, &documentation, &traits);
                let _ = builder.service(shape);
            }
            Rule::operation_statement => {
                let mut shape = parse_operation_statement(shape_statement)?;
                apply_traits(&mut shape, &documentation, &traits);
                let _ = builder.operation(shape);
            }
            Rule::resource_statement => {
                let mut shape = parse_resource_statement(shape_statement)?;
                apply_traits(&mut shape, &documentation, &traits);
                let _ = builder.resource(shape);
            }
            Rule::apply_statement => {
                let shape = parse_apply_statement(shape_statement)?;
                let _ = builder.reference(shape);
            }
            Rule::EOI => {}
            _ => unexpected!("parse_shape_statement", shape_statement),
        }
    }
    Ok(())
}

fn parse_documentation_text(input_pair: Pair<'_, Rule>) -> ModelResult<String> {
    if let Rule::documentation_text = input_pair.as_rule() {
        Ok(input_pair.as_str().to_string())
    } else {
        ParserError::new("parse_documentation_text")
            .context(&input_pair)
            .into()
    }
}

fn parse_trait_statements(input_pair: Pair<'_, Rule>) -> ModelResult<Vec<TraitBuilder>> {
    let mut traits: Vec<TraitBuilder> = Default::default();
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

fn parse_a_trait(input_pair: Pair<'_, Rule>) -> ModelResult<TraitBuilder> {
    let mut id: Option<String> = None;
    let mut node_value: Option<NodeValue> = None;
    let mut members: HashMap<String, NodeValue> = Default::default();
    for inner in input_pair.into_inner() {
        match inner.as_rule() {
            Rule::shape_id => {
                id = Some(inner.as_str().to_string());
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
        (Some(id), None) => Ok(TraitBuilder::new(&id)),
        (Some(id), Some(node_value)) => Ok(TraitBuilder::with_value(&id, node_value)),
        _ => ParserError::unreachable("parse_a_trait").into(),
    }
}

#[allow(unused_assignments)]
fn parse_trait_structure_kvp(input_pair: Pair<'_, Rule>) -> ModelResult<(String, NodeValue)> {
    let mut id: Option<String> = None;
    let mut node_value: Option<NodeValue> = None;
    for inner in input_pair.into_inner() {
        match inner.as_rule() {
            Rule::quoted_text => {
                for inner in inner.into_inner() {
                    match inner.as_rule() {
                        Rule::quoted_chars => {
                            // erroneous: value assigned to `id` is never read
                            id = Some(inner.as_str().to_string())
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
            Rule::identifier => id = Some(inner.as_str().to_string()),
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

fn parse_simple_shape_statement(input_pair: Pair<'_, Rule>) -> ModelResult<SimpleShapeBuilder> {
    let mut id: Option<String> = None;
    let mut simple_type: Option<Simple> = None;
    for inner in input_pair.into_inner() {
        match inner.as_rule() {
            Rule::identifier => {
                id = Some(inner.as_str().to_string());
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
        (Some(shape_name), Some(simple_type)) => {
            Ok(SimpleShapeBuilder::new(&shape_name, simple_type))
        }
        _ => ParserError::unreachable("parse_simple_shape_statement").into(),
    }
}

fn parse_list_statement(input_pair: Pair<'_, Rule>) -> ModelResult<ListBuilder> {
    let (id, members) = parse_membered_statement(input_pair)?;
    for member in members {
        return if member.name() == MEMBER_MEMBER {
            Ok(ListBuilder::new(&id, member.target()))
        } else {
            ParserError::new("parse_list_statement")
                .unreachable_rule()
                .debug_context(&member)
                .into()
        };
    }
    ParserError::unreachable("parse_list_statement").into()
}

fn parse_map_statement(input_pair: Pair<'_, Rule>) -> ModelResult<MapBuilder> {
    let (id, members) = parse_membered_statement(input_pair)?;
    let mut key: Option<String> = None;
    let mut value: Option<String> = None;
    for member in members {
        if member.name() == MEMBER_KEY {
            key = Some(member.target().to_string())
        } else if member.name() == MEMBER_VALUE {
            value = Some(member.target().to_string())
        } else {
            return ParserError::new("parse_map_statement")
                .unreachable_rule()
                .debug_context(&member)
                .into();
        }
    }
    match (key, value) {
        (Some(k), Some(v)) => Ok(MapBuilder::new(&id, &k, &v)),
        _ => ParserError::unreachable("parse_map_statement").into(),
    }
}

fn parse_structure_statement(input_pair: Pair<'_, Rule>) -> ModelResult<StructureBuilder> {
    let (id, members) = parse_membered_statement(input_pair)?;
    let mut shape = StructureBuilder::new(&id);
    for member in members {
        let _ = shape.add_member(member);
    }
    Ok(shape)
}

fn parse_union_statement(input_pair: Pair<'_, Rule>) -> ModelResult<StructureBuilder> {
    let (id, members) = parse_membered_statement(input_pair)?;
    let mut shape = StructureBuilder::new(&id);
    for member in members {
        let _ = shape.add_member(member);
    }
    Ok(shape)
}

fn parse_membered_statement(
    input_pair: Pair<'_, Rule>,
) -> ModelResult<(String, Vec<MemberBuilder>)> {
    let mut id: Option<String> = None;
    let mut members: Vec<MemberBuilder> = Default::default();
    for inner in input_pair.into_inner() {
        match inner.as_rule() {
            Rule::identifier => {
                id = Some(inner.as_str().to_string());
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

fn parse_populated_shape_members(input_pair: Pair<'_, Rule>) -> ModelResult<Vec<MemberBuilder>> {
    let mut members = Vec::default();
    for inner in input_pair.into_inner() {
        match inner.as_rule() {
            Rule::shape_member_kvp => {
                members.push(parse_shape_member_kvp(inner)?);
            }
            _ => unexpected!("parse_populated_shape_members", inner),
        }
    }
    Ok(members)
}

fn parse_shape_member_kvp(input_pair: Pair<'_, Rule>) -> ModelResult<MemberBuilder> {
    let mut documentation: Vec<String> = Default::default();
    let mut traits: Vec<TraitBuilder> = Default::default();
    let mut id: Option<String> = None;
    let mut shape_id: Option<String> = None;
    for inner in input_pair.into_inner() {
        match inner.as_rule() {
            Rule::documentation_text => {
                documentation.push(parse_documentation_text(inner)?);
            }
            Rule::trait_statements => {
                traits = parse_trait_statements(inner)?;
            }
            Rule::identifier => {
                id = Some(inner.as_str().to_string());
            }
            Rule::shape_id => {
                shape_id = Some(inner.as_str().to_string());
            }
            _ => unexpected!("parse_shape_member_kvp", inner),
        }
    }
    match (id, shape_id) {
        (Some(id), Some(shape_id)) => {
            let mut member = MemberBuilder::new(&id, &shape_id);
            apply_traits(&mut member, &documentation, &traits);
            Ok(member)
        }
        _ => ParserError::unreachable("parse_shape_member_kvp").into(),
    }
}

fn parse_service_statement(input_pair: Pair<'_, Rule>) -> ModelResult<ServiceBuilder> {
    let (id, object) = parse_id_and_object(input_pair)?;
    if let NodeValue::Object(object) = object {
        let mut service = ServiceBuilder::new(
            &id,
            object.get(MEMBER_VERSION).unwrap().as_string().unwrap(),
        );
        for (key, value) in object {
            match key.as_str() {
                MEMBER_VERSION => {}
                MEMBER_OPERATIONS => {
                    if let NodeValue::Array(values) = value {
                        for value in values {
                            let _ = service.operation(&value.as_string().unwrap());
                        }
                    } else {
                        return ParserError::unreachable("parse_service_statement")
                            .context(&value)
                            .into();
                    }
                }
                MEMBER_RESOURCES => {
                    if let NodeValue::Array(values) = value {
                        for value in values {
                            let _ = service.resource(&value.as_string().unwrap());
                        }
                    } else {
                        return ParserError::unreachable("parse_service_statement")
                            .context(&value)
                            .into();
                    }
                }
                _ => {
                    return ParserError::unreachable("parse_service_statement")
                        .context(&key)
                        .into()
                }
            }
        }
        Ok(service)
    } else {
        ParserError::unreachable("parse_service_statement")
            .context(&object)
            .into()
    }
}

fn parse_operation_statement(input_pair: Pair<'_, Rule>) -> ModelResult<OperationBuilder> {
    let (id, object) = parse_id_and_object(input_pair)?;
    if let NodeValue::Object(object) = object {
        let mut operation = OperationBuilder::new(&id);
        for (key, value) in object {
            match key.as_str() {
                MEMBER_INPUT => {
                    let _ = operation.input(&value.as_string().unwrap());
                }
                MEMBER_OUTPUT => {
                    let _ = operation.output(&value.as_string().unwrap());
                }
                MEMBER_ERRORS => {
                    if let NodeValue::Array(values) = value {
                        for value in values {
                            let _ = operation.error(&value.as_string().unwrap());
                        }
                    } else {
                        return ParserError::unreachable("parse_operation_statement")
                            .context(&value)
                            .into();
                    }
                }
                _ => {
                    return ParserError::unreachable("parse_operation_statement")
                        .context(&key)
                        .into()
                }
            }
        }
        Ok(operation)
    } else {
        ParserError::unreachable("parse_operation_statement")
            .context(&object)
            .into()
    }
}

fn parse_resource_statement(input_pair: Pair<'_, Rule>) -> ModelResult<ResourceBuilder> {
    let (id, object) = parse_id_and_object(input_pair)?;
    if let NodeValue::Object(object) = object {
        let mut resource = ResourceBuilder::new(&id);
        for (key, value) in object {
            match key.as_str() {
                MEMBER_IDENTIFIERS => {
                    if let NodeValue::Object(identifiers) = value {
                        for (id, target) in identifiers {
                            let _ = resource.identifier(&id, &target.as_string().unwrap());
                        }
                    } else {
                        return ParserError::unreachable("parse_resource_statement")
                            .context(&value)
                            .into();
                    }
                }
                MEMBER_CREATE => {
                    let _ = resource.create(&value.as_string().unwrap());
                }
                MEMBER_PUT => {
                    let _ = resource.put(&value.as_string().unwrap());
                }
                MEMBER_READ => {
                    let _ = resource.read(&value.as_string().unwrap());
                }
                MEMBER_UPDATE => {
                    let _ = resource.update(&value.as_string().unwrap());
                }
                MEMBER_DELETE => {
                    let _ = resource.delete(&value.as_string().unwrap());
                }
                MEMBER_LIST => {
                    let _ = resource.list(&value.as_string().unwrap());
                }
                MEMBER_OPERATIONS => {
                    if let NodeValue::Array(values) = value {
                        for value in values {
                            let _ = resource.operation(&value.as_string().unwrap());
                        }
                    } else {
                        return ParserError::unreachable("parse_resource_statement")
                            .context(&value)
                            .into();
                    }
                }
                MEMBER_COLLECTION_OPERATIONS => {
                    if let NodeValue::Array(values) = value {
                        for value in values {
                            let _ = resource.collection_operation(&value.as_string().unwrap());
                        }
                    } else {
                        return ParserError::unreachable("parse_resource_statement")
                            .context(&value)
                            .into();
                    }
                }
                MEMBER_RESOURCES => {
                    if let NodeValue::Array(values) = value {
                        for value in values {
                            let _ = resource.resource(&value.as_string().unwrap());
                        }
                    } else {
                        return ParserError::unreachable("parse_resource_statement")
                            .context(&value)
                            .into();
                    }
                }
                _ => {
                    return ParserError::unreachable("parse_resource_statement")
                        .context(&key)
                        .into()
                }
            }
        }
        Ok(resource)
    } else {
        ParserError::unreachable("parse_resource_statement")
            .context(&object)
            .into()
    }
}

fn parse_id_and_object(input_pair: Pair<'_, Rule>) -> ModelResult<(String, NodeValue)> {
    let mut id: Option<String> = None;
    let mut node_value: Option<NodeValue> = None;
    for inner in input_pair.into_inner() {
        match inner.as_rule() {
            Rule::identifier => {
                id = Some(inner.as_str().to_string());
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

fn parse_apply_statement(input_pair: Pair<'_, Rule>) -> ModelResult<ReferenceBuilder> {
    let mut id: Option<String> = None;
    let mut a_trait: Option<TraitBuilder> = None;
    for inner in input_pair.into_inner() {
        match inner.as_rule() {
            Rule::identifier => {
                id = Some(inner.as_str().to_string());
            }
            Rule::a_trait => a_trait = Some(parse_a_trait(inner)?),
            _ => unexpected!("parse_apply_statement", inner),
        }
    }
    if let Some(id) = id {
        let mut reference = ReferenceBuilder::new(&id);
        if let Some(a_trait) = a_trait {
            let _ = reference.apply_trait(a_trait);
        }
        Ok(reference)
    } else {
        ParserError::unreachable("parse_apply_statement").into()
    }
}

fn parse_node_value(input_pair: Pair<'_, Rule>) -> ModelResult<NodeValue> {
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
        Rule::shape_id => NodeValue::String(inner.as_str().to_string()),
        Rule::text_block => parse_text_block(inner)?,
        Rule::quoted_text => parse_quoted_text(inner)?,
        _ => {
            return ParserError::unreachable("parse_node_value")
                .debug_context(&inner)
                .into()
        }
    })
}

fn parse_quoted_text(input_pair: Pair<'_, Rule>) -> ModelResult<NodeValue> {
    for inner in input_pair.into_inner() {
        return match inner.as_rule() {
            Rule::quoted_chars => Ok(NodeValue::String(inner.as_str().to_string())),
            _ => unexpected!("parse_quoted_text", inner),
        };
    }
    ParserError::unreachable("parse_quoted_text").into()
}

fn parse_text_block(input_pair: Pair<'_, Rule>) -> ModelResult<NodeValue> {
    for inner in input_pair.into_inner() {
        match inner.as_rule() {
            Rule::block_quoted_chars => return Ok(NodeValue::String(inner.as_str().to_string())),
            _ => unexpected!("parse_text_block", inner),
        }
    }
    ParserError::unreachable("parse_text_block").into()
}

fn empty_node_array() -> NodeValue {
    NodeValue::Array(Default::default())
}

fn parse_populated_node_array(input_pair: Pair<'_, Rule>) -> ModelResult<NodeValue> {
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

fn parse_populated_node_object(input_pair: Pair<'_, Rule>) -> ModelResult<NodeValue> {
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

fn parse_node_object_kvp(input_pair: Pair<'_, Rule>) -> ModelResult<(String, NodeValue)> {
    let mut key: Option<String> = None;
    let mut value: Option<NodeValue> = None;
    for inner in input_pair.into_inner() {
        match inner.as_rule() {
            Rule::identifier => {
                key = Some(inner.as_str().to_string());
            }
            Rule::quoted_text => {
                key = Some(inner.as_str().to_string());
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

fn apply_traits(shape: &mut impl ShapeTraits, doc: &[String], traits: &[TraitBuilder]) {
    if !doc.is_empty() {
        let _ = shape.documentation(&doc.join("\n"));
    }
    for a_trait in traits {
        let _ = shape.apply_trait(a_trait.clone());
    }
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
