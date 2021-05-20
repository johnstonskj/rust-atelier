use atelier_smithy::SmithyReader;
use atelier_test::parse_and_compare_to_files;
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
    let expected_file = PathBuf::from(format!("{}/tests/good/{}.lines", MANIFEST_DIR, file_name));
    let mut reader = SmithyReader::default();
    parse_and_compare_to_files(&mut reader, &source_file, &expected_file);
}
