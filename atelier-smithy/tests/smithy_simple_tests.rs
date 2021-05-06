use atelier_core::builder::traits::{documentation, pattern, range, range_min};
use atelier_core::builder::{ModelBuilder, ShapeTraits, SimpleShapeBuilder};
use atelier_core::model::Model;
use atelier_core::Version;
use atelier_smithy::parser::parse_model;

fn model_test(input_str: &str, expected: Model) {
    println!("input:\n{}", input_str);
    println!("------------------------------------------------------------------------------------------------");
    println!("expected:\n{:#?}", expected);
    println!("------------------------------------------------------------------------------------------------");
    match parse_model(input_str) {
        Ok(actual) => {
            println!("actual:\n{:#?}", actual);
            println!("------------------------------------------------------------------------------------------------");
            assert_eq!(actual, expected)
        }
        Err(err) => panic!("error: {:#?}", err),
    }
}

#[test]
fn empty_file() {
    model_test("", Model::default())
}

#[test]
fn namespace_only() {
    model_test(
        "namespace smithy.waiters",
        ModelBuilder::new(Version::default(), "smithy.waiters").into(),
    )
}

#[test]
fn simple_shape_only() {
    model_test(
        r##"namespace smithy.waiters
        
        integer WaiterDelay"##,
        ModelBuilder::new(Version::default(), "smithy.waiters")
            .simple_shape(SimpleShapeBuilder::integer("WaiterDelay"))
            .into(),
    )
}

#[test]
fn simple_shape_with_block_text() {
    model_test(
        r##"namespace example.foo
        @documentation("""
            A wait time for "foo" to happen
        """)
        integer FooDelay"##,
        ModelBuilder::new(Version::default(), "example.foo")
            .simple_shape(
                SimpleShapeBuilder::integer("FooDelay")
                    .apply_trait(documentation(
                        r##"A wait time for "foo" to happen
        "##,
                    ))
                    .into(),
            )
            .into(),
    )
}

#[test]
fn simple_shape_with_traits() {
    model_test(
        r##"namespace smithy.waiters
        
        @box
        @range(min: 1)
        integer WaiterDelay"##,
        ModelBuilder::new(Version::default(), "smithy.waiters")
            .simple_shape(
                SimpleShapeBuilder::integer("WaiterDelay")
                    .boxed()
                    .apply_trait(range(Some(1), None))
                    .into(),
            )
            .into(),
    )
}

#[test]
fn simple_shape_with_traits_and_comments() {
    model_test(
        r##"// start of the file
        namespace smithy.waiters // this namespace is added to all shape names
        
        @box // it's a boxed, not atomic, value
        @range(min: 1) // set the minimum value
        integer WaiterDelay"##,
        ModelBuilder::new(Version::default(), "smithy.waiters")
            .simple_shape(
                SimpleShapeBuilder::integer("WaiterDelay")
                    .boxed()
                    .apply_trait(range_min(1))
                    .into(),
            )
            .into(),
    )
}

#[test]
fn simple_shape_with_traits_and_documentation() {
    model_test(
        r##"namespace smithy.waiters
        
        /// The name, or identifier, of a waiter.
        @pattern("^[A-Z]+[A-Za-z0-9]*$")
        string WaiterName"##,
        ModelBuilder::new(Version::default(), "smithy.waiters")
            .simple_shape(
                SimpleShapeBuilder::string("WaiterName")
                    .documentation("The name, or identifier, of a waiter.")
                    .apply_trait(pattern("^[A-Z]+[A-Za-z0-9]*$"))
                    .into(),
            )
            .into(),
    )
}
