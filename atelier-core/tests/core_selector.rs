use atelier_core::model::selector::*;
use atelier_core::model::values::Number;
use atelier_core::model::Identifier;
use std::str::FromStr;

fn selector_eq(input: Selector, expected: &str) {
    assert_eq!(input.to_string(), expected.to_string());
}

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

#[test]
fn test_spec_shape_types_all() {
    selector_eq(SelectorExpression::ShapeType(ShapeType::All).into(), "*");
}

#[test]
fn test_spec_shape_types_number() {
    selector_eq(
        SelectorExpression::ShapeType(ShapeType::Number).into(),
        "number",
    );
}

#[test]
fn test_spec_shape_types_simple_type() {
    selector_eq(
        SelectorExpression::ShapeType(ShapeType::SimpleType).into(),
        "simpleType",
    );
}

#[test]
fn test_spec_shape_types_collection() {
    selector_eq(
        SelectorExpression::ShapeType(ShapeType::Collection).into(),
        "collection",
    );
}

#[test]
fn test_spec_shape_types_blob() {
    selector_eq(
        SelectorExpression::ShapeType(ShapeType::Blob).into(),
        "blob",
    );
}

#[test]
fn test_spec_shape_types_boolean() {
    selector_eq(
        SelectorExpression::ShapeType(ShapeType::Boolean).into(),
        "boolean",
    );
}

#[test]
fn test_spec_shape_types_document() {
    selector_eq(
        SelectorExpression::ShapeType(ShapeType::Document).into(),
        "document",
    );
}

#[test]
fn test_spec_shape_types_string() {
    selector_eq(
        SelectorExpression::ShapeType(ShapeType::String).into(),
        "string",
    );
}

#[test]
fn test_spec_shape_types_integer() {
    selector_eq(
        SelectorExpression::ShapeType(ShapeType::Integer).into(),
        "integer",
    );
}

#[test]
fn test_spec_shape_types_byte() {
    selector_eq(
        SelectorExpression::ShapeType(ShapeType::Byte).into(),
        "byte",
    );
}

#[test]
fn test_spec_shape_types_short() {
    selector_eq(
        SelectorExpression::ShapeType(ShapeType::Short).into(),
        "short",
    );
}

#[test]
fn test_spec_shape_types_long() {
    selector_eq(
        SelectorExpression::ShapeType(ShapeType::Long).into(),
        "long",
    );
}

#[test]
fn test_spec_shape_types_float() {
    selector_eq(
        SelectorExpression::ShapeType(ShapeType::Float).into(),
        "float",
    );
}

#[test]
fn test_spec_shape_types_double() {
    selector_eq(
        SelectorExpression::ShapeType(ShapeType::Double).into(),
        "double",
    );
}

#[test]
fn test_spec_shape_types_big_decimal() {
    selector_eq(
        SelectorExpression::ShapeType(ShapeType::BigDecimal).into(),
        "bigDecimal",
    );
}

#[test]
fn test_spec_shape_types_big_integer() {
    selector_eq(
        SelectorExpression::ShapeType(ShapeType::BigInteger).into(),
        "bigInteger",
    );
}

#[test]
fn test_spec_shape_types_timestamp() {
    selector_eq(
        SelectorExpression::ShapeType(ShapeType::Timestamp).into(),
        "timestamp",
    );
}

#[test]
fn test_spec_shape_types_list() {
    selector_eq(
        SelectorExpression::ShapeType(ShapeType::List).into(),
        "list",
    );
}

#[test]
fn test_spec_shape_types_set() {
    selector_eq(SelectorExpression::ShapeType(ShapeType::Set).into(), "set");
}

#[test]
fn test_spec_shape_types_map() {
    selector_eq(SelectorExpression::ShapeType(ShapeType::Map).into(), "map");
}

#[test]
fn test_spec_shape_types_structure() {
    selector_eq(
        SelectorExpression::ShapeType(ShapeType::Structure).into(),
        "structure",
    );
}

#[test]
fn test_spec_shape_types_union() {
    selector_eq(
        SelectorExpression::ShapeType(ShapeType::Union).into(),
        "union",
    );
}

#[test]
fn test_spec_shape_types_service() {
    selector_eq(
        SelectorExpression::ShapeType(ShapeType::Service).into(),
        "service",
    );
}

#[test]
fn test_spec_shape_types_operation() {
    selector_eq(
        SelectorExpression::ShapeType(ShapeType::Operation).into(),
        "operation",
    );
}

#[test]
fn test_spec_shape_types_resource() {
    selector_eq(
        SelectorExpression::ShapeType(ShapeType::Resource).into(),
        "resource",
    );
}

#[test]
fn test_spec_shape_types_member() {
    selector_eq(
        SelectorExpression::ShapeType(ShapeType::Member).into(),
        "member",
    );
}

// ------------------------------------------------------------------------------------------------

#[test]
fn test_spec_selector_attr_1() {
    selector_eq(
        SelectorExpression::AttributeSelector(AttributeSelector::new(Key::with_path(
            Identifier::from_str("trait").unwrap(),
            &[Value::RootShapeIdentifier(Identifier::from_str("deprecated").unwrap()).into()],
        )))
        .into(),
        "[trait|deprecated]",
    );
}

#[test]
fn test_spec_selector_attr_2() {
    selector_eq(
        SelectorExpression::AttributeSelector(AttributeSelector::new(Key::with_path(
            Identifier::from_str("trait").unwrap(),
            &[
                Value::RootShapeIdentifier(Identifier::from_str("enum").unwrap()).into(),
                Identifier::from_str("values").unwrap().into(),
                Value::RootShapeIdentifier(Identifier::from_str("tags").unwrap()).into(),
                Identifier::from_str("values").unwrap().into(),
            ],
        )))
        .into(),
        "[trait|enum|(values)|tags|(values)]",
    );
}

#[test]
fn test_spec_selector_attr_3() {
    selector_eq(
        SelectorExpression::AttributeSelector(AttributeSelector::with_comparison(
            Key::with_path(
                Identifier::from_str("id").unwrap(),
                &[Value::RootShapeIdentifier(Identifier::from_str("namespace").unwrap()).into()],
            ),
            AttributeComparison::string_equal("smithy.example".into()),
        ))
        .into(),
        "[id|namespace = \"smithy.example\"]",
    );
}

#[test]
fn test_spec_selector_attr_4() {
    selector_eq(
        SelectorExpression::AttributeSelector(AttributeSelector::with_comparison(
            Key::with_path(
                Identifier::from_str("trait").unwrap(),
                &[Value::RootShapeIdentifier(Identifier::from_str("since").unwrap()).into()],
            ),
            AttributeComparison::new(
                Comparator::StringEqual,
                &[
                    Value::Number(Number::Integer(2019)),
                    Value::Number(Number::Integer(2020)),
                ],
            ),
        ))
        .into(),
        "[trait|since = 2019, 2020]",
    );
}
