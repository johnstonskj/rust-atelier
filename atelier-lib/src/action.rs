/*!
This module provides functions that wrap common actions into single entry points.
*/

use atelier_core::action::lint::{run_linter_actions, NamingConventions, UnwelcomeTerms};
use atelier_core::action::validate::{
    run_validation_actions, CorrectTypeReferences, NoOrphanedReferences,
};
use atelier_core::action::ActionIssue;
use atelier_core::model::Model;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Execute all the standard model lint actions.
///
pub fn standard_model_lint(model: &Model, fail_fast: bool) -> Option<Vec<ActionIssue>> {
    run_linter_actions(
        &[
            Box::new(NamingConventions::default()),
            Box::new(UnwelcomeTerms::default()),
        ],
        model,
        fail_fast,
    )
}

///
/// Execute all the standard model validation actions.
///
pub fn standard_model_validation(model: &Model, fail_fast: bool) -> Option<Vec<ActionIssue>> {
    run_validation_actions(
        &[
            Box::new(NoOrphanedReferences::default()),
            Box::new(CorrectTypeReferences::default()),
        ],
        model,
        fail_fast,
    )
}
