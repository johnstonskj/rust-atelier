use atelier_core::io::lines::make_line_oriented_form;
use atelier_core::io::read_model_from_file;
use atelier_smithy::SmithyReader;
use pretty_assertions::assert_eq;
use std::fs;
use std::path::PathBuf;

// ------------------------------------------------------------------------------------------------
// Test Cases
// ------------------------------------------------------------------------------------------------

#[test]
fn test_weather_example() {
    test_file_parses("weather");
}

#[test]
fn test_smithy_prelude() {
    test_file_parses("prelude-traits");
}

#[test]
fn test_waiters_example() {
    test_file_parses("waiters");
}

#[test]
fn test_mqtt_api_example() {
    test_file_parses("smithy-api-mqtt");
}

#[test]
fn test_motd_example() {
    test_file_parses("motd");
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

fn test_file_parses(file_name: &str) {
    let source_file = PathBuf::from(format!("{}/tests/good/{}.smithy", MANIFEST_DIR, file_name));
    println!("{:?}", source_file);

    let mut reader = SmithyReader::default();
    let result = read_model_from_file(&mut reader, source_file);

    match result {
        Ok(model) => {
            let actual_lines = make_line_oriented_form(&model);
            for line in &actual_lines {
                println!("{}", line);
            }
            let expected_file =
                PathBuf::from(format!("{}/tests/good/{}.lines", MANIFEST_DIR, file_name));
            let expected_lines = fs::read_to_string(expected_file)
                .unwrap()
                .split('\n')
                .map(str::to_string)
                .collect::<Vec<String>>();
            assert_eq!(actual_lines, expected_lines);
        }
        Err(err) => panic!("{}", err),
    }
}
