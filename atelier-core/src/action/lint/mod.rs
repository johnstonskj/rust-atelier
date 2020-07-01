/*!
This module contains core `Linter` implementations. It also provides a function,
`run_linter_actions`, that takes a list of linters to run against a model. This is the
preferred way to run the linter actions as it allows for _fast fail_ on detecting errors
in an action.
*/

use crate::action::{Action, ActionIssue, IssueLevel, Linter};
use crate::model::shapes::ShapeBody;
use crate::model::{Model, Named, ShapeID};
use heck::{CamelCase, MixedCase};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This will report any violations of the naming conventions described in the Smithy specification
/// and associated guides.
///
/// * Shape names should be in UpperCamelCase.
/// * Member names should be in lowerCamelCase.
///
#[derive(Debug)]
pub struct NamingConventions {}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Run each provided `Linter`, in order, against the provided `Model`. All issues will be collated
/// and returned together.
///
/// The `fail_fast` flag determines the behavior if a linter action returns an error. If `true` the
/// process stops and returns all reported issues up to that point, if `false` it continues on.
///
pub fn run_linter_actions(
    linters: &[Box<dyn Linter>],
    model: &Model,
    fail_fast: bool,
) -> Option<Vec<ActionIssue>> {
    let mut issues: Vec<ActionIssue> = Default::default();

    for linter in linters {
        if let Some(mut new_issues) = linter.check(model) {
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

impl Default for NamingConventions {
    fn default() -> Self {
        Self {}
    }
}

impl Action for NamingConventions {
    fn label(&self) -> &'static str {
        "NamingConventions"
    }
}

impl Linter for NamingConventions {
    fn check(&self, model: &Model) -> Option<Vec<ActionIssue>> {
        let mut issues: Vec<ActionIssue> = Default::default();
        for shape in model.shapes() {
            let shape_name = shape.id().shape_name().to_string();
            if shape_name.to_camel_case() != shape_name {
                issues.push(ActionIssue::info_at(
                    &self.label(),
                    &format!(
                        "Shape names should conform to UpperCamelCase, i.e. {}",
                        shape_name.to_camel_case()
                    ),
                    shape.id().clone(),
                ));
            }
            match shape.body() {
                ShapeBody::Structure(body) | ShapeBody::Union(body) => {
                    for member in body.members() {
                        let member_name = member.id().to_string();
                        if member_name.to_mixed_case() != member_name {
                            issues.push(ActionIssue::info_at(
                                &self.label(),
                                &format!(
                                    "Member names should conform to lowerCamelCase, i.e. {}",
                                    member_name.to_mixed_case()
                                ),
                                ShapeID::new(
                                    Some(model.namespace().clone()),
                                    shape.id().shape_name().clone(),
                                    Some(member.id().clone()),
                                ),
                            ));
                        }
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

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
