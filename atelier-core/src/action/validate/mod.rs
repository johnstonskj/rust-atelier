/*!
This module contains core `Validator` implementations. It also provides a function,
`run_validation_actions`, that takes a list of validators to run against a model. This is the
preferred way to run the validation actions as it allows for _fast fail_ on detecting errors
in an action.
*/

use crate::action::{Action, ActionIssue, IssueLevel, Validator};
use crate::model::shapes::{ShapeBody, Trait, Valued};
use crate::model::{Annotated, Identifier, Model, Named, ShapeID};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

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

///
/// Run each provided `Validator`, in order, against the provided `Model`. All issues will be
/// collated and returned together.
///
/// The `fail_fast` flag determines the behavior if a validation action returns an error. If `true`  
/// the process stops and returns all reported issues up to that point, if `false` it continues on.
///
pub fn run_validation_actions(
    validators: &[impl Validator],
    model: &Model,
    fail_fast: bool,
) -> Option<Vec<ActionIssue>> {
    let mut issues: Vec<ActionIssue> = Default::default();

    for validator in validators {
        if let Some(mut new_issues) = validator.validate(model) {
            issues.append(&mut new_issues);
            if fail_fast
                && new_issues
                    .iter()
                    .any(|issue| issue.level > IssueLevel::Warning)
            {
                return Some(issues);
            }
        }
    }

    if issues.is_empty() {
        None
    } else {
        Some(issues)
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations
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
    use crate::action::validate::{run_validation_actions, NoOrphanedReferences};
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
                    .add_trait(TraitBuilder::new("documentation").into())
                    .add_trait(TraitBuilder::new("notKnown").into())
                    .into(),
            )
            .shape(SimpleShapeBuilder::boolean("MyBoolean").into())
            .into();
        println!("{:?}", model);
        let result = run_validation_actions(&[NoOrphanedReferences::default()], &model, false);
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
