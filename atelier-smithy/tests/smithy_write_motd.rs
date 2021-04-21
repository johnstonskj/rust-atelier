use atelier_core::builder::{
    traits, MemberBuilder, ModelBuilder, OperationBuilder, ResourceBuilder, ServiceBuilder,
    ShapeTraits, SimpleShapeBuilder, StructureBuilder,
};
use atelier_core::error::ErrorSource;
use atelier_core::io::write_model_to_string;
use atelier_core::model::{Model, NamespaceID};
use atelier_core::Version;
use atelier_smithy::SmithyWriter;

#[test]
pub fn write_motd_model() {
    let model: Model = ModelBuilder::new(Version::V10, "example.motd")
        .service(
            ServiceBuilder::new("MessageOfTheDay", "2020-06-21")
                .documentation("Provides a Message of the day.")
                .resource("Message")
                .into(),
        )
        .resource(
            ResourceBuilder::new("Message")
                .identifier("date", "Date")
                .read("GetMessage")
                .into(),
        )
        .simple_shape(
            SimpleShapeBuilder::string("Date")
                .apply_trait(traits::pattern(r"^\d\d\d\d\-\d\d-\d\d$"))
                .into(),
        )
        .operation(
            OperationBuilder::new("GetMessage")
                .readonly()
                .input("GetMessageInput")
                .output("GetMessageOutput")
                .error("BadDateValue")
                .into(),
        )
        .structure(
            StructureBuilder::new("GetMessageInput")
                .member("date", "Date")
                .into(),
        )
        .structure(
            StructureBuilder::new("GetMessageOutput")
                .add_member(MemberBuilder::string("message").required().into())
                .into(),
        )
        .structure(
            StructureBuilder::new("BadDateValue")
                .error_source(ErrorSource::Client)
                .add_member(MemberBuilder::string("errorMessage").required().into())
                .into(),
        )
        .into();

    let mut writer = SmithyWriter::new(NamespaceID::new_unchecked("example.motd"));
    let output = write_model_to_string(&mut writer, &model);
    assert!(output.is_ok());
    println!("{}", output.unwrap())
}
