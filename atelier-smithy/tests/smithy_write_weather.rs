use atelier_core::io::write_model_to_string;
use atelier_core::model::NamespaceID;
use atelier_smithy::SmithyWriter;
use atelier_test::examples::weather::make_weather_model;

#[test]
fn write_weather_example() {
    let mut writer = SmithyWriter::new(NamespaceID::new_unchecked("example.weather"));
    let model = make_weather_model();
    let output = write_model_to_string(&mut writer, &model.model);
    assert!(output.is_ok());
    println!("{}", output.unwrap())
}
