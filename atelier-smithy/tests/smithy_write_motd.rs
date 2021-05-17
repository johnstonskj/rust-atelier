use atelier_core::io::write_model_to_string;
use atelier_core::model::{Model, NamespaceID};
use atelier_smithy::SmithyWriter;
use atelier_test::examples::motd::make_message_of_the_day_model;

#[test]
pub fn write_motd_model() {
    let model: Model = make_message_of_the_day_model();

    let mut writer = SmithyWriter::new(NamespaceID::new_unchecked("example.motd"));
    let output = write_model_to_string(&mut writer, &model);
    assert!(output.is_ok());
    println!("{}", output.unwrap())
}
