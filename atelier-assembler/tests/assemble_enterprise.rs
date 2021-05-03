use atelier_assembler::ModelAssembler;
use atelier_core::model::Model;
use std::convert::TryFrom;
use std::path::PathBuf;

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

#[test]
fn merge_good_models() {
    pretty_env_logger::try_init().expect("Could not initialize logger.");

    let mut path = PathBuf::from(MANIFEST_DIR);
    path.push("tests");
    path.push("good");

    let mut assembler = ModelAssembler::default();
    let _ = assembler.push(&path);

    let model = Model::try_from(assembler);
    assert!(model.is_ok());

    let model = model.unwrap();
    println!("{:#?}", model);
}
