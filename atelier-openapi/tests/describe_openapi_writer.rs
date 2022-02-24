use atelier_core::io::{read_model_from_file, write_model_to_string, ModelWriter};
use atelier_openapi::OpenApiWriter;
use atelier_smithy::SmithyReader;
use atelier_test::parse_and_compare_to_files;
use std::{
    fs,
    path::{Path, PathBuf},
};

#[test]
fn test_smithy_prelude() {
    test("union-test");
}

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

// test helper which reads a smithy file, converts it to openapi, and compares with the
// corresponding openapi spec from the /models directory.
// just a naive line by line equality check for now until we have a real model to work with
fn test(file_name: &str) {
    let source_file = PathBuf::from(format!(
        "{}/tests/models/{}.smithy",
        MANIFEST_DIR, file_name
    ));
    let expected_file = PathBuf::from(format!(
        "{}/tests/models/{}.openapi.jsoN",
        MANIFEST_DIR, file_name
    ));

    let mut reader = SmithyReader::default();
    let model = read_model_from_file(&mut reader, PathBuf::from(source_file)).unwrap();

    let mut writer = OpenApiWriter::default();
    let output = write_model_to_string(&mut writer, &model).unwrap();

    let expected_lines: Vec<String> = fs::read_to_string(expected_file)
        .unwrap()
        .split('\n')
        // .split(LINE_ENDING)
        .map(|s| {
            format!(
                "{:?}",
                s.replace("\\n", "\n")
                    .replace("\\t", "\t")
                    .replace("\\\"", "\"")
            )
        })
        .collect();

    let actual_lines: Vec<&str> = output.split("\n").collect();

    assert_eq!(actual_lines, expected_lines);
}
