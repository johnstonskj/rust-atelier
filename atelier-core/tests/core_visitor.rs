use atelier_core::model::shapes::{
    AppliedTraits, Operation, Resource, Service, Simple, StructureOrUnion,
};
use atelier_core::model::visitor::{walk_model, ModelVisitor};
use atelier_core::model::ShapeID;
use std::cell::RefCell;
use std::collections::HashSet;

use atelier_test::examples::motd::make_message_of_the_day_model;

struct ExampleVisitor {
    expected: RefCell<HashSet<String>>,
}

impl Default for ExampleVisitor {
    fn default() -> Self {
        Self {
            expected: RefCell::new(
                [
                    "service@example.motd#MessageOfTheDay",
                    "resource@example.motd#Message",
                    "simple@string@example.motd#Date",
                    "operation@example.motd#GetMessage",
                    "structure@example.motd#GetMessageInput",
                    "structure@example.motd#GetMessageOutput",
                    "structure@example.motd#BadDateValue",
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
        _: &AppliedTraits,
        simple: &Simple,
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
        _: &AppliedTraits,
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
        _: &AppliedTraits,
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
        _: &AppliedTraits,
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
        _: &AppliedTraits,
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
    let model = make_message_of_the_day_model();
    let visitor = ExampleVisitor::default();
    let result = walk_model(&model, &visitor);
    println!("{:#?}", result);
    assert!(result.is_ok());
    let remaining = visitor.expected.borrow();
    assert!(remaining.is_empty());
}
