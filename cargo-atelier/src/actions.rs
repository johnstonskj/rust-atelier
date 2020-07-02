/*!
One-line description.

More detailed description, with

# Example

*/

use crate::{FileCommand, FileFormat, TransformCommand};
use atelier_lib::action::{standard_model_lint, standard_model_validation};
use atelier_lib::core::action::ActionIssue;
use atelier_lib::core::error::{Error as ModelError, ErrorKind};
use atelier_lib::core::io::read_model_from_string;
use atelier_lib::core::model::Model;
use atelier_lib::format::json::io::JsonReader;
use atelier_lib::format::smithy::io::SmithyReader;
use std::error::Error;
use std::fs::File;
use std::io::Read;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn lint_file(cmd: FileCommand) -> Result<Option<Vec<ActionIssue>>, Box<dyn Error>> {
    Ok(check_file(cmd, &standard_model_lint)?)
}

pub fn validate_file(cmd: FileCommand) -> Result<Option<Vec<ActionIssue>>, Box<dyn Error>> {
    Ok(check_file(cmd, &standard_model_validation)?)
}

pub fn convert_file_format(
    _cmd: TransformCommand,
) -> Result<Option<Vec<ActionIssue>>, Box<dyn Error>> {
    Ok(None)
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

pub fn check_file(
    cmd: FileCommand,
    check_fn: &dyn Fn(&Model, bool) -> Option<Vec<ActionIssue>>,
) -> Result<Option<Vec<ActionIssue>>, Box<dyn Error>> {
    fn read_json(content: Vec<u8>) -> Result<Model, ModelError> {
        read_model_from_string(&mut JsonReader::default(), content)
    }
    fn read_smithy(content: Vec<u8>) -> Result<Model, ModelError> {
        read_model_from_string(&mut SmithyReader::default(), content)
    }
    let err: ModelError = ErrorKind::InvalidRepresentation("read".to_string()).into();

    let reader = match cmd.input_file.format {
        FileFormat::Json => read_json,
        FileFormat::Smithy => read_smithy,
        _ => {
            return Err(Box::new(err));
        }
    };
    let mut file: Box<dyn Read> = match cmd.input_file.file_name {
        None => Box::new(std::io::stdin()),
        Some(file_name) => Box::new(File::open(file_name)?),
    };
    let mut content: Vec<u8> = Vec::default();
    let _ = file.read_to_end(&mut content).unwrap();

    Ok(check_fn(&reader(content)?, false))
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
