/*!
Provides simple models as basic tests.
*/

use crate::TestCaseModel;
use atelier_core::builder::traits::{documentation, pattern, range, range_min};
use atelier_core::builder::{
    ModelBuilder, OperationBuilder, ServiceBuilder, ShapeTraits, SimpleShapeBuilder,
};
use atelier_core::Version;
use std::convert::TryInto;

// ------------------------------------------------------------------------------------------------

const NAMESPACE_ONLY: &[&str] = &[];

///
/// Example model with no shapes.
///
/// ```smithy
/// namespace smithy.waiters
/// ```
///
pub fn namespace_only() -> TestCaseModel {
    TestCaseModel {
        model: ModelBuilder::new(Version::default(), "smithy.waiters")
            .try_into()
            .unwrap(),
        expected_lines: NAMESPACE_ONLY.to_vec(),
    }
}

// ------------------------------------------------------------------------------------------------

const SIMPLE_SHAPE_ONLY: &[&str] = &["integer::smithy.waiters#WaiterDelay"];

///
/// Example model with a single simple shape.
///
/// ```smithy
/// namespace smithy.waiters
///
/// integer WaiterDelay
/// ```
///
pub fn simple_shape_only() -> TestCaseModel {
    TestCaseModel {
        model: ModelBuilder::new(Version::default(), "smithy.waiters")
            .simple_shape(SimpleShapeBuilder::integer("WaiterDelay"))
            .try_into()
            .unwrap(),
        expected_lines: SIMPLE_SHAPE_ONLY.to_vec(),
    }
}

// ------------------------------------------------------------------------------------------------

const SIMPLE_SHAPE_WITH_BLOCK_TEXT: &[&str] = &[
    "integer::example.foo#FooDelay",
    "integer::example.foo#FooDelay::trait::smithy.api#documentation<=\"A wait time for \"foo\" to happen\n        \"",
];

///
/// Example model with a simple shape that has multi-line documentation.
///
/// ```smithy
/// namespace example.foo
///
/// @documentation("""
///    A wait time for "foo" to happen
/// """)
/// integer FooDelay
/// ```
///
pub fn simple_shape_with_block_text() -> TestCaseModel {
    TestCaseModel {
        model: ModelBuilder::new(Version::default(), "example.foo")
            .simple_shape(
                SimpleShapeBuilder::integer("FooDelay")
                    .apply_trait(documentation(
                        r##"A wait time for "foo" to happen
        "##,
                    ))
                    .into(),
            )
            .try_into()
            .unwrap(),
        expected_lines: SIMPLE_SHAPE_WITH_BLOCK_TEXT.to_vec(),
    }
}

// ------------------------------------------------------------------------------------------------

const SIMPLE_SHAPE_WITH_BLOCK_TEXT_2: &[&str] = &[
    "integer::example.foo#FooDelay",
    "integer::example.foo#FooDelay::trait::smithy.api#documentation<=\"Do empty \"\" quotes work too?\"",
];

