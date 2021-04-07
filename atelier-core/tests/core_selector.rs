use atelier_core::builder::selector::{AttributeBuilder, ExpressionListBuilder};
use atelier_core::model::selector::*;
use atelier_core::model::{Identifier, ShapeID};
use std::str::FromStr;

fn selector_eq(input: Selector, expected: &str) {
    println!("{:#?}", input);
    assert_eq!(input.to_string(), expected.to_string());
}

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

#[test]
fn test_spec_shape_types_all() {
    selector_eq(ExpressionListBuilder::any_shape().into(), "*");
}

#[test]
fn test_spec_shape_types_number() {
    selector_eq(ExpressionListBuilder::any_number().into(), "number");
}

#[test]
fn test_spec_shape_types_simple_type() {
    selector_eq(
        ExpressionListBuilder::any_simple_type().into(),
        "simpleType",
    );
}

#[test]
fn test_spec_shape_types_collection() {
    selector_eq(ExpressionListBuilder::any_collection().into(), "collection");
}

#[test]
fn test_spec_shape_types_blob() {
    selector_eq(ExpressionListBuilder::blob().into(), "blob");
}

#[test]
fn test_spec_shape_types_boolean() {
    selector_eq(ExpressionListBuilder::boolean().into(), "boolean");
}

#[test]
fn test_spec_shape_types_document() {
    selector_eq(ExpressionListBuilder::document().into(), "document");
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
        ExpressionListBuilder::attribute(
            AttributeBuilder::new_trait()
                .path_segment_for_id(Identifier::from_str("deprecated").unwrap())
                .into(),
        )
        .into(),
        "[trait|deprecated]",
    );
}

#[test]
fn test_spec_selector_attr_2() {
    selector_eq(
        ExpressionListBuilder::attribute(
            AttributeBuilder::new_trait()
                .path_segment_for_id(Identifier::from_str("enum").unwrap())
                .path_segment_for_function(Identifier::from_str("values").unwrap())
                .path_segment_for_id(Identifier::from_str("tags").unwrap())
                .path_segment_for_function(Identifier::from_str("values").unwrap())
                .into(),
        )
        .into(),
        "[trait|enum|(values)|tags|(values)]",
    );
}

#[test]
fn test_spec_selector_attr_3() {
    selector_eq(
        ExpressionListBuilder::attribute(
            AttributeBuilder::new_id()
                .path_segment_for_id(Identifier::from_str("namespace").unwrap())
                .string_equal(&["smithy.example".into()], false)
                .into(),
        )
        .into(),
        "[id|namespace = \"smithy.example\"]",
    );
}

#[test]
fn test_spec_selector_attr_4() {
    selector_eq(
        ExpressionListBuilder::attribute(
            AttributeBuilder::new_trait()
                .path_segment_for_id(Identifier::from_str("since").unwrap())
                .string_equal(&[2019.into(), 2020.into()], false)
                .into(),
        )
        .into(),
        "[trait|since = 2019, 2020]",
    );
}

#[test]
fn test_spec_selector_attr_5() {
    selector_eq(
        ExpressionListBuilder::attribute(
            AttributeBuilder::new_trait()
                .path_segment_for_id(Identifier::from_str("httpError").unwrap())
                .number_greater(&[500.into()])
                .into(),
        )
        .into(),
        "[trait|httpError > 500]",
    );
}

#[test]
fn test_spec_selector_attr_6() {
    selector_eq(
        ExpressionListBuilder::attribute(
            AttributeBuilder::new_trait()
                .path_segment_for_id(Identifier::from_str("range").unwrap())
                .path_segment_for_id(Identifier::from_str("min").unwrap())
                .string_equal(&[1.into()], false)
                .into(),
        )
        .into(),
        "[trait|range|min = 1]",
    );
}

#[test]
fn test_spec_selector_attr_7() {
    selector_eq(
        ExpressionListBuilder::attribute(
            AttributeBuilder::new_trait()
                .path_segment_for_id(Identifier::from_str("documentation").unwrap())
                .path_segment_for_id(Identifier::from_str("invalid").unwrap())
                .path_segment_for_id(Identifier::from_str("child").unwrap())
                .string_equal(&[Identifier::from_str("Hi").unwrap().into()], false)
                .into(),
        )
        .into(),
        "[trait|documentation|invalid|child = Hi]",
    );
}

// ------------------------------------------------------------------------------------------------

#[test]
fn test_spec_multiple_1() {
    selector_eq(
        ExpressionListBuilder::string()
            .add_attribute(
                AttributeBuilder::new_trait()
                    .path_segment_for_id(Identifier::from_str("sensitive").unwrap())
                    .into(),
            )
            .into(),
        "string [trait|sensitive]",
    );
}

