use atelier_core::io::read_model_from_string;
use atelier_json::JsonReader;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Test Cases
// ------------------------------------------------------------------------------------------------

#[test]
fn test_aws_api() {
    test_file_parses("aws.api");
}

#[test]
fn test_aws_api_gateway() {
    test_file_parses("aws.apigateway");
}

#[test]
fn test_aws_auth() {
    test_file_parses("aws.auth");
}

#[test]
fn test_aws_iam() {
    test_file_parses("aws.iam");
}

#[test]
fn test_aws_protocols() {
    test_file_parses("aws.protocols");
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

fn test_file_parses(file_name: &str) {
    let mut path = PathBuf::from_str(MANIFEST_DIR).unwrap();
    path.push(format!("tests/good/{}.json", file_name));
    println!("{:?}", path);
    let mut file = File::open(path).unwrap();
    let mut content: Vec<u8> = Vec::default();
    let _ = file.read_to_end(&mut content).unwrap();

    let mut reader = JsonReader::default();
    let _ = read_model_from_string(&mut reader, content).unwrap();
}
