use cargo_atelier::{actions, command_line, report, Command};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let result = match command_line::parse()? {
        Command::Lint(cmd, options) => {
            report::report_action_issues(actions::lint_file(cmd)?, options.use_color)
        }
        Command::Validate(cmd, options) => {
            report::report_action_issues(actions::validate_file(cmd)?, options.use_color)
        }
        Command::Convert(cmd, _) => actions::convert_file_format(cmd),
        Command::Document(cmd, _) => actions::document_file(cmd),
    };

    if result.is_err() {
        eprintln!("{}", (&result).as_ref().err().unwrap().source().unwrap())
    }

    Ok(())
}
