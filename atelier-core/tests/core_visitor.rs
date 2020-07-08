use atelier_core::error::ErrorSource;
use atelier_core::model::builder::{
    MemberBuilder, ModelBuilder, OperationBuilder, ResourceBuilder, ServiceBuilder, ShapeBuilder,
    SimpleShapeBuilder, StructureBuilder, TraitBuilder,
};
use atelier_core::model::shapes::{
    Operation, Resource, Service, SimpleType, StructureOrUnion, Trait,
};
use atelier_core::model::visitor::{walk_model, ModelVisitor};
use atelier_core::model::{Model, ShapeID};
use atelier_core::Version;
use std::cell::RefCell;
use std::collections::HashSet;

fn make_example_model() -> Model {
    let model: Model = ModelBuilder::new("example.motd", Some(Version::V10))
        .shape(
            ServiceBuilder::new("MessageOfTheDay")
                .documentation("Provides a Message of the day.")
                .version("2020-06-21")
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
                .add_trait(TraitBuilder::pattern(r"^\d\d\d\d\-\d\d-\d\d$").into())
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
                .add_member(MemberBuilder::new("date").refers_to("Date").into())
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

struct ExampleVisitor {
    expected: RefCell<HashSet<String>>,
}

impl Default for ExampleVisitor {
    fn default() -> Self {
        Self {
            expected: RefCell::new(
                [
                    "service@MessageOfTheDay",
                    "resource@Message",
                    "simple@string@Date",
                    "operation@GetMessage",
                    "structure@GetMessageInput",
                    "structure@GetMessageOutput",
                    "structure@BadDateValue",
                ]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            ),
        }
    }
}

impl ModelVisitor for ExampleVisitor {
    type Error = String;

    fn simple_shape(
        &self,
        id: &ShapeID,
        _: &[Trait],
        simple: &SimpleType,
    ) -> std::result::Result<(), Self::Error> {
        let mut expected = self.expected.borrow_mut();
        if expected.remove(&format!("simple@{}@{}", simple, id)) {
            Ok(())
        } else {
            Err(id.to_string())
        }
    }

    fn structure(
        &self,
        id: &ShapeID,
        _: &[Trait],
        _: &StructureOrUnion,
    ) -> std::result::Result<(), Self::Error> {
        let mut expected = self.expected.borrow_mut();
        if expected.remove(&format!("structure@{}", id)) {
            Ok(())
        } else {
            Err(id.to_string())
        }
    }

    fn service(
        &self,
        id: &ShapeID,
        _: &[Trait],
        _: &Service,
    ) -> std::result::Result<(), Self::Error> {
        let mut expected = self.expected.borrow_mut();
        if expected.remove(&format!("service@{}", id)) {
            Ok(())
        } else {
            Err(id.to_string())
        }
    }

    fn operation(
        &self,
        id: &ShapeID,
        _: &[Trait],
        _: &Operation,
    ) -> std::result::Result<(), Self::Error> {
        let mut expected = self.expected.borrow_mut();
        if expected.remove(&format!("operation@{}", id)) {
            Ok(())
        } else {
            Err(id.to_string())
        }
    }

    fn resource(
        &self,
        id: &ShapeID,
        _: &[Trait],
        _: &Resource,
    ) -> std::result::Result<(), Self::Error> {
        let mut expected = self.expected.borrow_mut();
        if expected.remove(&format!("resource@{}", id)) {
            Ok(())
        } else {
            Err(id.to_string())
        }
    }
}

#[test]
fn test_model_visitor() {
    let model = make_example_model();
    let visitor = ExampleVisitor::default();
    let result = walk_model(&model, &visitor);
    println!("{:#?}", result);
    assert!(result.is_ok());
    let remaining = visitor.expected.borrow();
    assert!(remaining.is_empty());
}
