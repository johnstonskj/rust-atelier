use atelier_core::io::write_model_to_string;
use atelier_json::JsonWriter;
use atelier_test::examples::weather::make_weather_model;

fn main() {
    let mut writer = JsonWriter::new(true);
    let test_case = make_weather_model();
    let output = write_model_to_string(&mut writer, &test_case.model);
    assert!(output.is_ok());
    println!("{}", output.unwrap())
}
