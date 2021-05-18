use atelier_core::model::Model;
use atelier_smithy::SmithyReader;
use atelier_test::{parse_and_compare_model, parts as model_part};

fn model_test(input_str: &str, expected: Model) {
    parse_and_compare_model(input_str, &mut SmithyReader::default(), expected);
}

#[test]
fn empty_file() {
    model_test("", Model::default())
}

#[test]
fn namespace_only() {
    model_test(
        "namespace smithy.waiters",
        model_part::namespace_only().model,
    )
}

#[test]
fn simple_shape_only() {
    model_test(
        r##"namespace smithy.waiters
        
        integer WaiterDelay"##,
        model_part::simple_shape_only().model,
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
        model_part::simple_shape_with_block_text().model,
    )
}

#[test]
fn simple_shape_with_block_text_2() {
    model_test(
        r##"namespace example.foo
        @documentation("""Do empty "" quotes work too?""")
        integer FooDelay"##,
        model_part::simple_shape_with_block_text_2().model,
    )
}

#[test]
fn simple_shape_with_traits() {
    model_test(
        r##"namespace smithy.waiters
        
        @box
        @range(min: 1)
        integer WaiterDelay"##,
        model_part::simple_shape_with_traits().model,
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
        model_part::simple_shape_with_traits_and_comments().model,
    )
}

#[test]
fn simple_shape_with_traits_and_documentation() {
    model_test(
        r##"namespace smithy.waiters
        
        /// The name, or identifier, of a waiter.
        @pattern("^[A-Z]+[A-Za-z0-9]*$")
        string WaiterName"##,
        model_part::simple_shape_with_traits_and_documentation().model,
    )
}
