use cargo_atelier::{actions, command_line, report, Command};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    match command_line::parse()? {
        Command::Lint(cmd) => report::report_action_issues(actions::lint_file(cmd)?)?,
        Command::Validate(cmd) => report::report_action_issues(actions::validate_file(cmd)?)?,
        Command::Convert(cmd) => actions::convert_file_format(cmd)?,
    }
    Ok(())
}