// ------------------------------------------------------------------------------------------------

#[test]
fn test_spec_id_attribute_1() {
    selector_eq(
        ExpressionListBuilder::attribute(
            AttributeBuilder::new_id()
                .string_equal(
                    &[ShapeID::from_str("foo.baz#Structure").unwrap().into()],
                    false,
                )
                .into(),
        )
        .into(),
        "[id = foo.baz#Structure]",
    );
}

#[test]
fn test_spec_id_attribute_2() {
    selector_eq(
        ExpressionListBuilder::attribute(
            AttributeBuilder::new_id()
                .string_equal(&["foo.baz#Structure$foo".into()], false)
                .into(),
        )
        .into(),
        "[id = \"foo.baz#Structure$foo\"]",
    );
}

#[test]
fn test_spec_id_attribute_4() {
    selector_eq(
        ExpressionListBuilder::attribute(
            AttributeBuilder::new_id()
                .path_segment_for_id(Identifier::from_str("name").unwrap())
                .string_equal(&[Identifier::from_str("MyShape").unwrap().into()], false)
                .into(),
        )
        .into(),
        "[id|name = MyShape]",
    );
}

#[test]
fn test_spec_id_attribute_5() {
    selector_eq(
        ExpressionListBuilder::attribute(
            AttributeBuilder::new_id()
                .path_segment_for_id(Identifier::from_str("member").unwrap())
                .string_equal(&[Identifier::from_str("foo").unwrap().into()], false)
                .into(),
        )
        .into(),
        "[id|member = foo]",
    );
}

#[test]
fn test_spec_id_attribute_6() {
    selector_eq(
        ExpressionListBuilder::attribute(
            AttributeBuilder::new_id()
                .path_segment_for_function(Identifier::from_str("length").unwrap())
                .number_greater(&[80.into()])
                .into(),
        )
        .into(),
        "[id|(length) > 80]",
    );
}

#[test]
fn test_spec_id_attribute_7() {
    selector_eq(
        ExpressionListBuilder::attribute(
            AttributeBuilder::new_id()
                .path_segment_for_id(Identifier::from_str("member").unwrap())
                .path_segment_for_function(Identifier::from_str("length").unwrap())
                .number_greater(&[20.into()])
                .into(),
        )
        .into(),
        "[id|member|(length) > 20]",
    );
}

// ------------------------------------------------------------------------------------------------

#[test]
fn test_spec_service_attribute_1() {
    selector_eq(
        ExpressionListBuilder::attribute(AttributeBuilder::new_service().into()).into(),
        "[service]",
    );
}

#[test]
fn test_spec_service_attribute_3() {
    selector_eq(
        ExpressionListBuilder::attribute(
            AttributeBuilder::new_service()
                .string_equal(
                    &[ShapeID::from_str("smithy.example#MyService")
                        .unwrap()
                        .into()],
                    false,
                )
                .into(),
        )
        .into(),
        "[service = smithy.example#MyService]",
    );
}

#[test]
fn test_spec_service_attribute_4() {
    selector_eq(
        ExpressionListBuilder::attribute(
            AttributeBuilder::new_service()
                .path_segment_for_id(Identifier::from_str("version").unwrap())
                .string_starts_with(&["2018-".into()], false)
                .into(),
        )
        .into(),
        "[service|version ^= \"2018-\"]",
    );
}

// ------------------------------------------------------------------------------------------------

#[test]
fn test_spec_trait_attribute_1() {
    selector_eq(
        ExpressionListBuilder::attribute(
            AttributeBuilder::new_trait()
                .path_segment_for_function(Identifier::from_str("keys").unwrap())
                .path_segment_for_id(Identifier::from_str("namespace").unwrap())
                .string_equal(&["smithy.example".into()], false)
                .into(),
        )
        .into(),
        "[trait|(keys)|namespace = \"smithy.example\"]",
    );
}

#[test]
fn test_spec_trait_attribute_2() {
    selector_eq(
        ExpressionListBuilder::attribute(
            AttributeBuilder::new_trait()
                .path_segment_for_function(Identifier::from_str("values").unwrap())
                .path_segment_for_id(Identifier::from_str("tags").unwrap())
                .into(),
        )
        .into(),
        "[trait|(values)|tags]",
    );
}

#[test]
fn test_spec_trait_attribute_3() {
    selector_eq(
        ExpressionListBuilder::attribute(
            AttributeBuilder::new_trait()
                .path_segment_for_function(Identifier::from_str("length").unwrap())
                .number_greater(&[10.into()])
                .into(),
        )
        .into(),
        "[trait|(length) > 10]",
    );
}

