use atelier_core::io::write_model_to_string;
use atelier_describe::document::writer::DocumentationWriter;
use atelier_test::examples::motd::make_message_of_the_day_model;

#[test]
pub fn test_documentation_writer() {
    let model = make_message_of_the_day_model();
    let mut writer = DocumentationWriter::default();
    let out_str = write_model_to_string(&mut writer, &model).unwrap();
    println!("{}", out_str);
}
