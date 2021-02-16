use atelier_core::builder::{
    traits, MemberBuilder, ModelBuilder, OperationBuilder, ResourceBuilder, ServiceBuilder,
    ShapeTraits, SimpleShapeBuilder, StructureBuilder,
};
use atelier_core::error::ErrorSource;
use atelier_core::model::values::Value;
use atelier_core::model::Model;
use atelier_core::Version;

pub fn make_message_of_the_day_model() -> Model {
    ModelBuilder::new(Version::V10, "example.motd")
        .meta_data("Author".to_string(), Value::String("Simon".to_string()))
        .service(
            ServiceBuilder::new("MessageOfTheDay", "2020-06-21")
                .documentation("Provides a simple per-day message given the day as an input.")
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
                .documentation("Represents a date in YYYY-MM-DD format.")
                .apply_trait(traits::pattern(r"^\d\d\d\d\-\d\d-\d\d$"))
                .into(),
        )
        .operation(
            OperationBuilder::new("GetMessage")
                .documentation("Return the message for a given date. Will return an error if the date string is badly formatted or is in the future.")
                .readonly()
                .input("GetMessageInput")
                .output("GetMessageOutput")
                .error("BadDateValue")
                .into(),
        )
        .structure(
            StructureBuilder::new("GetMessageInput")
                .documentation("The input only requires the date.")
                .member("date", "Date")
                .unstable()
                .into(),
        )
        .structure(
            StructureBuilder::new("GetMessageOutput")
                .documentation("The output is simply the message as a string.")
                .add_member(MemberBuilder::string("message").required().into())
                .into(),
        )
        .structure(
            StructureBuilder::new("BadDateValue")
                .error_source(ErrorSource::Client)
                .add_member(MemberBuilder::string("errorMessage").required().into())
                .into(),
        )
        .into()
}