#[test]
fn test_spec_trait_attribute_4() {
    selector_eq(
        ExpressionListBuilder::attribute(
            AttributeBuilder::new_trait()
                .path_segment_for_shape(ShapeID::from_str("smithy.api#deprecated").unwrap())
                .into(),
        )
        .into(),
        "[trait|smithy.api#deprecated]",
    );
}

#[test]
fn test_spec_trait_attribute_5() {
    selector_eq(
        ExpressionListBuilder::attribute(
            AttributeBuilder::new_trait()
                .path_segment_for_id(Identifier::from_str("deprecated").unwrap())
                .into(),
        )
        .into(),
        "[trait|deprecated]",
    );
}

#[test]
fn test_spec_trait_attribute_6() {
    selector_eq(
        ExpressionListBuilder::attribute(
            AttributeBuilder::new_trait()
                .path_segment_for_id(Identifier::from_str("error").unwrap())
                .string_equal(&[Identifier::from_str("client").unwrap().into()], false)
                .into(),
        )
        .into(),
        "[trait|error = client]",
    );
}

#[test]
fn test_spec_trait_attribute_7() {
    selector_eq(
        ExpressionListBuilder::attribute(
            AttributeBuilder::new_trait()
                .path_segment_for_id(Identifier::from_str("error").unwrap())
                .string_not_equal(&[Identifier::from_str("client").unwrap().into()], false)
                .into(),
        )
        .into(),
        "[trait|error != client]",
    );
}

#[test]
fn test_spec_trait_attribute_8() {
    selector_eq(
        ExpressionListBuilder::attribute(
            AttributeBuilder::new_trait()
                .path_segment_for_id(Identifier::from_str("documentation").unwrap())
                .string_contains(
                    &[
                        Identifier::from_str("TODO").unwrap().into(),
                        Identifier::from_str("FIXME").unwrap().into(),
                    ],
                    false,
                )
                .into(),
        )
        .into(),
        "[trait|documentation *= TODO, FIXME]",
    );
}

// ------------------------------------------------------------------------------------------------

#[test]
fn test_spec_node_attribute_1() {
    selector_eq(
        ExpressionListBuilder::attribute(
            AttributeBuilder::new_trait()
                .path_segment_for_id(Identifier::from_str("externalDocumentation").unwrap())
                .path_segment_for_function(Identifier::from_str("keys").unwrap())
                .string_equal(&[Identifier::from_str("Homepage").unwrap().into()], false)
                .into(),
        )
        .into(),
        "[trait|externalDocumentation|(keys) = Homepage]",
    );
}

#[test]
fn test_spec_node_attribute_2() {
    selector_eq(
        ExpressionListBuilder::attribute(
            AttributeBuilder::new_trait()
                .path_segment_for_id(Identifier::from_str("enum").unwrap())
                .path_segment_for_function(Identifier::from_str("values").unwrap())
                .path_segment_for_id(Identifier::from_str("tags").unwrap())
                .path_segment_for_function(Identifier::from_str("values").unwrap())
                .string_equal(&[Identifier::from_str("internal").unwrap().into()], false)
                .into(),
        )
        .into(),
        "[trait|enum|(values)|tags|(values) = internal]",
    );
}

#[test]
fn test_spec_node_attribute_3() {
    selector_eq(
        ExpressionListBuilder::attribute(
            AttributeBuilder::new_trait()
                .path_segment_for_id(Identifier::from_str("documentation").unwrap())
                .path_segment_for_function(Identifier::from_str("length").unwrap())
                .number_less(&[3.into()])
                .into(),
        )
        .into(),
        "[trait|documentation|(length) < 3]",
    );
}

#[test]
fn test_spec_node_attribute_4() {
    selector_eq(
        ExpressionListBuilder::attribute(
            AttributeBuilder::new_trait()
                .path_segment_for_id(Identifier::from_str("externalDocumentation").unwrap())
                .path_segment_for_text("Reference Docs")
                .into(),
        )
        .into(),
        "[trait|externalDocumentation|\"Reference Docs\"]",
    );
}

/* ------------------------------------------------------------------------------------------------
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
------------------------------------------------------------------------------------------------ */

/* ------------------------------------------------------------------------------------------------
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
------------------------------------------------------------------------------------------------ */

// ------------------------------------------------------------------------------------------------

#[test]
fn test_spec_neighbors_1() {
    selector_eq(
        ExpressionListBuilder::map()
            .add_forward_undirected()
            .add_member()
            .into(),
        "map > member",
    );
}

