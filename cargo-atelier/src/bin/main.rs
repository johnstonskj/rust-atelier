use cargo_atelier::{actions, command_line, report, Command};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let result = match command_line::parse()? {
        Command::Lint(cmd, search_path, options) => {
            report::report_action_issues(actions::lint_file(cmd, search_path)?, options.use_color)
        }
        Command::Validate(cmd, search_path, options) => report::report_action_issues(
            actions::validate_file(cmd, search_path)?,
            options.use_color,
        ),
        Command::Convert(cmd, search_path, _) => actions::convert_file_format(cmd, search_path),
        Command::Document(cmd, search_path, _) => actions::document_file(cmd, search_path),
    };

    if result.is_err() {
        eprintln!("{}", (&result).as_ref().err().unwrap().source().unwrap())
    }

    Ok(())
}
