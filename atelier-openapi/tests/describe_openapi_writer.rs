use atelier_core::io::{read_model_from_file, write_model_to_string};
use atelier_openapi::OpenApiWriter;
use atelier_smithy::SmithyReader;
use std::{fs, path::PathBuf};

use okapi::openapi3;

#[test]
fn test_unions() {
    test("union-test");
}

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

// test helper which reads a smithy file, converts it to openapi, and compares with the
// corresponding openapi spec from the /models directory.
fn test(file_name: &str) {
    let source_file = PathBuf::from(format!(
        "{}/tests/models/{}.smithy",
        MANIFEST_DIR, file_name
    ));
    let expected_file_path = PathBuf::from(format!(
        "{}/tests/models/{}.openapi.jsoN",
        MANIFEST_DIR, file_name
    ));

    let mut reader = SmithyReader::default();
    let model = read_model_from_file(&mut reader, source_file).unwrap();

    let mut writer = OpenApiWriter::default();
    let actual_str = write_model_to_string(&mut writer, &model).unwrap();
    let actual_spec: openapi3::OpenApi = serde_json::from_str(&actual_str).unwrap();

    let expected_file = fs::read_to_string(expected_file_path).unwrap();
    let expected_spec: openapi3::OpenApi = serde_json::from_str(&expected_file).unwrap();

    assert_eq!(actual_spec, expected_spec);
}
