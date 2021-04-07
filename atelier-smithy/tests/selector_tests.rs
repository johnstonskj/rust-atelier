use atelier_core::model::selector::*;
use atelier_core::model::values::Number;
use atelier_core::model::Identifier;
use atelier_smithy::parser::parse_selector;
use std::str::FromStr;

fn selector_eq(input: &str, expected: Option<Selector>) {
    println!(">>>>> {} >>>>>", input);
    let parsed = parse_selector(input);
    assert!(parsed.is_ok());
    let parsed = parsed.unwrap();
    println!("{:#?}", parsed);
    println!("<<<<< {} <<<<<<", parsed);
    if let Some(expected) = expected {
        assert_eq!(parsed, expected);
    }
}

fn selector_fail(input: &str) {
    let parsed = parse_selector(input);
    println!("{:#?}", parsed);
    assert!(parsed.is_err());
}

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

#[test]
fn test_spec_shape_types_fail() {
    selector_fail("");
}

// ------------------------------------------------------------------------------------------------

#[test]
fn test_spec_shape_types_all() {
    selector_eq(
        "*",
        Some(SelectorExpression::ShapeType(ShapeType::Any).into()),
    );
}

#[test]
fn test_spec_shape_types_number() {
    selector_eq(
        "number",
        Some(SelectorExpression::ShapeType(ShapeType::Number).into()),
    );
}

#[test]
fn test_spec_shape_types_simple_type() {
    selector_eq(
        "simpleType",
        Some(SelectorExpression::ShapeType(ShapeType::SimpleType).into()),
    );
}

#[test]
fn test_spec_shape_types_collection() {
    selector_eq(
        "collection",
        Some(SelectorExpression::ShapeType(ShapeType::Collection).into()),
    );
}

#[test]
fn test_spec_shape_types_blob() {
    selector_eq(
        "blob",
        Some(SelectorExpression::ShapeType(ShapeType::Blob).into()),
    );
}

#[test]
fn test_spec_shape_types_boolean() {
    selector_eq(
        "boolean",
        Some(SelectorExpression::ShapeType(ShapeType::Boolean).into()),
    );
}

#[test]
fn test_spec_shape_types_document() {
    selector_eq(
        "document",
        Some(SelectorExpression::ShapeType(ShapeType::Document).into()),
    );
}

#[test]
fn test_spec_shape_types_string() {
    selector_eq(
        "string",
        Some(SelectorExpression::ShapeType(ShapeType::String).into()),
    );
}

#[test]
fn test_spec_shape_types_integer() {
    selector_eq(
        "integer",
        Some(SelectorExpression::ShapeType(ShapeType::Integer).into()),
    );
}

#[test]
fn test_spec_shape_types_byte() {
    selector_eq(
        "byte",
        Some(SelectorExpression::ShapeType(ShapeType::Byte).into()),
    );
}

#[test]
fn test_spec_shape_types_short() {
    selector_eq(
        "short",
        Some(SelectorExpression::ShapeType(ShapeType::Short).into()),
    );
}

#[test]
fn test_spec_shape_types_long() {
    selector_eq(
        "long",
        Some(SelectorExpression::ShapeType(ShapeType::Long).into()),
    );
}

#[test]
fn test_spec_shape_types_float() {
    selector_eq(
        "float",
        Some(SelectorExpression::ShapeType(ShapeType::Float).into()),
    );
}

#[test]
fn test_spec_shape_types_double() {
    selector_eq(
        "double",
        Some(SelectorExpression::ShapeType(ShapeType::Double).into()),
    );
}

#[test]
fn test_spec_shape_types_big_decimal() {
    selector_eq(
        "bigDecimal",
        Some(SelectorExpression::ShapeType(ShapeType::BigDecimal).into()),
    );
}

#[test]
fn test_spec_shape_types_big_integer() {
    selector_eq(
        "bigInteger",
        Some(SelectorExpression::ShapeType(ShapeType::BigInteger).into()),
    );
}

#[test]
fn test_spec_shape_types_timestamp() {
    selector_eq(
        "timestamp",
        Some(SelectorExpression::ShapeType(ShapeType::Timestamp).into()),
    );
}

#[test]
fn test_spec_shape_types_list() {
    selector_eq(
        "list",
        Some(SelectorExpression::ShapeType(ShapeType::List).into()),
    );
}

#[test]
fn test_spec_shape_types_set() {
    selector_eq(
        "set",
        Some(SelectorExpression::ShapeType(ShapeType::Set).into()),
    );
}

#[test]
fn test_spec_shape_types_map() {
    selector_eq(
        "map",
        Some(SelectorExpression::ShapeType(ShapeType::Map).into()),
    );
}

#[test]
fn test_spec_shape_types_structure() {
    selector_eq(
        "structure",
        Some(SelectorExpression::ShapeType(ShapeType::Structure).into()),
    );
}

#[test]
fn test_spec_shape_types_union() {
    selector_eq(
        "union",
        Some(SelectorExpression::ShapeType(ShapeType::Union).into()),
    );
}

