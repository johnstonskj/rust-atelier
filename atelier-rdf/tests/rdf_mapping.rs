use atelier_rdf::model;

pub mod common;

#[test]
fn test_smithy_to_rdf() {
    let model = common::make_message_of_the_day_model();
    let result = model::model_to_rdf(&model, None);
    assert!(result.is_ok());
    let rdf = result.unwrap();
}
