use atelier_core::error::{Error, ErrorKind, Result, ResultExt};
use atelier_core::model::shapes::{
    HasMembers, Member, Operation, Resource, Service, Shape, ShapeInner, SimpleType, Trait,
};
use atelier_core::model::values::{Key, NodeValue, Number};
use atelier_core::model::{Annotated, Identifier, Model, Namespace, ShapeID};
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
// Private Functions
// ------------------------------------------------------------------------------------------------

fn parse_idl(idl: Pair<'_, Rule>) -> Result<Model> {
    match idl.as_rule() {
        Rule::idl => {
            let mut control_data: HashMap<Key, NodeValue> = Default::default();
            let mut meta_data: HashMap<Key, NodeValue> = Default::default();
            let mut shape_section: Option<(Namespace, Vec<ShapeID>, Vec<Shape>)> = None;
            for inner in idl.into_inner() {
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
                    Rule::EOI => {}
                    _ => unreachable!("parse_model > Rule::idl > unreachable? {:#?}", inner),
                }
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
        _ => unreachable!("parse_model > unreachable! {:#?}", idl),
    }
}

fn parse_control_section(control_section: Pair<'_, Rule>) -> Result<HashMap<Key, NodeValue>> {
    Ok(Default::default())
}

fn parse_metadata_section(metadata_section: Pair<'_, Rule>) -> Result<HashMap<Key, NodeValue>> {
    Ok(Default::default())
}

fn parse_shape_section(
    shape_section: Pair<'_, Rule>,
) -> Result<(Namespace, Vec<ShapeID>, Vec<Shape>)> {
    let mut namespace: Option<Namespace> = None;
    let mut uses: Vec<ShapeID> = Default::default();
    let mut shapes: Vec<Shape> = Default::default();
    for inner in shape_section.into_inner() {
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
            _ => unreachable!("parse_shape_section > unreachable! {:#?}", inner),
        }
    }
    Ok((namespace.unwrap(), uses, shapes))
}

