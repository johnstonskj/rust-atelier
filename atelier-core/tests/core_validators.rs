use atelier_core::action::validate::{
    run_validation_actions, CorrectTypeReferences, NoOrphanedReferences,
};
use atelier_core::builder::{ModelBuilder, SimpleShapeBuilder, StructureBuilder, TraitBuilder};
use atelier_core::model::Model;
use atelier_core::Version;

fn make_model() -> Model {
    ModelBuilder::new(Version::V10)
        .default_namespace("smithy.example")
        .uses("foo.baz#Bar")
        .shape(SimpleShapeBuilder::string("MyString").into())
        .shape(
            StructureBuilder::new("MyStructure")
                .member("a", "MyString")
                .member("b", "smithy.example#MyString")
                .member("c", "Bar")
                .member("d", "foo.baz#Bar")
                .member("e", "foo.baz#MyString")
                .member("f", "String")
                .member("g", "MyBoolean")
                .member("h", "InvalidShape")
                .apply_trait(TraitBuilder::new("documentation").into())
                .apply_trait(TraitBuilder::new("notKnown").into())
                .into(),
        )
        .shape(SimpleShapeBuilder::boolean("MyBoolean").into())
        .into()
}

#[test]
fn test_no_orphaned_references() {
    let expected = [
        "Shape, or member, has a trait that refers to an unknown identifier: notKnown",
        "Shape, or member, refers to an unknown identifier: InvalidShape",
        "Shape, or member, refers to an unknown identifier: foo.baz#MyString",
    ];
    let model: Model = make_model();
    let result = run_validation_actions(
        &mut [Box::new(NoOrphanedReferences::default())],
        &model,
        false,
    );
    assert!(result.is_ok());
    let actual = result.unwrap();
    assert_eq!(actual.len(), expected.len());
    let actual: Vec<String> = actual
        .iter()
        .map(|issue| issue.message())
        .cloned()
        .collect();
    for message in &expected {
        assert!(actual.contains(&message.to_string()));
    }
}

#[test]
fn test_correct_type_references() {
    let expected = [
        "The simple shape (MyString) is simply a synonym, did you mean to add any constraint traits?",
        "The simple shape (MyBoolean) is simply a synonym, did you mean to add any constraint traits?",
        "Structure member\'s type (Bar) cannot be resolved to a shape in this model.",
        "Structure member\'s type (foo.baz#Bar) cannot be resolved to a shape in this model.",
        "Structure member\'s type (foo.baz#MyString) cannot be resolved to a shape in this model.",
        "Structure member\'s type (InvalidShape) cannot be resolved to a shape in this model."
    ];
    let model: Model = make_model();
    let result = run_validation_actions(
        &mut [Box::new(CorrectTypeReferences::default())],
        &model,
        false,
    );
    assert!(result.is_ok());
    let actual = result.unwrap();
    assert_eq!(actual.len(), expected.len());
    let actual: Vec<String> = actual
        .iter()
        .map(|issue| issue.message())
        .cloned()
        .collect();
    for message in &expected {
        assert!(actual.contains(&message.to_string()));
    }
}
