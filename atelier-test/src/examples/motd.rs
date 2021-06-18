use crate::TestCaseModel;
use atelier_core::builder::traits::ErrorSource;
use atelier_core::builder::{
    traits, MemberBuilder, ModelBuilder, OperationBuilder, ResourceBuilder, ServiceBuilder,
    ShapeTraits, SimpleShapeBuilder, StructureBuilder,
};
use atelier_core::model::Model;
use atelier_core::Version;
use std::convert::TryInto;

const MESSAGE_OF_THE_DAY_AS_LINES: &[&str] = &[
    "operation::example.motd#GetMessage",
    "operation::example.motd#GetMessage::error=>example.motd#BadDateValue",
    "operation::example.motd#GetMessage::input=>example.motd#GetMessageInput",
    "operation::example.motd#GetMessage::output=>example.motd#GetMessageOutput",
    "operation::example.motd#GetMessage::trait::smithy.api#readonly<={}",
    "resource::example.motd#Message",
    "resource::example.motd#Message::identifier::date=>example.motd#Date",
    "resource::example.motd#Message::read=>example.motd#GetMessage",
    "service::example.motd#MessageOfTheDay",
    "service::example.motd#MessageOfTheDay::resource=>example.motd#Message",
    "service::example.motd#MessageOfTheDay::trait::smithy.api#documentation<=\"Provides a Message of the day.\"",
    "service::example.motd#MessageOfTheDay::version<=\"2020-06-21\"",
    "string::example.motd#Date",
    "string::example.motd#Date::trait::smithy.api#pattern<=\"^\\\\d\\\\d\\\\d\\\\d\\\\-\\\\d\\\\d-\\\\d\\\\d$\"",
    "structure::example.motd#BadDateValue",
    "structure::example.motd#BadDateValue::errorMessage::trait::smithy.api#required<={}",
    "structure::example.motd#BadDateValue::errorMessage=>smithy.api#String",
    "structure::example.motd#BadDateValue::trait::smithy.api#error<=\"client\"",
    "structure::example.motd#GetMessageInput",
    "structure::example.motd#GetMessageInput::date=>example.motd#Date",
    "structure::example.motd#GetMessageOutput",
    "structure::example.motd#GetMessageOutput::message::trait::smithy.api#required<={}",
    "structure::example.motd#GetMessageOutput::message=>smithy.api#String",

];

pub fn make_message_of_the_day_model() -> TestCaseModel {
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
        .try_into()
        .unwrap();
    TestCaseModel {
        model,
        expected_lines: MESSAGE_OF_THE_DAY_AS_LINES.to_vec(),
    }
}
