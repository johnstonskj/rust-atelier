/*!
One-line description.

More detailed description, with

# Example

*/

use crate::{FileCommand, FileFormat, TransformCommand};
use atelier_lib::action::{standard_model_lint, standard_model_validation};
use atelier_lib::core::action::ActionIssue;
use atelier_lib::core::error::{Error as ModelError, ErrorKind};
use atelier_lib::core::io::plant_uml::PlantUmlWriter;
use atelier_lib::core::io::read_model_from_string;
use atelier_lib::core::io::ModelWriter;
use atelier_lib::core::model::Model;
use atelier_lib::format::json::io::{JsonReader, JsonWriter};
use atelier_lib::format::smithy::io::{SmithyReader, SmithyWriter};
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};

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

pub fn convert_file_format(cmd: TransformCommand) -> Result<(), Box<dyn Error>> {
    transform_file(cmd, None)
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

pub fn transform_file(
    cmd: TransformCommand,
    transform_fn: Option<&dyn Fn(Model) -> Result<Model, Box<dyn Error>>>,
) -> Result<(), Box<dyn Error>> {
    fn read_json(content: Vec<u8>) -> Result<Model, ModelError> {
        read_model_from_string(&mut JsonReader::default(), content)
    }
    fn read_smithy(content: Vec<u8>) -> Result<Model, ModelError> {
        read_model_from_string(&mut SmithyReader::default(), content)
    }
    fn write_json(w: &mut impl Write, model: Model) -> Result<(), Box<dyn Error>> {
        let mut writer = JsonWriter::new(true);
        writer.write(w, &model)?;
        Ok(())
    }
    fn write_smithy(w: &mut impl Write, model: Model) -> Result<(), Box<dyn Error>> {
        let mut writer = SmithyWriter::default();
        writer.write(w, &model)?;
        Ok(())
    }
    fn write_uml(w: &mut impl Write, model: Model) -> Result<(), Box<dyn Error>> {
        let mut writer = PlantUmlWriter::default();
        writer.write(w, &model)?;
        Ok(())
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

    let model = reader(content)?;
    let model = if let Some(transform_fn) = transform_fn {
        transform_fn(model)?
    } else {
        model
    };

    let mut file: Box<dyn Write> = match cmd.output_file.file_name {
        None => Box::new(std::io::stdout()),
        Some(file_name) => Box::new(File::open(file_name)?),
    };

    match cmd.output_file.format {
        FileFormat::Json => write_json(&mut file, model),
        FileFormat::Smithy => write_smithy(&mut file, model),
        FileFormat::Uml => write_uml(&mut file, model),
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