fn parse_namespace_statement(namespace_statement: Pair<'_, Rule>) -> Result<Namespace> {
    let namespace: Pair<'_, Rule> = namespace_statement.into_inner().next().unwrap();
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

fn parse_use_section(use_section: Pair<'_, Rule>) -> Result<Vec<ShapeID>> {
    let mut uses: Vec<ShapeID> = Default::default();
    for use_statement in use_section.into_inner() {
        match use_statement.as_rule() {
            Rule::use_statement => {
                let absolute_root_shape_id: Pair<'_, Rule> =
                    use_statement.into_inner().next().unwrap();
                if let Rule::absolute_root_shape_id = absolute_root_shape_id.as_rule() {
                    uses.push(ShapeID::from_str(absolute_root_shape_id.as_str())?);
                } else {
                    unreachable!("parse_uses > unreachable! {:#?}", absolute_root_shape_id)
                }
            }
            _ => unreachable!("parse_uses > unreachable! {:#?}", use_statement),
        }
    }
    Ok(uses)
}

fn parse_shape_statements(shape_statements: Pair<'_, Rule>) -> Result<Vec<Shape>> {
    let mut shapes: Vec<Shape> = Default::default();
    for shape_statement in shape_statements.into_inner() {
        match shape_statement.as_rule() {
            Rule::shape_statement => {
                shapes.push(parse_shape_statement(shape_statement)?);
            }
            _ => unreachable!(
                "parse_shape_statements > unreachable! {:#?}",
                shape_statement
            ),
        }
    }
    Ok(shapes)
}

fn parse_shape_statement(shape_statement: Pair<'_, Rule>) -> Result<Shape> {
    let mut documentation: Vec<String> = Default::default();
    let mut traits: Vec<Trait> = Default::default();
    let mut inner: Option<(Identifier, ShapeInner)> = None;
    let mut applies: Option<(ShapeID, Trait)> = None;
    for shape_statement in shape_statement.into_inner() {
        match shape_statement.as_rule() {
            Rule::shape_documentation_comments => {
                documentation.push(parse_shape_documentation_comments(shape_statement)?);
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
            _ => unreachable!(
                "parse_shape_statements > unreachable! {:#?}",
                shape_statement
            ),
        }
    }
    if let Some((id, inner)) = inner {
        let mut shape = Shape::new(id, inner);
        shape.documentation(&documentation.join("\n"));
        for a_trait in traits {
            shape.add_trait(a_trait);
        }
        Ok(shape)
    // } else if let Some((id, a_trait)) = applies {
    //     let inner = ShapeInner::Apply;
    //     let mut shape = Shape::new(id, inner);
    //     shape.add_trait(a_trait);
    //     Ok(shape)
    } else {
        Err(ErrorKind::Deserialization(
            "Smithy".to_string(),
            "parser::parse_shape_statement".to_string(),
            None,
        )
        .into())
    }
}

fn parse_shape_documentation_comments(
    shape_documentation_comments: Pair<'_, Rule>,
) -> Result<String> {
    if shape_documentation_comments.as_str().is_empty() {
        Ok(String::new())
    } else {
        let documentation_text: Pair<'_, Rule> =
            shape_documentation_comments.into_inner().next().unwrap();

        if let Rule::documentation_text = documentation_text.as_rule() {
            Ok(documentation_text.as_str().to_string())
        } else {
            Err(ErrorKind::Deserialization(
                "Smithy".to_string(),
                "parser::parse_shape_documentation_comments".to_string(),
                Some(documentation_text.to_string()),
            )
            .into())
        }
    }
}

fn parse_trait_statements(trait_statements: Pair<'_, Rule>) -> Result<Vec<Trait>> {
    let mut traits: Vec<Trait> = Default::default();
    for a_trait in trait_statements.into_inner() {
        match a_trait.as_rule() {
            Rule::a_trait => {
                traits.push(parse_a_trait(a_trait)?);
            }
            _ => unreachable!("parse_trait_statements > unreachable! {:#?}", a_trait),
        }
    }
    Ok(traits)
}

fn parse_a_trait(a_trait: Pair<'_, Rule>) -> Result<Trait> {
    let mut id: Option<ShapeID> = None;
    let mut node_value: Option<NodeValue> = None;
    for inner in a_trait.into_inner() {
        match inner.as_rule() {
            Rule::shape_id => {
                id = Some(ShapeID::from_str(inner.as_str())?);
            }
            Rule::node_value => {
                node_value = Some(parse_node_value(inner)?);
            }
            Rule::trait_structure_kvp => {}
            _ => unreachable!("parse_a_trait > unreachable! {:#?}", inner),
        }
    }
    Ok(match (id, node_value) {
        (Some(id), None) => Trait::new(id),
        (Some(id), Some(node_value)) => Trait::with_value(id, node_value),
        _ => unreachable!("parse_a_trait > unreachable!"),
    })
}

fn parse_simple_shape_statement(statement: Pair<'_, Rule>) -> Result<(Identifier, ShapeInner)> {
    let mut id: Option<Identifier> = None;
    let mut simple_type: Option<SimpleType> = None;
    for inner in statement.into_inner() {
        match inner.as_rule() {
            Rule::identifier => {
                id = Some(Identifier::from_str(inner.as_str())?);
            }
            Rule::type_blob => simple_type = Some(SimpleType::Blob),
            Rule::type_boolean => simple_type = Some(SimpleType::Boolean),
            Rule::type_document => simple_type = Some(SimpleType::Document),
            Rule::type_string => simple_type = Some(SimpleType::String),
            Rule::type_byte => simple_type = Some(SimpleType::Byte),
            Rule::type_short => simple_type = Some(SimpleType::Short),
            Rule::type_integer => simple_type = Some(SimpleType::Integer),
            Rule::type_long => simple_type = Some(SimpleType::Long),
            Rule::type_float => simple_type = Some(SimpleType::Float),
            Rule::type_double => simple_type = Some(SimpleType::Double),
            Rule::type_big_integer => simple_type = Some(SimpleType::BigInteger),
            Rule::type_big_decimal => simple_type = Some(SimpleType::BigDecimal),
            Rule::type_timestamp => simple_type = Some(SimpleType::Timestamp),
            _ => unreachable!("parse_simple_shape_statement > unreachable! {:#?}", inner),
        }
    }
    Ok(match (id, simple_type) {
        (Some(id), Some(simple_type)) => (id, ShapeInner::SimpleType(simple_type)),
        _ => unreachable!("parse_simple_shape_statement > unreachable!"),
    })
}

fn parse_list_statement(statement: Pair<'_, Rule>) -> Result<(Identifier, ShapeInner)> {
    unimplemented!("parse_list_statement")
}

fn parse_set_statement(statement: Pair<'_, Rule>) -> Result<(Identifier, ShapeInner)> {
    unimplemented!("parse_set_statement")
}

fn parse_map_statement(statement: Pair<'_, Rule>) -> Result<(Identifier, ShapeInner)> {
    unimplemented!("parse_map_statement")
}

fn parse_structure_statement(statement: Pair<'_, Rule>) -> Result<(Identifier, ShapeInner)> {
    unimplemented!("parse_structure_statement")
}

fn parse_union_statement(statement: Pair<'_, Rule>) -> Result<(Identifier, ShapeInner)> {
    unimplemented!("parse_union_statement")
}

fn parse_service_statement(statement: Pair<'_, Rule>) -> Result<(Identifier, ShapeInner)> {
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
                _ => unreachable!("parse_service_statement > unreachable! {:#?}", key),
            }
        }
        Ok((id, ShapeInner::Service(service)))
    } else {
        unreachable!("parse_service_statement > unreachable! {:#?}", object)
    }
}