#[test]
fn test_spec_shape_types_service() {
    selector_eq(
        "service",
        Some(SelectorExpression::ShapeType(ShapeType::Service).into()),
    );
}

#[test]
fn test_spec_shape_types_operation() {
    selector_eq(
        "operation",
        Some(SelectorExpression::ShapeType(ShapeType::Operation).into()),
    );
}

#[test]
fn test_spec_shape_types_resource() {
    selector_eq(
        "resource",
        Some(SelectorExpression::ShapeType(ShapeType::Resource).into()),
    );
}

#[test]
fn test_spec_shape_types_member() {
    selector_eq(
        "member",
        Some(SelectorExpression::ShapeType(ShapeType::Member).into()),
    );
}

// ------------------------------------------------------------------------------------------------

#[test]
fn test_spec_selector_attr_1() {
    selector_eq(
        "[trait|deprecated]",
        Some(
            SelectorExpression::AttributeSelector(AttributeSelector::new(Key::with_path(
                Identifier::from_str("trait").unwrap(),
                &[Value::RootShapeIdentifier(Identifier::from_str("deprecated").unwrap()).into()],
            )))
            .into(),
        ),
    );
}

#[test]
fn test_spec_selector_attr_2() {
    selector_eq(
        "[trait|enum|(values)|tags|(values)]",
        Some(
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
        ),
    );
}

#[test]
fn test_spec_selector_attr_3() {
    selector_eq(
        "[id|namespace = 'smithy.example']",
        Some(
            SelectorExpression::AttributeSelector(AttributeSelector::with_comparison(
                Key::with_path(
                    Identifier::from_str("id").unwrap(),
                    &[
                        Value::RootShapeIdentifier(Identifier::from_str("namespace").unwrap())
                            .into(),
                    ],
                ),
                AttributeComparison::string_equal("smithy.example".into()),
            ))
            .into(),
        ),
    );
}

#[test]
fn test_spec_selector_attr_4() {
    selector_eq(
        "[trait|since = 2019, 2020]",
        Some(
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
        ),
    );
}

#[test]
fn test_spec_selector_attr_5() {
    selector_eq("[trait|httpError > 500]", None);
}

#[test]
fn test_spec_selector_attr_6() {
    selector_eq("[trait|range|min = 1]", None);
}

#[test]
fn test_spec_selector_attr_7() {
    selector_eq("[trait|documentation|invalid|child = Hi]", None);
}

// ------------------------------------------------------------------------------------------------

#[test]
fn test_spec_multiple_1() {
    selector_eq("string [trait|sensitive]", None);
}

// ------------------------------------------------------------------------------------------------

#[test]
fn test_spec_id_attribute_1() {
    selector_eq("[id = foo.baz#Structure]", None);
}

#[test]
fn test_spec_id_attribute_2() {
    selector_eq("[id = 'foo.baz#Structure$foo']", None);
}

#[test]
fn test_spec_id_attribute_4() {
    selector_eq("[id|name = MyShape]", None);
}

#[test]
fn test_spec_id_attribute_5() {
    selector_eq("[id|member = foo]", None);
}

#[test]
fn test_spec_id_attribute_6() {
    selector_eq("[id|(length) > 80]", None);
}

#[test]
fn test_spec_id_attribute_7() {
    selector_eq("[id|member|(length) > 20]", None);
}

// ------------------------------------------------------------------------------------------------

#[test]
fn test_spec_service_attribute_1() {
    selector_eq("[service]", None);
}

#[test]
fn test_spec_service_attribute_2() {
    selector_eq("service", None);
}

#[test]
fn test_spec_service_attribute_3() {
    selector_eq("[service = smithy.example#MyService]", None);
}

#[test]
fn test_spec_service_attribute_4() {
    selector_eq("[service|version ^= '2018-']", None);
}

// ------------------------------------------------------------------------------------------------

#[test]
fn test_spec_trait_attribute_1() {
    selector_eq("[trait|(keys)|namespace = 'smithy.example']", None);
}

#[test]
fn test_spec_trait_attribute_2() {
    selector_eq("[trait|(values)|tags]", None);
}

#[test]
fn test_spec_trait_attribute_3() {
    selector_eq("[trait|(length) > 10]", None);
}

#[test]
fn test_spec_trait_attribute_4() {
    selector_eq("[trait|smithy.api#deprecated]", None);
}

#[test]
fn test_spec_trait_attribute_5() {
    selector_eq("[trait|deprecated]", None);
}

#[test]
fn test_spec_trait_attribute_6() {
    selector_eq("[trait|error = client]", None);
}

#[test]
fn test_spec_trait_attribute_7() {
    selector_eq("[trait|error != client]", None);
}

#[test]
fn test_spec_trait_attribute_8() {
    selector_eq("[trait|documentation *= TODO, FIXME]", None);
}

// ------------------------------------------------------------------------------------------------

#[test]
fn test_spec_node_attribute_1() {
    selector_eq("[trait|externalDocumentation|(keys) = Homepage]", None);
}

#[test]
fn test_spec_node_attribute_2() {
    selector_eq("[trait|enum|(values)|tags|(values) = internal]", None);
}

