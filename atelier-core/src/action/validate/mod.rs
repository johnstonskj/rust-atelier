/*!
This module contains core `Validator` implementations. It also provides a function,
`run_validation_actions`, that takes a list of validators to run against a model. This is the
preferred way to run the validation actions as it allows for _fast fail_ on detecting errors
in an action.
*/

use crate::action::{Action, ActionIssue, IssueLevel, Validator};
use crate::error::Result as ModelResult;
use crate::model::shapes::{AppliedTrait, Shape, ShapeKind};
use crate::model::{Identifier, Model, ShapeID};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

linter_or_validator_defn! { NoOrphanedReferences, r#"This validator will ensure that all references to
shape identifiers are valid.

For every shape it will ensure all members refer to shapes that can be resolved. It also
ensures that all traits on shapes and members have names that can be resolved."# }

linter_or_validator_defn! { CorrectTypeReferences, r#"This validator ensures that all shape members
refer to shapes of the correct type.

So, a `List` cannot have members that are services and a `Service`'s operations actually have
to be `Operation` shapes."# }

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
    validators: &mut [Box<dyn Validator>],
    model: &Model,
    fail_fast: bool,
) -> ModelResult<Vec<ActionIssue>> {
    let mut issues: Vec<ActionIssue> = Default::default();

    for validator in validators.iter_mut() {
        validator.validate(model)?;
        let new_issues = validator.issues_mut();
        issues.append(new_issues);
        if fail_fast
            && new_issues
                .iter()
                .any(|issue| issue.level > IssueLevel::Warning)
        {
            break;
        }
    }

    Ok(issues)
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

linter_or_validator_default_impl! { NoOrphanedReferences }

linter_or_validator_action_impl! { NoOrphanedReferences, "NoOrphanedReferences" }

impl Validator for NoOrphanedReferences {
    fn validate(&mut self, model: &Model) -> ModelResult<()> {
        for shape in model.shapes() {
            let this_shape_id = shape.id();
            self.resolve_traits(&this_shape_id, shape.traits(), model);
            match shape.body() {
                ShapeKind::List(body) | ShapeKind::Set(body) => {
                    self.resolve(&this_shape_id, body.member().id(), model);
                }
                ShapeKind::Map(body) => {
                    self.resolve(&this_shape_id, body.key().id(), model);
                    self.resolve(&this_shape_id, body.value().id(), model);
                }
                ShapeKind::Structure(body) | ShapeKind::Union(body) => {
                    for member in body.members() {
                        self.resolve_traits(&member.id(), member.traits(), model);
                        self.resolve(&member.id(), member.target(), model);
                    }
                }
                ShapeKind::Service(body) => {
                    for operation in body.operations() {
                        self.resolve(&this_shape_id, operation, model);
                    }
                    for resource in body.resources() {
                        self.resolve(&this_shape_id, resource, model);
                    }
                }
                ShapeKind::Operation(body) => {
                    if let Some(input) = body.input() {
                        self.resolve(&this_shape_id, input, model);
                    }
                    if let Some(output) = body.output() {
                        self.resolve(&this_shape_id, output, model);
                    }
                    for error in body.errors() {
                        self.resolve(&this_shape_id, error, model);
                    }
                }
                ShapeKind::Resource(body) => {
                    for (id, target) in body.identifiers() {
                        self.resolve(
                            &shape.id().make_member(Identifier::from_str(id).unwrap()),
                            &ShapeID::from_str(&target.as_string().unwrap()).unwrap(),
                            model,
                        );
                    }
                    if let Some(create) = body.create() {
                        self.resolve(&this_shape_id, create, model);
                    }
                    if let Some(put) = body.put() {
                        self.resolve(&this_shape_id, put, model);
                    }
                    if let Some(read) = body.read() {
                        self.resolve(&this_shape_id, read, model);
                    }
                    if let Some(update) = body.update() {
                        self.resolve(&this_shape_id, update, model);
                    }
                    if let Some(delete) = body.delete() {
                        self.resolve(&this_shape_id, delete, model);
                    }
                    if let Some(list) = body.list() {
                        self.resolve(&this_shape_id, list, model);
                    }
                    for operation in body.operations() {
                        self.resolve(&this_shape_id, operation, model);
                    }
                    for operation in body.collection_operations() {
                        self.resolve(&this_shape_id, operation, model);
                    }
                    for resource in body.resources() {
                        self.resolve(&this_shape_id, resource, model);
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }
}

impl NoOrphanedReferences {
    fn resolve(&mut self, referrer: &ShapeID, shape_id: &ShapeID, model: &Model) {
        if model.shape(shape_id).is_none() {
            self.issues.push(ActionIssue::error_at(
                self.label(),
                &format!(
                    "Shape, or member, refers to an unknown identifier: {}",
                    shape_id
                ),
                referrer.clone(),
            ));
        }
    }

    fn resolve_traits(&mut self, referrer: &ShapeID, traits: &[AppliedTrait], model: &Model) {
        for a_trait in traits {
            if model.shape(a_trait.id()).is_none() {
                self.issues.push(ActionIssue::error_at(
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

linter_or_validator_default_impl! { CorrectTypeReferences }

linter_or_validator_action_impl! { CorrectTypeReferences, "CorrectTypeReferences" }

impl Validator for CorrectTypeReferences {
    fn validate(&mut self, model: &Model) -> ModelResult<()> {
        for shape in model.shapes() {
            match shape.body() {
                ShapeKind::Simple(_) => {
                    if !shape.has_traits() {
                        self.issues.push(ActionIssue::info_at(
                            self.label(),
                            &format!("The simple shape ({}) is simply a synonym, did you mean to add any constraint traits?", shape.id()),
                            shape.id().clone(),
                        ));
                    }
                }
                ShapeKind::List(list_or_set) | ShapeKind::Set(list_or_set) => {
                    self.check_type_only(
                        &shape.id(),
                        list_or_set.member().id(),
                        model,
                        "List or Set member",
                    );
                }
                ShapeKind::Map(map) => {
                    self.check_type_only(&shape.id(), map.key().id(), model, "Map key");
                    self.check_type_only(&shape.id(), map.value().id(), model, "Map value");
                }
                ShapeKind::Structure(structured) | ShapeKind::Union(structured) => {
                    for member in structured.members() {
                        self.check_type_only(
                            member.id(),
                            &member.target(),
                            model,
                            "Structure member",
                        );
                    }
                }
                ShapeKind::Service(service) => {
                    for target in service.operations() {
                        self.check_operation_only(&shape.id(), target, model, "Service operation");
                    }
                    for target in service.resources() {
                        self.check_resource_only(&shape.id(), target, model, "Service resource");
                    }
                }
                ShapeKind::Operation(operation) => {
                    if let Some(target) = operation.input() {
                        self.check_type_only(&shape.id(), target, model, "Operation input");
                    }
                    if let Some(target) = operation.output() {
                        self.check_type_only(&shape.id(), target, model, "Operation output");
                    }
                    for target in operation.errors() {
                        self.check_type_only(&shape.id(), target, model, "Operation error");
                    }
                }
                ShapeKind::Resource(resource) => {
                    for (id, target) in resource.identifiers() {
                        self.check_type_only(
                            &shape.id().make_member(Identifier::from_str(id).unwrap()),
                            &ShapeID::from_str(&target.as_string().unwrap()).unwrap(),
                            model,
                            "Resource identifier",
                        );
                    }
                    if let Some(target) = resource.create() {
                        self.check_operation_only(
                            &shape.id(),
                            target,
                            model,
                            "Resource operation 'create'",
                        );
                    }
                    if let Some(target) = resource.put() {
                        self.check_operation_only(
                            &shape.id(),
                            target,
                            model,
                            "Resource operation 'put'",
                        );
                    }
                    if let Some(target) = resource.read() {
                        self.check_operation_only(
                            &shape.id(),
                            target,
                            model,
                            "Resource operation 'read'",
                        );
                    }
                    if let Some(target) = resource.update() {
                        self.check_operation_only(
                            &shape.id(),
                            target,
                            model,
                            "Resource operation 'update'",
                        );
                    }
                    if let Some(target) = resource.delete() {
                        self.check_operation_only(
                            &shape.id(),
                            target,
                            model,
                            "Resource operation 'delete'",
                        );
                    }
                    if let Some(target) = resource.list() {
                        self.check_operation_only(
                            &shape.id(),
                            target,
                            model,
                            "Resource operation 'list'",
                        );
                    }
                    for target in resource.operations() {
                        self.check_operation_only(&shape.id(), target, model, "Resource operation");
                    }
                    for target in resource.collection_operations() {
                        self.check_operation_only(
                            &shape.id(),
                            target,
                            model,
                            "Resource collection operation",
                        );
                    }
                    for target in resource.resources() {
                        self.check_resource_only(&shape.id(), target, model, "Resource resource");
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}

impl CorrectTypeReferences {
    fn check_type_only(&mut self, shape: &ShapeID, target: &ShapeID, model: &Model, member: &str) {
        if let Some(target) = model.shape(target) {
            let target = target.body();
            if target.is_service() || target.is_operation() || target.is_resource() {
                self.issues.push(ActionIssue::error_at(
                    self.label(),
                    &format!(
                        "{} must not refer to a service, operation, resource or member.",
                        member
                    ),
                    shape.clone(),
                ));
            }
        } else {
            self.issues.push(ActionIssue::warning_at(
                self.label(),
                &format!(
                    "{}'s type ({}) cannot be resolved to a shape in this model.",
                    member, target,
                ),
                shape.clone(),
            ));
        }
    }

    fn check_operation_only(
        &mut self,
        shape: &ShapeID,
        target: &ShapeID,
        model: &Model,
        member: &str,
    ) {
        if let Some(target) = model.shape(target) {
            let target = target.body();
            if !target.is_operation() {
                self.issues.push(ActionIssue::error_at(
                    self.label(),
                    &format!("{} must be an operation.", member),
                    shape.clone(),
                ));
            }
        } else {
            self.issues.push(ActionIssue::warning_at(
                self.label(),
                &format!(
                    "{}'s type ({}) cannot be resolved to a shape in this model.",
                    member, target,
                ),
                shape.clone(),
            ));
        }
    }

    fn check_resource_only(
        &mut self,
        shape: &ShapeID,
        target: &ShapeID,
        model: &Model,
        member: &str,
    ) {
        if let Some(target) = model.shape(target) {
            let target = target.body();
            if !target.is_resource() {
                self.issues.push(ActionIssue::error_at(
                    self.label(),
                    &format!("{} must be a resource.", member),
                    shape.clone(),
                ));
            }
        } else {
            self.issues.push(ActionIssue::warning_at(
                self.label(),
                &format!(
                    "{} type ({}) cannot be resolved to a shape in this model.",
                    member, target,
                ),
                shape.clone(),
            ));
        }
    }
}
