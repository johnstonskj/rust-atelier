use atelier_core::builder::{
    MemberBuilder, ModelBuilder, OperationBuilder, ResourceBuilder, ServiceBuilder,
    SimpleShapeBuilder, StructureBuilder, TraitBuilder,
};
use atelier_core::error::ErrorSource;
use atelier_core::io::plant_uml::PlantUmlWriter;
use atelier_core::io::write_model_to_string;
use atelier_core::model::Model;
use atelier_core::Version;

fn make_example_model() -> Model {
    let model: Model = ModelBuilder::new(Version::V10)
        .default_namespace("example.motd")
        .shape(
            ServiceBuilder::new("MessageOfTheDay", "2020-06-21")
                .documentation("Provides a Message of the day.")
                .resource("Message")
                .into(),
        )
        .shape(
            ResourceBuilder::new("Message")
                .identifier("date", "Date")
                .read("GetMessage")
                .into(),
        )
        .shape(
            SimpleShapeBuilder::string("Date")
                .apply_trait(TraitBuilder::pattern(r"^\d\d\d\d\-\d\d-\d\d$").into())
                .into(),
        )
        .shape(
            OperationBuilder::new("GetMessage")
                .readonly()
                .input("GetMessageInput")
                .output("GetMessageOutput")
                .error("BadDateValue")
                .into(),
        )
        .shape(
            StructureBuilder::new("GetMessageInput")
                .add_member(MemberBuilder::new("date", "Date").into())
                .into(),
        )
        .shape(
            StructureBuilder::new("GetMessageOutput")
                .add_member(MemberBuilder::string("message").required().into())
                .into(),
        )
        .shape(
            StructureBuilder::new("BadDateValue")
                .error(ErrorSource::Client)
                .add_member(MemberBuilder::string("errorMessage").required().into())
                .into(),
        )
        .into();
    model
}

#[test]
fn test_uml_writer() {
    let model = make_example_model();
    let mut writer = PlantUmlWriter::new(true);
    let output = write_model_to_string(&mut writer, &model);
    assert!(output.is_ok());
    println!("{}", output.unwrap())
}
