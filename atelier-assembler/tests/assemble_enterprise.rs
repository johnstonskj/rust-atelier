use atelier_assembler::ModelAssembler;
use atelier_core::model::Model;
use std::convert::TryFrom;

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

#[test]
fn merge_good_models() {
    pretty_env_logger::try_init().expect("Could not initialize logger.");

    let base_dir = format!("{}/tests/good", MANIFEST_DIR);
    let mut assembler = ModelAssembler::default();
    let _ = assembler.push(base_dir.as_ref());

    let model = Model::try_from(assembler);
    assert!(model.is_ok());

    let model = model.unwrap();
    println!("{:#?}", model);
}
