use atelier_assembler::ModelAssembler;
use atelier_core::model::Model;
use atelier_test::compare_model_to_file;
use std::convert::TryFrom;
use std::path::PathBuf;

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

#[test]
fn test_shape_name_resolution() {
    pretty_env_logger::try_init().expect("Could not initialize logger.");

    let mut path = PathBuf::from(MANIFEST_DIR);
    path.push("tests");
    path.push("resolver_cases");

    let mut assembler = ModelAssembler::default();
    let _ = assembler.push(&path);

    let result = Model::try_from(assembler);
    match result {
        Err(e) => {
            eprintln!("{:#?}", e);
            panic!();
        }
        Ok(model) => compare_model_to_file(
            model,
            &PathBuf::from(&format!(
                "{}/tests/resolver_cases/animals.lines",
                MANIFEST_DIR
            )),
        ),
    }
}
