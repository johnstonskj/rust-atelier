use atelier_rdf::writer::model_to_rdf;
use rdftk_io::turtle::TurtleWriter;
use rdftk_io::GraphWriter;

pub mod common;

#[test]
fn test_smithy_to_rdf() {
    let model = common::make_message_of_the_day_model();

    let result = model_to_rdf(&model, None);
    assert!(result.is_ok());
    let rdf = result.unwrap();

    let writer = TurtleWriter::default();
    assert!(writer.write(&mut std::io::stdout(), &rdf).is_ok());
}