fn parse_operation_statement(statement: Pair<'_, Rule>) -> Result<(Identifier, ShapeInner)> {
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
                _ => unreachable!("parse_operation_statement > unreachable! {:#?}", key),
            }
        }
        Ok((id, ShapeInner::Operation(operation)))
    } else {
        unreachable!("parse_operation_statement > unreachable! {:#?}", object)
    }
}

fn parse_resource_statement(statement: Pair<'_, Rule>) -> Result<(Identifier, ShapeInner)> {
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
                _ => unreachable!("parse_resource_statement > unreachable! {:#?}", key),
            }
        }
        Ok((id, ShapeInner::Resource(resource)))
    } else {
        unreachable!("parse_resource_statement > unreachable! {:#?}", object)
    }
}

fn parse_id_and_object(statement: Pair<'_, Rule>) -> Result<(Identifier, NodeValue)> {
    let mut id: Option<Identifier> = None;
    let mut node_value: Option<NodeValue> = None;
    for inner in statement.into_inner() {
        match inner.as_rule() {
            Rule::identifier => {
                id = Some(Identifier::from_str(inner.as_str())?);
            }
            Rule::empty_node_object => node_value = Some(empty_node_object()),
            Rule::populated_node_object => node_value = Some(parse_populated_node_object(inner)?),
            _ => unreachable!("parse_id_and_object > unreachable! {:#?}", inner),
        }
    }
    Ok(match (id, node_value) {
        (Some(id), Some(node_value)) => (id, node_value),
        _ => unreachable!("parse_id_and_object > unreachable!"),
    })
}

fn parse_apply_statement(statement: Pair<'_, Rule>) -> Result<(ShapeID, Trait)> {
    let mut id: Option<ShapeID> = None;
    let mut a_trait: Option<Trait> = None;
    for inner in statement.into_inner() {
        match inner.as_rule() {
            Rule::identifier => {
                id = Some(ShapeID::from_str(inner.as_str())?);
            }
            Rule::a_trait => a_trait = Some(parse_a_trait(inner)?),
            _ => unreachable!("parse_apply_statement > unreachable! {:#?}", inner),
        }
    }
    Ok(match (id, a_trait) {
        (Some(id), Some(a_trait)) => (id, a_trait),
        _ => unreachable!("parse_apply_statement > unreachable!"),
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
        match inner.as_rule() {
            Rule::node_value => array.push(parse_node_value(inner)?),
            _ => unreachable!("parse_populated_node_object > unreachable! {:#?}", inner),
        }
    }
    Ok(NodeValue::Array(array))
}

fn empty_node_object() -> NodeValue {
    NodeValue::Object(Default::default())
}

fn parse_populated_node_object(populated_node_object: Pair<'_, Rule>) -> Result<NodeValue> {
    let mut object: HashMap<Key, NodeValue> = Default::default();
    for inner in populated_node_object.into_inner() {
        match inner.as_rule() {
            Rule::node_object_kvp => {
                let (key, value) = parse_node_object_kvp(inner)?;
                let _ = object.insert(key, value);
            }
            _ => unreachable!("parse_populated_node_object > unreachable! {:#?}", inner),
        }
    }
    Ok(NodeValue::Object(object))
}

fn parse_node_object_kvp(node_object_kvp: Pair<'_, Rule>) -> Result<(Key, NodeValue)> {
    let mut key: Option<Key> = None;
    let mut value: Option<NodeValue> = None;
    for inner in node_object_kvp.into_inner() {
        match inner.as_rule() {
            Rule::identifier => {
                key = Some(Key::Identifier(Identifier::from_str(inner.as_str())?));
            }
            Rule::quoted_text => {
                key = Some(Key::String(inner.as_str().to_string()));
            }
            Rule::node_value => value = Some(parse_node_value(inner)?),
            _ => unreachable!("parse_node_object_kvp > unreachable! {:#?}", inner),
        }
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