///
/// Example model with a simple shape that has documentation containing double quotes.
///
/// ```smithy
/// namespace example.foo
///
/// @documentation("""Do empty "" quotes work too?""")
/// integer FooDelay
/// ```
///
pub fn simple_shape_with_block_text_2() -> TestCaseModel {
    TestCaseModel {
        model: ModelBuilder::new(Version::default(), "example.foo")
            .simple_shape(
                SimpleShapeBuilder::integer("FooDelay")
                    .apply_trait(documentation(r##"Do empty "" quotes work too?"##))
                    .into(),
            )
            .try_into()
            .unwrap(),
        expected_lines: SIMPLE_SHAPE_WITH_BLOCK_TEXT_2.to_vec(),
    }
}

// ------------------------------------------------------------------------------------------------

const SIMPLE_SHAPE_WITH_TRAITS: &[&str] = &[
    "integer::smithy.waiters#WaiterDelay",
    "integer::smithy.waiters#WaiterDelay::trait::smithy.api#box<={}",
    "integer::smithy.waiters#WaiterDelay::trait::smithy.api#range<={min}=1",
];

///
/// Example model with a simple shape with constraint traits.
///
/// ```smithy
/// namespace smithy.waiters
///
/// @box
/// @range(min: 1)
/// integer WaiterDelay
/// ```
///
pub fn simple_shape_with_traits() -> TestCaseModel {
    TestCaseModel {
        model: ModelBuilder::new(Version::default(), "smithy.waiters")
            .simple_shape(
                SimpleShapeBuilder::integer("WaiterDelay")
                    .boxed()
                    .apply_trait(range(Some(1), None))
                    .into(),
            )
            .try_into()
            .unwrap(),
        expected_lines: SIMPLE_SHAPE_WITH_TRAITS.to_vec(),
    }
}

// ------------------------------------------------------------------------------------------------

const SIMPLE_SHAPE_WITH_TRAITS_AND_COMMENTS: &[&str] = &[
    "integer::smithy.waiters#WaiterDelay",
    "integer::smithy.waiters#WaiterDelay::trait::smithy.api#box<={}",
    "integer::smithy.waiters#WaiterDelay::trait::smithy.api#range<={min}=1",
];

///
/// Example model with a simple shape with constraint traits and a lot of comments.
///
/// ```smithy
/// // start of the file
/// namespace smithy.waiters // this namespace is added to all shape names
///
/// @box // it's a boxed, not atomic, value
/// @range(min: 1) // set the minimum value
/// integer WaiterDelay
/// ```
///
pub fn simple_shape_with_traits_and_comments() -> TestCaseModel {
    TestCaseModel {
        model: ModelBuilder::new(Version::default(), "smithy.waiters")
            .simple_shape(
                SimpleShapeBuilder::integer("WaiterDelay")
                    .boxed()
                    .apply_trait(range_min(1))
                    .into(),
            )
            .try_into()
            .unwrap(),
        expected_lines: SIMPLE_SHAPE_WITH_TRAITS_AND_COMMENTS.to_vec(),
    }
}

// ------------------------------------------------------------------------------------------------

const SIMPLE_SHAPE_WITH_TRAITS_AND_DOCUMENTATION: &[&str] = &[
    "string::smithy.waiters#WaiterName",
    "string::smithy.waiters#WaiterName::trait::smithy.api#documentation<=\"The name, or identifier, of a waiter.\"",
    "string::smithy.waiters#WaiterName::trait::smithy.api#pattern<=\"^[A-Z]+[A-Za-z0-9]*$\"",
];

///
/// Example model with a simple shape and a doc-comment.
///
/// ```smithy
/// namespace smithy.waiters
///
/// /// The name, or identifier, of a waiter.
/// @pattern("^[A-Z]+[A-Za-z0-9]*$")
/// string WaiterName
/// ```
///
pub fn simple_shape_with_traits_and_documentation() -> TestCaseModel {
    TestCaseModel {
        model: ModelBuilder::new(Version::default(), "smithy.waiters")
            .simple_shape(
                SimpleShapeBuilder::string("WaiterName")
                    .documentation("The name, or identifier, of a waiter.")
                    .apply_trait(pattern("^[A-Z]+[A-Za-z0-9]*$"))
                    .into(),
            )
            .try_into()
            .unwrap(),
        expected_lines: SIMPLE_SHAPE_WITH_TRAITS_AND_DOCUMENTATION.to_vec(),
    }
}

// ------------------------------------------------------------------------------------------------

const SERVICE_WITH_RENAMES: &[&str] = &[
    "operation::smithy.example#GetSomething",
    "service::smithy.example#MyService",
    "service::smithy.example#MyService::operation=>smithy.example#GetSomething",
    "service::smithy.example#MyService::rename::foo.example#Widget<=FooWidget",
    "service::smithy.example#MyService::version<=\"2017-02-11\"",
];

///
/// Example model with a service shape.
///
/// ```smithy
/// namespace smithy.example
///
/// operation GetSomething {
/// }
///
/// service MyService {
///     version: "2017-02-11",
///     operations: [GetSomething],
///     rename: {
///         "foo.example#Widget": "FooWidget"
///     }
/// }
/// ```
///
pub fn service_with_renames() -> TestCaseModel {
    TestCaseModel {
        model: ModelBuilder::new(Version::default(), "smithy.example")
            .operation(OperationBuilder::new("GetSomething"))
            .service(
                ServiceBuilder::new("MyService", "2017-02-11")
                    .operation("GetSomething")
                    .rename("foo.example#Widget", "FooWidget")
                    .into(),
            )
            .try_into()
            .unwrap(),
        expected_lines: SERVICE_WITH_RENAMES.to_vec(),
    }
}
