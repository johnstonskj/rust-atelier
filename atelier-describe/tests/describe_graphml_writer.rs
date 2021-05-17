use atelier_test::examples::motd::make_message_of_the_day_model;

#[test]
fn test_graphml_writer() {
    use atelier_core::io::write_model_to_string;
    use atelier_describe::graphml::writer::GraphMLWriter;

    let model = make_message_of_the_day_model();
    let mut writer = GraphMLWriter::default();
    let output = write_model_to_string(&mut writer, &model);
    assert!(output.is_ok());
    println!("{}", output.unwrap())
}