#[test]
fn test_spec_neighbors_2() {
    selector_eq(
        ExpressionListBuilder::list()
            .add_forward_undirected()
            .add_member()
            .add_forward_undirected()
            .add_string()
            .into(),
        "list > member > string",
    );
}

#[test]
fn test_spec_neighbors_3() {
    selector_eq(
        ExpressionListBuilder::operation()
            .add_forward_undirected()
            .add_any_shape()
            .into(),
        "operation > *",
    );
}

#[test]
fn test_spec_neighbors_4() {
    selector_eq(
        ExpressionListBuilder::operation()
            .add_forward_directed(&[
                Identifier::from_str("input").unwrap(),
                Identifier::from_str("output").unwrap(),
            ])
            .add_structure()
            .into(),
        "operation -[input, output]-> structure",
    );
}

#[test]
fn test_spec_neighbors_5() {
    selector_eq(
        ExpressionListBuilder::service()
            .add_test_function(
                ExpressionListBuilder::forward_directed(&[Identifier::new_unchecked("trait")])
                    .add_attribute(
                        AttributeBuilder::new_trait()
                            .path_segment_for_id(Identifier::new_unchecked("protocolDefinition"))
                            .into(),
                    )
                    .into(),
            )
            .into(),
        "service :test(-[trait]-> [trait|protocolDefinition])",
    );
}

#[test]
fn test_spec_neighbors_6() {
    selector_eq(
        ExpressionListBuilder::service()
            .add_forward_recursive_directed()
            .add_operation()
            .into(),
        "service ~> operation",
    );
}

#[test]
fn test_spec_neighbors_7() {
    selector_eq(
        ExpressionListBuilder::service()
            .add_attribute(
                AttributeBuilder::new_trait()
                    .path_segment_for_shape(ShapeID::new_unchecked(
                        "aws.protocols",
                        "restJson1",
                        None,
                    ))
                    .into(),
            )
            .add_forward_recursive_directed()
            .add_operation()
            .add_not_function(ExpressionListBuilder::attribute(
                AttributeBuilder::new_trait()
                    .path_segment_for_id(Identifier::new_unchecked("http"))
                    .into(),
            ))
            .into(),
        "service [trait|aws.protocols#restJson1] ~> operation :not([trait|http])",
    );
}

#[test]
fn test_spec_neighbors_8() {
    selector_eq(
        ExpressionListBuilder::string()
            .add_test_function(
                ExpressionListBuilder::reverse_undirected()
                    .add_member()
                    .add_reverse_undirected()
                    .add_list()
                    .into(),
            )
            .into(),
        "string :test(< member < list)",
    );
}

#[test]
fn test_spec_neighbors_9() {
    selector_eq(
        ExpressionListBuilder::fn_not(ExpressionListBuilder::attribute(
            AttributeBuilder::new_trait()
                .path_segment_for_id(Identifier::new_unchecked("trait"))
                .into(),
        ))
        .add_not_function(
            ExpressionListBuilder::reverse_undirected()
                .add_any_shape()
                .into(),
        )
        .into(),
        ":not([trait|trait]) :not(< *)",
    );
}

#[test]
fn test_spec_neighbors_10() {
    selector_eq(
        ExpressionListBuilder::attribute(
            AttributeBuilder::new_trait()
                .path_segment_for_id(Identifier::new_unchecked("streaming"))
                .into(),
        )
        .add_test_function(ExpressionListBuilder::reverse_undirected())
        .add_not_function(
            ExpressionListBuilder::reverse_undirected()
                .add_member()
                .add_reverse_undirected()
                .add_structure()
                .add_reverse_directed(&[
                    Identifier::new_unchecked("input"),
                    Identifier::new_unchecked("output"),
                ])
                .add_operation()
                .into(),
        )
        .into(),
        "[trait|streaming] :test(<) :not(< member < structure <-[input, output]- operation)",
    );
}

#[test]
fn test_spec_neighbors_11() {
    selector_eq(
        ExpressionListBuilder::attribute(
            AttributeBuilder::new_trait()
                .path_segment_for_id(Identifier::new_unchecked("trait"))
                .into(),
        )
        .add_not_function(ExpressionListBuilder::reverse_directed(&[
            Identifier::new_unchecked("trait"),
        ]))
        .into(),
        "[trait|trait] :not(<-[trait]-)",
    );
}

// ------------------------------------------------------------------------------------------------

#[test]
fn test_spec_functions_1() {
    selector_eq(
        ExpressionListBuilder::list()
            .add_function_from(
                Identifier::from_str("test").unwrap(),
                ExpressionListBuilder::forward_undirected()
                    .add_member()
                    .add_forward_undirected()
                    .add_string()
                    .into(),
            )
            .into(),
        "list :test(> member > string)",
    );
}
