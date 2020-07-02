use atelier_core::action::lint::{run_linter_actions, NamingConventions};
use atelier_core::model::builder::{
    ModelBuilder, ShapeBuilder, SimpleShapeBuilder, StructureBuilder, TraitBuilder,
};
use atelier_core::model::Model;
use atelier_core::Version;

#[test]
fn test_naming_conventions() {
    let model: Model = ModelBuilder::new("smithy.example", Some(Version::V10))
        .shape(SimpleShapeBuilder::string("shouldBeUpper").into())
        .shape(
            StructureBuilder::new("MyStructure")
                .member("okName", "String")
                .member("BadName", "MyString")
                .member("thing", "ThingAsJSON")
                .add_trait(TraitBuilder::new("BadTraitName").into())
                .into(),
        )
        .into();
    let result = run_linter_actions(&[Box::new(NamingConventions::default())], &model, false);
    println!("{:#?}", result);
}
