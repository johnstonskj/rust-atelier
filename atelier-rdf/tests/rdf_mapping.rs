use atelier_rdf::writer::model_to_rdf;
use atelier_test::examples::motd::make_message_of_the_day_model;
use rdftk_io::turtle::writer::TurtleWriter;
use rdftk_io::GraphWriter;

#[test]
fn test_smithy_to_rdf() {
    let model = make_message_of_the_day_model();

    let result = model_to_rdf(&model.model, None);
    assert!(result.is_ok());
    let rdf = result.unwrap();

    let writer = TurtleWriter::default();
    assert!(writer.write(&mut std::io::stdout(), &rdf).is_ok());
}