#[test]
fn test_spec_node_attribute_3() {
    selector_eq("[trait|documentation|(length) < 3]", None);
}

#[test]
fn test_spec_node_attribute_4() {
    selector_eq("[trait|externalDocumentation|'Reference Docs']", None);
}

// ------------------------------------------------------------------------------------------------

#[test]
fn test_spec_projection_1() {
    selector_eq(
        r##"service
[trait|smithy.example#allowedTags]
$service(*)
~>
[trait|tags]
:not([@: @{trait|tags|(values)} = @{var|service|trait|smithy.example#allowedTags|(values)}])"##,
        None,
    );
}

#[test]
fn test_spec_projection_2() {
    selector_eq(
        r##"service
[trait|smithy.example#allowedTags]
$service(*)
~>
[trait|enum]
:not([@: @{trait|enum|(values)|tags|(values)}
         = @{var|service|trait|smithy.example#allowedTags|(values)}])"##,
        None,
    );
}

#[test]
fn test_spec_projection_3() {
    selector_eq(
        r##"service
[trait|smithy.example#allowedTags]
$service(*)
~>
[trait|enum]
:not([@: @{trait|enum|(values)|tags|(values)}
         {<} @{var|service|trait|smithy.example#allowedTags|(values)}])"##,
        None,
    );
}

// ------------------------------------------------------------------------------------------------

#[test]
fn test_spec_context_values_1() {
    let attribute_selector = ScopedAttributeSelector::with_key(
        Key::with_path(
            Identifier::from_str("trait").unwrap(),
            &[KeyPathSegment::Value(
                Identifier::from_str("range").unwrap().into(),
            )],
        ),
        &[ScopedAttributeAssertion::new_case_insensitive(
            ScopedValue::ContextValue(vec![Value::RootShapeIdentifier(
                Identifier::from_str("min").unwrap(),
            )
            .into()]),
            Comparator::NumberGreaterThan,
            &[ScopedValue::ContextValue(vec![Value::RootShapeIdentifier(
                Identifier::from_str("max").unwrap(),
            )
            .into()])],
        )],
    );
    let mut selector = Selector::default();
    selector.add_expression(attribute_selector.into());
    selector_eq("[@trait|range: @{min} > @{max} i]", Some(selector));
}

#[test]
fn test_spec_context_values_2() {
    selector_eq("[trait|trait][@: @{trait|(keys)} = @{id}]", None);
}

#[test]
fn test_spec_context_values_3() {
    selector_eq(
        r##"[@trait|enum|(values):
    @{deprecated} = true &&
    @{tags|(values)} = "deprecated"]"##,
        None,
    );
}

#[test]
fn test_spec_context_values_4() {
    selector_eq(
        r##"[@trait|idRef:
    @{failWhenMissing} = true &&
    @{errorMessage} ?= false]"##,
        None,
    );
}

#[test]
fn test_spec_context_values_5() {
    selector_eq(
        r##"[@trait|httpApiKeyAuth:
    @{name} = header &&
    @{in} != 'x-api-token', 'authorization']"##,
        None,
    );
}

#[test]
fn test_spec_context_values_6() {
    selector_eq(
        r##"[@trait|httpApiKeyAuth:
    @{name} = header i &&
    @{in} != 'x-api-token', 'authorization' i]"##,
        None,
    );
}

#[test]
fn test_spec_context_values_7() {
    selector_eq(
        r##"[@trait|httpApiKeyAuth:
    @{name} = header &&
    @{in} != 'x-api-token', 'authorization' i]"##,
        None,
    );
}

// ------------------------------------------------------------------------------------------------

#[test]
fn test_spec_neighbors_1() {
    selector_eq("map > member", None);
}

#[test]
fn test_spec_neighbors_2() {
    selector_eq("list > member > string", None);
}

#[test]
fn test_spec_neighbors_3() {
    selector_eq("operation > *", None);
}

#[test]
fn test_spec_neighbors_4() {
    selector_eq("operation -[input, output]-> structure", None);
}

#[test]
fn test_spec_neighbors_5() {
    selector_eq("service :test(-[trait]-> [trait|protocolDefinition])", None);
}

#[test]
fn test_spec_neighbors_6() {
    selector_eq("service ~> operation", None);
}

#[test]
fn test_spec_neighbors_7() {
    selector_eq(
        r##"service[trait|aws.protocols#restJson1]
    ~> operation:not([trait|http])"##,
        None,
    );
}

#[test]
fn test_spec_neighbors_8() {
    selector_eq("string :test(< member < list)", None);
}

#[test]
fn test_spec_neighbors_9() {
    selector_eq(":not([trait|trait]) :not(< *)", None);
}

#[test]
fn test_spec_neighbors_10() {
    selector_eq(
        r##"[trait|streaming]
:test(<)
:not(< member < structure <-[input, output]- operation)"##,
        None,
    );
}

#[test]
fn test_spec_neighbors_11() {
    selector_eq("[trait|trait] :not(<-[trait]-)", None);
}

// ------------------------------------------------------------------------------------------------

#[test]
fn test_spec_functions_1() {
    selector_eq("list:test(> member > string)", None);
}
