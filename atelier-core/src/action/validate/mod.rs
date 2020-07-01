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

///
/// This validator ensures that all shape members refer to shapes of the correct type.
///
/// So, a `List` cannot have members that are services and a `Service`'s operations actually have
/// to be `Operation` shapes.
///
#[derive(Debug)]
pub struct CorrectTypeReferences {}

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
    validators: &[Box<dyn Validator>],
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
        traits: &[Trait],
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

impl Default for CorrectTypeReferences {
    fn default() -> Self {
        Self {}
    }
}

impl Action for CorrectTypeReferences {
    fn label(&self) -> &'static str {
        "CorrectTypeReferences"
    }
}

impl Validator for CorrectTypeReferences {
    fn validate(&self, model: &Model) -> Option<Vec<ActionIssue>> {
        let mut issues: Vec<ActionIssue> = Default::default();

        for shape in model.shapes() {
            match shape.body() {
                ShapeBody::List(list_or_set) | ShapeBody::Set(list_or_set) => {
                    self.check_type_only(
                        &shape.id(),
                        list_or_set.member(),
                        model,
                        "List or Set member",
                        &mut issues,
                    );
                }
                ShapeBody::Map(map) => {
                    self.check_type_only(&shape.id(), map.key(), model, "Map key", &mut issues);
                    self.check_type_only(&shape.id(), map.value(), model, "Map value", &mut issues);
                }
                ShapeBody::Structure(structured) | ShapeBody::Union(structured) => {
                    for member in structured.members() {
                        self.check_type_only(
                            &shape.id().to_member(member.id().clone()),
                            &member.value().as_ref().unwrap().as_shape_id().unwrap(),
                            model,
                            "Structure member",
                            &mut issues,
                        );
                    }
                }
                ShapeBody::Service(service) => {
                    for target in service.operations() {
                        self.check_operation_only(
                            &shape.id(),
                            target,
                            model,
                            "Service operation",
                            &mut issues,
                        );
                    }
                    for target in service.resources() {
                        self.check_resource_only(
                            &shape.id(),
                            target,
                            model,
                            "Service resource",
                            &mut issues,
                        );
                    }
                }
                ShapeBody::Operation(operation) => {
                    if let Some(target) = operation.input() {
                        self.check_type_only(
                            &shape.id(),
                            target,
                            model,
                            "Operation input",
                            &mut issues,
                        );
                    }
                    if let Some(target) = operation.output() {
                        self.check_type_only(
                            &shape.id(),
                            target,
                            model,
                            "Operation output",
                            &mut issues,
                        );
                    }
                    for target in operation.errors() {
                        self.check_type_only(
                            &shape.id(),
                            target,
                            model,
                            "Operation error",
                            &mut issues,
                        );
                    }
                }
                ShapeBody::Resource(resource) => {
                    for (id, target) in resource.identifiers() {
                        self.check_type_only(
                            &shape.id().to_member(id.clone()),
                            target,
                            model,
                            "Resource identifier",
                            &mut issues,
                        );
                    }
                    if let Some(target) = resource.create() {
                        self.check_type_only(
                            &shape.id(),
                            target,
                            model,
                            "Resource create",
                            &mut issues,
                        );
                    }
                    if let Some(target) = resource.put() {
                        self.check_type_only(
                            &shape.id(),
                            target,
                            model,
                            "Resource put",
                            &mut issues,
                        );
                    }
                    if let Some(target) = resource.read() {
                        self.check_type_only(
                            &shape.id(),
                            target,
                            model,
                            "Resource read",
                            &mut issues,
                        );
                    }
                    if let Some(target) = resource.update() {
                        self.check_type_only(
                            &shape.id(),
                            target,
                            model,
                            "Resource update",
                            &mut issues,
                        );
                    }
                    if let Some(target) = resource.delete() {
                        self.check_type_only(
                            &shape.id(),
                            target,
                            model,
                            "Resource delete",
                            &mut issues,
                        );
                    }
                    if let Some(target) = resource.list() {
                        self.check_type_only(
                            &shape.id(),
                            target,
                            model,
                            "Resource list",
                            &mut issues,
                        );
                    }
                    for target in resource.operations() {
                        self.check_operation_only(
                            &shape.id(),
                            target,
                            model,
                            "Resource operation",
                            &mut issues,
                        );
                    }
                    for target in resource.collection_operations() {
                        self.check_operation_only(
                            &shape.id(),
                            target,
                            model,
                            "Resource collection operation",
                            &mut issues,
                        );
                    }
                    for target in resource.resources() {
                        self.check_resource_only(
                            &shape.id(),
                            target,
                            model,
                            "Resource resource",
                            &mut issues,
                        );
                    }
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

impl CorrectTypeReferences {
    fn check_type_only(
        &self,
        shape: &ShapeID,
        target: &ShapeID,
        model: &Model,
        member: &str,
        issues: &mut Vec<ActionIssue>,
    ) {
        if let Some(target) = model.shape(target) {
            let target = target.body();
            if target.is_service()
                || target.is_operation()
                || target.is_resource()
                || target.is_apply()
            {
                issues.push(ActionIssue::error_at(
                    self.label(),
                    &format!(
                        "{} may not be a service, operation, resource or apply.",
                        member
                    ),
                    shape.clone(),
                ));
            }
        } else {
            issues.push(ActionIssue::warning_at(
                self.label(),
                &format!(
                    "{} type cannot be resolved in the model to validate.",
                    member
                ),
                shape.clone(),
            ));
        }
    }

    fn check_operation_only(
        &self,
        shape: &ShapeID,
        target: &ShapeID,
        model: &Model,
        member: &str,
        issues: &mut Vec<ActionIssue>,
    ) {
        if let Some(target) = model.shape(target) {
            let target = target.body();
            if !target.is_operation() {
                issues.push(ActionIssue::error_at(
                    self.label(),
                    &format!("{} must be an operation.", member),
                    shape.clone(),
                ));
            }
        } else {
            issues.push(ActionIssue::warning_at(
                self.label(),
                &format!(
                    "{} type cannot be resolved in the model to validate.",
                    member
                ),
                shape.clone(),
            ));
        }
    }

    fn check_resource_only(
        &self,
        shape: &ShapeID,
        target: &ShapeID,
        model: &Model,
        member: &str,
        issues: &mut Vec<ActionIssue>,
    ) {
        if let Some(target) = model.shape(target) {
            let target = target.body();
            if !target.is_resource() {
                issues.push(ActionIssue::error_at(
                    self.label(),
                    &format!("{} must be a resource.", member),
                    shape.clone(),
                ));
            }
        } else {
            issues.push(ActionIssue::warning_at(
                self.label(),
                &format!(
                    "{} type cannot be resolved in the model to validate.",
                    member
                ),
                shape.clone(),
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
        let result =
            run_validation_actions(&[Box::new(NoOrphanedReferences::default())], &model, false);
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
