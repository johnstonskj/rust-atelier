mod common;
use common::parse_and_write_json;

/// [JsonWriter puts metadata as a child of 'shapes' but it should be top-level](https://github.com/johnstonskj/rust-atelier/issues/34)
#[test]
fn metadata_incorrectly_inside_shapes() {
    parse_and_write_json(
        r#"{
    "smithy": "1.0",
        "metadata": {
        "foo": "hello"
    },
    "shapes": {
    }
}"#,
    )
}
