/*!
Reads and writes the [JSON AST](https://awslabs.github.io/smithy/1.0/spec/core/json-ast.html)
representation described in the specification.

As that there is no separate model-level namespace, all shape names have been made absolute in
this representation. This is a problem when constructing the model, if there are shapes from
different namespaces present the parser will return an error at this time.

# Example

The following JSON demonstrates the structure of the AST format.

```json
{
    "smithy": "1.0",
    "metadata": {
        "authors": [
            "Simon"
        ]
    },
    "shapes": {
        "smithy.example#MyString": {
            "type": "string",
            "traits": {
                "smithy.api#documentation": "My documentation string",
                "smithy.api#tags": [
                    "a",
                    "b"
                ]
            }
        },
        "smithy.example#MyList": {
            "type": "list",
            "member": {
                "target": "smithy.api#String"
            }
        },
        "smithy.example#MyStructure": {
            "type": "structure",
            "members": {
                "stringMember": {
                    "target": "smithy.api#String",
                    "traits": {
                        "smithy.api#required": {}
                    }
                },
                "numberMember": {
                    "target": "smithy.api#Integer"
                }
            }
        }
    }
}
```

The following will parse the model above.

```rust
use atelier_core::io::read_model_from_string;
use atelier_json::JsonReader;

# const JSON: &str =
#        r#"{ "smithy": "1.0", "shapes": { "smithy.example#MyString": { "type": "string" } } }"#;
let mut reader = JsonReader::default();
let result = read_model_from_string(&mut reader, JSON);
if result.is_err() {
    println!("{:?}", result);
}
assert!(result.is_ok());
println!("{:#?}", result.unwrap());
```

*/

#![warn(
    // ---------- Stylistic
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    // ---------- Public
    missing_debug_implementations,
    missing_docs,
    unreachable_pub,
    // ---------- Unsafe
    unsafe_code,
    // ---------- Unused
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
)]

// ------------------------------------------------------------------------------------------------
// Public Values
// ------------------------------------------------------------------------------------------------

///
/// The extension to use when reading from, or writing to, files of this type.
///
pub const FILE_EXTENSION: &str = "json";

///
/// The name to report in errors in this representation.
///
pub const REPRESENTATION_NAME: &str = "JSON AST";

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
pub mod reader;
pub use reader::JsonReader;

#[doc(hidden)]
pub mod writer;
pub use writer::JsonWriter;

mod syntax;
