use atelier_core::io::read_model_from_string;
use atelier_smithy::io::SmithyReader;
use std::fs::File;
use std::io::Read;

#[test]
fn test_weather_example() {
    test_file_parses("weather");
}

fn test_file_parses(file_name: &str) {
    let file_name = format!("tests/good/{}.smithy", file_name);
    println!("{}", file_name);
    let mut file = File::open(file_name).unwrap();
    let mut content: Vec<u8> = Vec::default();
    let _ = file.read_to_end(&mut content).unwrap();

    let mut reader = SmithyReader::default();
    let _ = read_model_from_string(&mut reader, content).unwrap();
}
