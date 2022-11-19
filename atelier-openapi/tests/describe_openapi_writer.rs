use atelier_core::io::{read_model_from_file, write_model_to_string};
use atelier_json::JsonReader;
use atelier_openapi::OpenApiWriter;
use atelier_smithy::SmithyReader;
use okapi::openapi3;
use pretty_assertions::assert_eq;
use std::{ffi::OsStr, fs, path::PathBuf};

#[test]
fn test_service() {
    test("test-service.json");
}

#[test]
fn test_unions() {
    test("union-test.smithy");
}

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

// test helper which reads either a smithy idl file or smithy json file, converts it
// to openapi, and compares with the corresponding openapi spec from the /models directory.
fn test(file_name: &str) {
    let source_file = PathBuf::from(format!("{}/tests/models/{}", MANIFEST_DIR, file_name));
    let expected_file_path = source_file.with_extension("openapi.json");

    let extension = source_file.extension().and_then(OsStr::to_str).unwrap();

    let model = match extension {
        "smithy" => read_model_from_file(&mut SmithyReader::default(), source_file),
        "json" => read_model_from_file(&mut JsonReader::default(), source_file),
        _ => panic!("test input extension must be .smithy or .json"),
    }
    .unwrap();

    let mut writer = OpenApiWriter::default();
    let actual_str = write_model_to_string(&mut writer, &model).unwrap();
    let actual_spec: openapi3::OpenApi = serde_json::from_str(&actual_str).unwrap();

    let expected_file = fs::read_to_string(expected_file_path).unwrap();
    let expected_spec: openapi3::OpenApi = serde_json::from_str(&expected_file).unwrap();

    assert_eq!(actual_spec, expected_spec);
}
