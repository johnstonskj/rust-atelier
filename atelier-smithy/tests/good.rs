use atelier_core::io::read_model_from_string;
use atelier_smithy::io::SmithyReader;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;

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

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

fn test_file_parses(file_name: &str) {
    let mut path = PathBuf::from_str(MANIFEST_DIR).unwrap();
    path.push(format!("tests/good/{}.smithy", file_name));
    println!("{:?}", path);
    let mut file = File::open(path).unwrap();
    let mut content: Vec<u8> = Vec::default();
    let _ = file.read_to_end(&mut content).unwrap();

    let mut reader = SmithyReader::default();
    let _ = read_model_from_string(&mut reader, content).unwrap();
}
