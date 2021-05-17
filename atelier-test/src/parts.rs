use atelier_core::builder::traits::{documentation, pattern, range, range_min};
use atelier_core::builder::{ModelBuilder, ShapeTraits, SimpleShapeBuilder};
use atelier_core::model::Model;
use atelier_core::Version;
use std::convert::TryInto;

///
/// ```smithy
/// namespace smithy.waiters
/// ```
///
pub fn namespace_only() -> Model {
    ModelBuilder::new(Version::default(), "smithy.waiters")
        .try_into()
        .unwrap()
}

///
/// ```smithy
/// namespace smithy.waiters
///
/// integer WaiterDelay
/// ```
///
pub fn simple_shape_only() -> Model {
    ModelBuilder::new(Version::default(), "smithy.waiters")
        .simple_shape(SimpleShapeBuilder::integer("WaiterDelay"))
        .try_into()
        .unwrap()
}

///
/// ```smithy
/// namespace example.foo
/// @documentation("""
///    A wait time for "foo" to happen
/// """)
/// integer FooDelay
/// ```
///
pub fn simple_shape_with_block_text() -> Model {
    ModelBuilder::new(Version::default(), "example.foo")
        .simple_shape(
            SimpleShapeBuilder::integer("FooDelay")
                .apply_trait(documentation(
                    r##"A wait time for "foo" to happen
        "##,
                ))
                .into(),
        )
        .try_into()
        .unwrap()
}

///
/// ```smithy
/// namespace example.foo
/// @documentation("""Do empty "" quotes work too?""")
/// integer FooDelay
/// ```
///
pub fn simple_shape_with_block_text_2() -> Model {
    ModelBuilder::new(Version::default(), "example.foo")
        .simple_shape(
            SimpleShapeBuilder::integer("FooDelay")
                .apply_trait(documentation(r##"Do empty "" quotes work too?"##))
                .into(),
        )
        .try_into()
        .unwrap()
}

///
/// ```smithy
/// namespace smithy.waiters
///
/// @box
/// @range(min: 1)
/// integer WaiterDelay
/// ```
///
pub fn simple_shape_with_traits() -> Model {
    ModelBuilder::new(Version::default(), "smithy.waiters")
        .simple_shape(
            SimpleShapeBuilder::integer("WaiterDelay")
                .boxed()
                .apply_trait(range(Some(1), None))
                .into(),
        )
        .try_into()
        .unwrap()
}

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
pub fn simple_shape_with_traits_and_comments() -> Model {
    ModelBuilder::new(Version::default(), "smithy.waiters")
        .simple_shape(
            SimpleShapeBuilder::integer("WaiterDelay")
                .boxed()
                .apply_trait(range_min(1))
                .into(),
        )
        .try_into()
        .unwrap()
}

///
/// ```smithy
/// namespace smithy.waiters
///
/// /// The name, or identifier, of a waiter.
/// @pattern("^[A-Z]+[A-Za-z0-9]*$")
/// string WaiterName
/// ```
///
pub fn simple_shape_with_traits_and_documentation() -> Model {
    ModelBuilder::new(Version::default(), "smithy.waiters")
        .simple_shape(
            SimpleShapeBuilder::string("WaiterName")
                .documentation("The name, or identifier, of a waiter.")
                .apply_trait(pattern("^[A-Z]+[A-Za-z0-9]*$"))
                .into(),
        )
        .try_into()
        .unwrap()
}
