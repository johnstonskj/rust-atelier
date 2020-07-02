/*!
One-line description.

More detailed description, with

# Example

*/

use atelier_lib::core::action::{ActionIssue, IssueLevel};
use std::error::Error;

#[cfg(feature = "color")]
use colored::Colorize;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn report_action_issues(
    issues: Option<Vec<ActionIssue>>,
    color: bool,
) -> Result<(), Box<dyn Error>> {
    println!();
    if let Some(issues) = issues {
        for issue in issues {
            if color {
                report_issue(issue)
            } else {
                report_issue_no_color(issue)
            }
        }
    } else {
        println!("No issues reported.");
    }
    Ok(())
}

fn report_issue_no_color(issue: ActionIssue) {
    println!("[{}] {}", issue.level(), issue.message(),);
    println!(
        "\tReported by {} for element {}.",
        issue.reporter(),
        match issue.locus() {
            Some(id) => id.to_string(),
            None => String::new(),
        }
    );
    println!()
}

#[cfg(not(feature = "color"))]
fn report_issue(issue: ActionIssue) {
    report_issue_no_color(issue)
}

#[cfg(feature = "color")]
fn report_issue(issue: ActionIssue) {
    println!(
        "{} {}",
        match issue.level() {
            IssueLevel::Info => "[info]".normal(),
            IssueLevel::Warning => "[warning]".yellow(),
            IssueLevel::Error => "[error]".bright_red(),
        },
        issue.message().bold()
    );

    println!(
        "{}",
        format!(
            "\tReported by {}{}.",
            issue.reporter(),
            match issue.locus() {
                Some(id) => format!(" on/for element `{}`", id.to_string().underline()),
                None => String::new(),
            }
        )
        .dimmed()
    );
    println!()
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
