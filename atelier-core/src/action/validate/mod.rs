/*!
This module contains core `Validator` implementations.
*/

use crate::action::{Action, ActionIssue, Validator};
use crate::model::shapes::{ShapeBody, Trait, Valued};
use crate::model::{Annotated, Identifier, Model, Named, ShapeID};
use std::fmt::{Debug, Formatter};
use std::str::FromStr;

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
/// For every shape it will ensure all members refer to shapes that can be resolved. It also
/// ensures that all traits on shapes and members have names that can be resolved.
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
        let member_id = Identifier::from_str("member").unwrap();
        let key_id = Identifier::from_str("key").unwrap();
        let value_id = Identifier::from_str("value").unwrap();
        let operations_id = Identifier::from_str("operations").unwrap();
        let resources_id = Identifier::from_str("resources").unwrap();
        let input_id = Identifier::from_str("input").unwrap();
        let output_id = Identifier::from_str("output").unwrap();
        let errors_id = Identifier::from_str("errors").unwrap();
        let create_id = Identifier::from_str("create").unwrap();
        let put_id = Identifier::from_str("put").unwrap();
        let read_id = Identifier::from_str("read").unwrap();
        let update_id = Identifier::from_str("update").unwrap();
        let delete_id = Identifier::from_str("delete").unwrap();
        let list_id = Identifier::from_str("list").unwrap();
        let collection_operations_id = Identifier::from_str("collectionOperations").unwrap();
        let mut issues: Vec<ActionIssue> = Default::default();
        for shape in model.shapes() {
            let this_shape_id = shape.id();
            self.resolve_traits(&this_shape_id, shape.traits(), model, &mut issues);
            match shape.body() {
                ShapeBody::List(body) | ShapeBody::Set(body) => {
                    self.resolve(
                        &this_shape_id.to_member(member_id.clone()),
                        body.member(),
                        model,
                        &mut issues,
                    );
                }
                ShapeBody::Map(body) => {
                    self.resolve(
                        &this_shape_id.to_member(key_id.clone()),
                        body.key(),
                        model,
                        &mut issues,
                    );
                    self.resolve(
                        &this_shape_id.to_member(value_id.clone()),
                        body.value(),
                        model,
                        &mut issues,
                    );
                }
                ShapeBody::Structure(body) | ShapeBody::Union(body) => {
                    for member in body.members() {
                        let this_member_id = &this_shape_id.to_member(member.id().clone());
                        self.resolve_traits(&this_member_id, member.traits(), model, &mut issues);
                        self.resolve(
                            &this_member_id,
                            member.value().as_ref().unwrap().as_shape_id().unwrap(),
                            model,
                            &mut issues,
                        );
                    }
                }
                ShapeBody::Service(body) => {
                    for operation in body.operations() {
                        self.resolve(
                            &this_shape_id.to_member(operations_id.clone()),
                            operation,
                            model,
                            &mut issues,
                        );
                    }
                    for resource in body.resources() {
                        self.resolve(
                            &this_shape_id.to_member(resources_id.clone()),
                            resource,
                            model,
                            &mut issues,
                        );
                    }
                }
                ShapeBody::Operation(body) => {
                    if let Some(input) = body.input() {
                        self.resolve(
                            &this_shape_id.to_member(input_id.clone()),
                            input,
                            model,
                            &mut issues,
                        );
                    }
                    if let Some(output) = body.output() {
                        self.resolve(
                            &this_shape_id.to_member(output_id.clone()),
                            output,
                            model,
                            &mut issues,
                        );
                    }
                    for error in body.errors() {
                        self.resolve(
                            &this_shape_id.to_member(errors_id.clone()),
                            error,
                            model,
                            &mut issues,
                        );
                    }
                }
                ShapeBody::Resource(body) => {
                    for (id_id, shape_id) in body.identifiers() {
                        self.resolve(
                            &this_shape_id.to_member(id_id.clone()),
                            shape_id,
                            model,
                            &mut issues,
                        );
                    }
                    if let Some(create) = body.create() {
                        self.resolve(
                            &this_shape_id.to_member(create_id.clone()),
                            create,
                            model,
                            &mut issues,
                        );
                    }
                    if let Some(put) = body.put() {
                        self.resolve(
                            &this_shape_id.to_member(put_id.clone()),
                            put,
                            model,
                            &mut issues,
                        );
                    }
                    if let Some(read) = body.read() {
                        self.resolve(
                            &this_shape_id.to_member(read_id.clone()),
                            read,
                            model,
                            &mut issues,
                        );
                    }
                    if let Some(update) = body.update() {
                        self.resolve(
                            &this_shape_id.to_member(update_id.clone()),
                            update,
                            model,
                            &mut issues,
                        );
                    }
                    if let Some(delete) = body.delete() {
                        self.resolve(
                            &this_shape_id.to_member(delete_id.clone()),
                            delete,
                            model,
                            &mut issues,
                        );
                    }
                    if let Some(list) = body.list() {
                        self.resolve(
                            &this_shape_id.to_member(list_id.clone()),
                            list,
                            model,
                            &mut issues,
                        );
                    }
                    for operation in body.operations() {
                        self.resolve(
                            &this_shape_id.to_member(operations_id.clone()),
                            operation,
                            model,
                            &mut issues,
                        );
                    }
                    for operation in body.collection_operations() {
                        self.resolve(
                            &this_shape_id.to_member(collection_operations_id.clone()),
                            operation,
                            model,
                            &mut issues,
                        );
                    }
                    for resource in body.resources() {
                        self.resolve(
                            &this_shape_id.to_member(resources_id.clone()),
                            resource,
                            model,
                            &mut issues,
                        );
                    }
                }
                ShapeBody::Apply => {
                    self.resolve(this_shape_id, this_shape_id, model, &mut issues);
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

    fn resolve_traits(
        &self,
        referrer: &ShapeID,
        traits: &Vec<Trait>,
        model: &Model,
        issues: &mut Vec<ActionIssue>,
    ) {
        for a_trait in traits {
            if model.resolve_id(a_trait.id(), true).is_none() {
                issues.push(ActionIssue::error_at(
                    self.label(),
                    &format!(
                        "Shape, or member, has a trait that refers to an unknown identifier: {}",
                        a_trait.id()
                    ),
                    referrer.clone(),
                ));
            }
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
    use crate::model::builder::{
        ModelBuilder, ShapeBuilder, SimpleShapeBuilder, StructureBuilder, TraitBuilder,
    };
    use crate::model::Model;
    use crate::Version;

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
                    .add_trait(TraitBuilder::new("notKnown").into())
                    .into(),
            )
            .shape(SimpleShapeBuilder::boolean("MyBoolean").into())
            .into();
        println!("{:?}", model);
        let validator = NoOrphanedReferences::default();
        let result = validator.validate(&model);
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
}
