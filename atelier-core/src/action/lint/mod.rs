/*!
This module contains core `Linter` implementations.
*/

use crate::action::{Action, ActionIssue, Linter};
use crate::model::shapes::ShapeBody;
use crate::model::{Model, Named, ShapeID};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This will report any violations of the naming conventions described in the Smithy specification
/// and associated guides.
///
#[derive(Debug)]
pub struct NamingConventions {}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Action for NamingConventions {
    fn label(&self) -> &'static str {
        "NamingConventions"
    }
}

impl Linter for NamingConventions {
    fn check(&self, model: &Model) -> Option<Vec<ActionIssue>> {
        let mut issues: Vec<ActionIssue> = Default::default();
        for shape in model.shapes() {
            if shape
                .id()
                .shape_name()
                .to_string()
                .starts_with(|c: char| c.is_lowercase())
            {
                issues.push(ActionIssue::info_at(
                    &self.label(),
                    "Shape names should start with an upper case character",
                    shape.id().clone(),
                ));
            }
            match shape.body() {
                ShapeBody::Structure(body) | ShapeBody::Union(body) => {
                    for member in body.members() {
                        if member
                            .id()
                            .to_string()
                            .starts_with(|c: char| c.is_uppercase())
                        {
                            issues.push(ActionIssue::info_at(
                                &self.label(),
                                "Member names should start with a lower case character",
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
