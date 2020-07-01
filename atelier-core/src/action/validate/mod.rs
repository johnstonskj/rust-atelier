/*!
This module contains core `Validator` implementations.
*/

use crate::action::{Action, ActionIssue, Validator};
use crate::model::shapes::{ShapeBody, Valued};
use crate::model::{Model, Named, ShapeID};
use std::fmt::{Debug, Formatter};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This validator will actually batch a list of other validators and execute them in order. When
/// constructed you can specify whether it will _fail fast_ that is return the errors from the first
/// validator that fails, or whether it will gather all validation errors and return a combined set.
///
pub struct ValidateAll {
    fast_fail: bool,
    validators: Vec<Box<dyn Validator>>,
}

///
/// This validator will ensure that all references to shape identifiers are valid.
///
#[derive(Debug)]
pub struct NoOrphanedReferences {}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Debug for ValidateAll {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ValidateAll")
            .field("fast_fail", &self.fast_fail)
            .field(
                "validators",
                &self
                    .validators
                    .iter()
                    .map(|v| v.label().to_string())
                    .collect::<Vec<String>>(),
            )
            .finish()
    }
}

impl Action for ValidateAll {
    fn label(&self) -> &'static str {
        "ValidateAll"
    }
}

impl Validator for ValidateAll {
    fn validate(&self, model: &Model) -> Option<Vec<ActionIssue>> {
        let mut issues: Vec<ActionIssue> = Default::default();
        for validator in &self.validators {
            if let Some(mut new_issues) = validator.validate(model) {
                if self.fast_fail {
                    return Some(issues);
                } else {
                    issues.append(&mut new_issues);
                }
            }
        }
        if !issues.is_empty() {
            Some(issues)
        } else {
            None
        }
    }
}

impl ValidateAll {
    ///
    /// Create a validator that will call each validator in order. It will return the error from
    /// the first validator that fails, ignoring the rest.
    ///
    pub fn fast_fail(validators: Vec<Box<dyn Validator>>) -> Self {
        Self {
            fast_fail: true,
            validators,
        }
    }

    ///
    /// Create a validator that will call each validator in order. It will call all the validators
    /// and aggregate the reasons reported by any and all that fail.
    ///
    pub fn run_all(validators: Vec<Box<dyn Validator>>) -> Self {
        Self {
            fast_fail: false,
            validators,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for NoOrphanedReferences {
    fn default() -> Self {
        Self {}
    }
}

impl Action for NoOrphanedReferences {
    fn label(&self) -> &'static str {
        "NoOrphanedReferences"
    }
}

impl Validator for NoOrphanedReferences {
    fn validate(&self, model: &Model) -> Option<Vec<ActionIssue>> {
        let mut issues: Vec<ActionIssue> = Default::default();
        for shape in model.shapes() {
            match shape.body() {
                ShapeBody::List(body) | ShapeBody::Set(body) => {
                    self.resolve(shape.id(), body.member(), model, &mut issues);
                }
                ShapeBody::Map(body) => {
                    self.resolve(shape.id(), body.key(), model, &mut issues);
                    self.resolve(shape.id(), body.value(), model, &mut issues);
                }
                ShapeBody::Structure(body) | ShapeBody::Union(body) => {
                    for member in body.members() {
                        self.resolve(
                            shape.id(),
                            member.value().as_ref().unwrap().as_shape_id().unwrap(),
                            model,
                            &mut issues,
                        );
                    }
                }
                ShapeBody::Service(body) => {
                    for operation in body.operations() {
                        self.resolve(shape.id(), operation, model, &mut issues);
                    }
                    for resource in body.resources() {
                        self.resolve(shape.id(), resource, model, &mut issues);
                    }
                }
                ShapeBody::Operation(body) => {
                    if let Some(input) = body.input() {
                        self.resolve(shape.id(), input, model, &mut issues);
                    }
                    if let Some(output) = body.output() {
                        self.resolve(shape.id(), output, model, &mut issues);
                    }
                    for error in body.errors() {
                        self.resolve(shape.id(), error, model, &mut issues);
                    }
                }
                ShapeBody::Resource(body) => {
                    for (_, shape_id) in body.identifiers() {
                        self.resolve(shape.id(), shape_id, model, &mut issues);
                    }
                    if let Some(create) = body.create() {
                        self.resolve(shape.id(), create, model, &mut issues);
                    }
                    if let Some(put) = body.put() {
                        self.resolve(shape.id(), put, model, &mut issues);
                    }
                    if let Some(read) = body.read() {
                        self.resolve(shape.id(), read, model, &mut issues);
                    }
                    if let Some(update) = body.update() {
                        self.resolve(shape.id(), update, model, &mut issues);
                    }
                    if let Some(delete) = body.delete() {
                        self.resolve(shape.id(), delete, model, &mut issues);
                    }
                    if let Some(list) = body.list() {
                        self.resolve(shape.id(), list, model, &mut issues);
                    }
                    for operation in body.operations() {
                        self.resolve(shape.id(), operation, model, &mut issues);
                    }
                    for operation in body.collection_operations() {
                        self.resolve(shape.id(), operation, model, &mut issues);
                    }
                    for resource in body.resources() {
                        self.resolve(shape.id(), resource, model, &mut issues);
                    }
                }
                ShapeBody::Apply => {
                    self.resolve(shape.id(), shape.id(), model, &mut issues);
                }
                _ => {}
            }
        }
        if issues.is_empty() {
            None
        } else {
            Some(issues)
        }
    }
}

impl NoOrphanedReferences {
    fn resolve(
        &self,
        referrer: &ShapeID,
        shape_id: &ShapeID,
        model: &Model,
        issues: &mut Vec<ActionIssue>,
    ) {
        if model.resolve_id(shape_id, true).is_none() {
            issues.push(ActionIssue::error_at(
                self.label(),
                &format!(
                    "Shape, or member, refers to an unknown identifier: {}",
                    shape_id
                ),
                referrer.clone(),
            ));
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::action::validate::NoOrphanedReferences;
    use crate::action::Validator;
    use crate::model::builder::{ModelBuilder, SimpleShapeBuilder, StructureBuilder};
    use crate::model::Model;
    use crate::Version;

    #[test]
    fn test_no_orphaned_references() {
        //
        // taken from https://awslabs.github.io/smithy/1.0/spec/core/shapes.html#relative-shape-id-resolution
        //
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
                    .into(),
            )
            .shape(SimpleShapeBuilder::boolean("MyBoolean").into())
            .into();
        let validator = NoOrphanedReferences::default();
        let result = validator.validate(&model);
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.len(), 2);
        let result = result
            .iter()
            .map(|issue| issue.message().clone())
            .collect::<Vec<String>>()
            .join("\n");
        assert!(result.contains(": InvalidShape"));
        assert!(result.contains(": foo.baz#MyString"));
    }
}
