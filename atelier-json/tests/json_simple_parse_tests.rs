use atelier_core::model::Model;
use atelier_json::JsonReader;
use atelier_test::{parse_and_compare_model, parts as model_part};

fn model_test(input_str: &str, expected: Model) {
    parse_and_compare_model(input_str, &mut JsonReader::default(), expected);
}

#[test]
fn empty_file() {
    model_test(r##"{"smithy": "1"}"##, Model::default())
}

#[test]
fn namespace_only() {
    model_test(r##"{"smithy": "1"}"##, model_part::namespace_only().model)
}

#[test]
fn simple_shape_only() {
    model_test(
        r##"{
    "smithy": "1",
    "shapes": {
        "smithy.waiters#WaiterDelay": {
            "type": "integer"
        }
    }    
}"##,
        model_part::simple_shape_only().model,
    )
}

#[test]
fn simple_shape_with_block_text() {
    model_test(
        r##"{
    "smithy": "1",
    "shapes": {
        "example.foo#FooDelay": {
            "type": "integer",
            "traits": {
                "smithy.api#documentation": "A wait time for \"foo\" to happen\n        "
            }
        }
    }    
}"##,
        model_part::simple_shape_with_block_text().model,
    )
}

#[test]
fn simple_shape_with_block_text_2() {
    model_test(
        r##"{
    "smithy": "1",
    "shapes": {
        "example.foo#FooDelay": {
            "type": "integer",
            "traits": {
                "smithy.api#documentation": "Do empty \"\" quotes work too?"
            }
        }
    }    
}"##,
        model_part::simple_shape_with_block_text_2().model,
    )
}

#[test]
fn simple_shape_with_traits() {
    model_test(
        r##"{
    "smithy": "1",
    "shapes": {
        "smithy.waiters#WaiterDelay": {
            "type": "integer",
            "traits": {
                "smithy.api#box": {},
                "smithy.api#range": {
                    "min": 1
                }
            }
        }
    }    
}"##,
        model_part::simple_shape_with_traits().model,
    )
}

#[test]
fn simple_shape_with_traits_and_comments() {
    model_test(
        r##"{
    "smithy": "1",
    "shapes": {
        "smithy.waiters#WaiterDelay": {
            "type": "integer",
            "traits": {
                "smithy.api#box": {},
                "smithy.api#range": {
                    "min": 1
                }
            }
        }
    }    
}"##,
        model_part::simple_shape_with_traits_and_comments().model,
    )
}

#[test]
fn simple_shape_with_traits_and_documentation() {
    model_test(
        // r##"namespace smithy.waiters
        //
        // /// The name, or identifier, of a waiter.
        // @pattern("^[A-Z]+[A-Za-z0-9]*$")
        // string WaiterName"##,
        r##"{
    "smithy": "1",
    "shapes": {
        "smithy.waiters#WaiterName": {
            "type": "string",
            "traits": {
                "smithy.api#documentation": "The name, or identifier, of a waiter.",
                "smithy.api#pattern": "^[A-Z]+[A-Za-z0-9]*$"
            }
        }
    }    
}"##,
        model_part::simple_shape_with_traits_and_documentation().model,
    )
}

#[test]
fn service_with_renames() {
    model_test(
        r##"{
    "smithy": "1.0",
    "shapes": {
        "smithy.example#GetSomething": {
            "type": "operation"
        },
        "smithy.example#MyService":{
            "type": "service",
            "version": "2017-02-11",
            "operations": [
                {
                    "target": "smithy.example#GetSomething"
                }
            ],
            "rename": {
                "foo.example#Widget": "FooWidget"
            }
        }
    }
}"##,
        model_part::service_with_renames().model,
    )
}
