/*!
This module contains core `Validator` implementations. It also provides a function,
`run_validation_actions`, that takes a list of validators to run against a model. This is the
preferred way to run the validation actions as it allows for _fast fail_ on detecting errors
in an action.
*/

use crate::action::{Action, ActionIssue, IssueLevel, Validator};
use crate::error::Result as ModelResult;
use crate::model::shapes::{HasTraits, ShapeKind};
use crate::model::{HasIdentity, Model, ShapeID};
use crate::prelude::PRELUDE_NAMESPACE;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

linter_or_validator_defn! { CorrectTypeReferences, r#"This validator ensures that all shape members
refer to shapes of the correct type.

So, a `List` cannot have members that are services and a `Service`'s operations actually have
to be `Operation` shapes."# }

linter_or_validator_defn! { NoUnresolvedReferences, r#"This validator ensures that the model
is complete; the model contains no unresolved shape references."# }
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
                        list_or_set.member().target(),
                        model,
                        "List or Set member",
                    );
                }
                ShapeKind::Map(map) => {
                    self.check_type_only(&shape.id(), map.key().target(), model, "Map key");
                    self.check_type_only(&shape.id(), map.value().target(), model, "Map value");
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
                            &shape.id().make_member(id.clone()),
                            &target,
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
        } else if target.namespace().to_string() != PRELUDE_NAMESPACE {
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
        } else if target.namespace().to_string() != PRELUDE_NAMESPACE {
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
        } else if target.namespace().to_string() != PRELUDE_NAMESPACE {
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

// ------------------------------------------------------------------------------------------------

linter_or_validator_default_impl! { NoUnresolvedReferences }

linter_or_validator_action_impl! { NoUnresolvedReferences, "NoUnresolvedReferences" }

impl Validator for NoUnresolvedReferences {
    fn validate(&mut self, model: &Model) -> ModelResult<()> {
        for shape_id in model.shapes().filter_map(|shape| match shape.body() {
            ShapeKind::Unresolved => Some(shape.id().to_string()),
            _ => None,
        }) {
            self.issues.push(ActionIssue::error(
                self.label(),
                &format!(
                    "The model has an unresolved reference to shape '{}'",
                    shape_id
                ),
            ));
        }
        Ok(())
    }
}
