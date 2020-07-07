use atelier_core::action::lint::{run_linter_actions, NamingConventions, UnwelcomeTerms};
use atelier_core::model::builder::{
    ModelBuilder, ShapeBuilder, SimpleShapeBuilder, StructureBuilder, TraitBuilder,
};
use atelier_core::model::Model;
use atelier_core::Version;

fn make_model() -> Model {
    ModelBuilder::new("smithy.example", Some(Version::V10))
        .shape(SimpleShapeBuilder::string("shouldBeUpper").into())
        .shape(
            StructureBuilder::new("MyStructure")
                .member("okName", "String")
                .member("BadName", "MyString")
                .member("thing", "ThingAsJSON")
                .member("checkAgainst", "TheBlacklist")
                .member("killMasterNode", "Boolean")
                .add_trait(TraitBuilder::new("BadTraitName").into())
                .into(),
        )
        .into()
}

#[test]
fn test_naming_conventions() {
    let expected = [
        "Shape names should conform to UpperCamelCase, i.e. ShouldBeUpper",
        "Trait names should conform to lowerCamelCase, i.e. badTraitName",
        "Member names should conform to lowerCamelCase, i.e. badName",
        "Shape names should conform to UpperCamelCase, i.e. ThingAsJson",
    ];
    let model: Model = make_model();
    let result = run_linter_actions(&[Box::new(NamingConventions::default())], &model, false);
    assert!(result.is_some());
    let actual = result.unwrap();
    assert_eq!(actual.len(), expected.len());
    let actual: Vec<String> = actual.iter().map(|i| i.message()).cloned().collect();
    for message in &expected {
        assert!(actual.contains(&message.to_string()));
    }
}

#[test]
fn test_unwelcome_terms() {
    let expected = [
        "The term \'kill\' is considered either insensitive, divisive, or otherwise unwelcome",
        "The term \'master\' is considered either insensitive, divisive, or otherwise unwelcome",
        "The term \'blacklist\' is considered either insensitive, divisive, or otherwise unwelcome",
    ];
    let model: Model = make_model();
    let result = run_linter_actions(&[Box::new(UnwelcomeTerms::default())], &model, false);
    assert!(result.is_some());
    let actual = result.unwrap();
    assert_eq!(actual.len(), expected.len());
    let actual: Vec<String> = actual.iter().map(|i| i.message()).cloned().collect();
    for message in &expected {
        assert!(actual.contains(&message.to_string()));
    }
}
