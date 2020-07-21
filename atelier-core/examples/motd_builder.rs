use atelier_core::builder::{
    MemberBuilder, ModelBuilder, OperationBuilder, ResourceBuilder, ServiceBuilder,
    SimpleShapeBuilder, StructureBuilder, TraitBuilder,
};
use atelier_core::error::ErrorSource;
use atelier_core::model::Model;
use atelier_core::Version;

pub fn main() {
    let model: Model = ModelBuilder::new(Version::V10, "example.motd")
        .service(
            ServiceBuilder::new("MessageOfTheDay", "2020-06-21")
                .documentation("Provides a Message of the day.")
                .resource("Message"),
        )
        .resource(
            ResourceBuilder::new("Message")
                .identifier("date", "Date")
                .read("GetMessage"),
        )
        .simple_shape(
            SimpleShapeBuilder::string("Date")
                .apply_trait(TraitBuilder::pattern(r"^\d\d\d\d\-\d\d-\d\d$").into()),
        )
        .operation(
            OperationBuilder::new("GetMessage")
                .readonly()
                .input("GetMessageInput")
                .output("GetMessageOutput")
                .error("BadDateValue"),
        )
        .structure(StructureBuilder::new("GetMessageInput").member("date", "Date"))
        .structure(
            StructureBuilder::new("GetMessageOutput")
                .add_member(MemberBuilder::string("message").required().into()),
        )
        .structure(
            StructureBuilder::new("BadDateValue")
                .error_source(ErrorSource::Client)
                .add_member(MemberBuilder::string("errorMessage").required().into()),
        )
        .into();
    println!("{:#?}", model);
}
