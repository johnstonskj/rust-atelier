use atelier_core::action::validate::{run_validation_actions, NoOrphanedReferences};
use atelier_core::model::builder::{
    ModelBuilder, ShapeBuilder, SimpleShapeBuilder, StructureBuilder, TraitBuilder,
};
use atelier_core::model::Model;
use atelier_core::Version;

#[test]
fn test_no_orphaned_references() {
    let model: Model = ModelBuilder::new("smithy.example", Some(Version::V10))
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
                .add_trait(TraitBuilder::new("documentation").into())
                .add_trait(TraitBuilder::new("notKnown").into())
                .into(),
        )
        .shape(SimpleShapeBuilder::boolean("MyBoolean").into())
        .into();
    println!("{:?}", model);
    let result =
        run_validation_actions(&[Box::new(NoOrphanedReferences::default())], &model, false);
    assert!(result.is_some());
    let result = result.unwrap();
    println!("{:#?}", result);
    assert_eq!(result.len(), 3);
    let result = result
        .iter()
        .map(|issue| issue.message().clone())
        .collect::<Vec<String>>()
        .join("\n");
    assert!(result.contains(": notKnown"));
    assert!(result.contains(": InvalidShape"));
    assert!(result.contains(": foo.baz#MyString"));
}
