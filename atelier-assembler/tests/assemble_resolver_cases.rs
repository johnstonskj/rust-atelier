// **************************************************
// *********** BROKEN TEST BELOW DISABLED ***********
// **************************************************
#[cfg(broken_test)]
#[test]
fn test_shape_name_resolution() {
    use atelier_assembler::ModelAssembler;
    use atelier_core::model::Model;
    use std::convert::TryFrom;
    use std::path::PathBuf;

    const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

    pretty_env_logger::try_init().expect("Could not initialize logger.");

    let mut path = PathBuf::from(MANIFEST_DIR);
    path.push("tests");
    path.push("resolver_cases");

    let mut assembler = ModelAssembler::default();
    let _ = assembler.push(&path);

    let model = Model::try_from(assembler);
    println!("{:#?}", model);
    assert!(model.is_ok());
    assert_ne!(model.unwrap(), Model::default());
}
