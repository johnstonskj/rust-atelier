use atelier_core::action::validate::{run_validation_actions, CorrectTypeReferences};
use atelier_core::builder::{ModelBuilder, SimpleShapeBuilder, StructureBuilder, TraitBuilder};
use atelier_core::model::Model;
use atelier_core::Version;

fn make_model() -> Model {
    ModelBuilder::new(Version::V10, "smithy.example")
        .uses("foo.baz#Bar")
        .structure(
            StructureBuilder::new("MyStructure")
                .member("a", "MyString")
                .member("b", "smithy.example#MyString")
                .member("d", "foo.baz#Bar")
                .member("e", "foo.baz#MyString")
                .member("f", "String")
                .member("g", "MyBoolean")
                .apply_trait(TraitBuilder::new("documentation"))
                .into(),
        )
        .simple_shape(SimpleShapeBuilder::string("MyString"))
        .simple_shape(SimpleShapeBuilder::boolean("MyBoolean"))
        .into()
}

#[test]
fn test_correct_type_references() {
    let expected = [
        "The simple shape (smithy.example#MyString) is simply a synonym, did you mean to add any constraint traits?",
        "The simple shape (smithy.example#MyBoolean) is simply a synonym, did you mean to add any constraint traits?",
        "Structure member\'s type (foo.baz#MyString) cannot be resolved to a shape in this model.",
    ];
    let model: Model = make_model();
    let result = run_validation_actions(
        &mut [Box::new(CorrectTypeReferences::default())],
        &model,
        false,
    );
    assert!(result.is_ok());
    let actual = result.unwrap();
    println!("{:#?}", actual);
    assert_eq!(actual.len(), expected.len());
    let actual: Vec<String> = actual
        .iter()
        .map(|issue| issue.message())
        .cloned()
        .collect();
    println!("{:#?}", actual);
    for message in &expected {
        assert!(actual.contains(&message.to_string()));
    }
}
