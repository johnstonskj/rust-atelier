use atelier_core::action::lint::{run_linter_actions, NamingConventions, UnwelcomeTerms};
use atelier_core::builder::{
    ListBuilder, ModelBuilder, SimpleShapeBuilder, StructureBuilder, TraitBuilder,
};
use atelier_core::model::Model;
use atelier_core::Version;

fn make_model() -> Model {
    ModelBuilder::new(Version::V10, "smithy.example")
        .uses("amazon.fashion#BadTraitName")
        .simple_shape(SimpleShapeBuilder::string("smithy.example#shouldBeUpper"))
        .simple_shape(SimpleShapeBuilder::string("MyString"))
        .simple_shape(SimpleShapeBuilder::string("ThingAsJSON"))
        .list(ListBuilder::new("TheBlacklist", "String"))
        .structure(
            StructureBuilder::new("MyStructure")
                .member("okName", "String")
                .member("BadName", "MyString")
                .member("thing", "ThingAsJSON")
                .member("checkAgainst", "TheBlacklist")
                .member("killMasterNode", "Boolean")
                .apply_trait(TraitBuilder::new("amazon.fashion#BadTraitName"))
                .into(),
        )
        .into()
}

#[test]
fn test_naming_conventions() {
    let expected = [
        "Trait names should conform to lowerCamelCase, i.e. badTraitName",
        "Member names should conform to lowerCamelCase, i.e. badName",
        "Defined shape names should conform to UpperCamelCase, i.e. ShouldBeUpper",
        "Defined shape names should conform to UpperCamelCase, i.e. ThingAsJson",
        "References to shape names should conform to UpperCamelCase, i.e. ThingAsJson",
    ];
    let model: Model = make_model();
    let result = run_linter_actions(&mut [Box::new(NamingConventions::default())], &model, false);
    assert!(result.is_ok());
    let actual = result.unwrap();
    assert_eq!(actual.len(), expected.len());
    let actual: Vec<String> = actual.iter().map(|i| i.message()).cloned().collect();
    println!("{:#?}", actual);
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
    let result = run_linter_actions(&mut [Box::new(UnwelcomeTerms::default())], &model, false);
    assert!(result.is_ok());
    let actual = result.unwrap();
    assert_eq!(actual.len(), expected.len());
    let actual: Vec<String> = actual.iter().map(|i| i.message()).cloned().collect();
    for message in &expected {
        assert!(actual.contains(&message.to_string()));
    }
}
