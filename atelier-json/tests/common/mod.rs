use atelier_core::io::ModelReader;
use atelier_core::model::Model;
use atelier_json::{model_to_json, JsonReader};
use pretty_assertions::assert_eq;
use serde_json::Value;

pub fn parse_and_write_json(json: &str) {
    // Read the string and create a new model.
    let mut reader = JsonReader::default();
    let model: Model = reader.read(&mut json.as_bytes()).unwrap();

    println!("{:#?}", model);

    // From that model, create a JSON AST value.
    let value = model_to_json(&model);

    // Compare with serde json expected value.
    let expected: Value = serde_json::from_str(json).unwrap();
    assert_eq!(&value, &expected);
}
