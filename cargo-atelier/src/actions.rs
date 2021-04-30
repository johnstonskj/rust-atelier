use crate::{DocumentCommand, File as FileArg, FileCommand, FileFormat, TransformCommand};
use atelier_lib::action::{standard_model_lint, standard_model_validation};
use atelier_lib::core::action::ActionIssue;
use atelier_lib::core::error::{Error as ModelError, ErrorKind, Result as ModelResult};
use atelier_lib::core::io::read_model_from_string;
use atelier_lib::core::io::ModelWriter;
use atelier_lib::core::model::{Model, NamespaceID};
use atelier_lib::format::document::writer::describe_model;
use atelier_lib::format::json::{JsonReader, JsonWriter};
use atelier_lib::format::plant_uml::writer::PlantUmlWriter;
use atelier_lib::format::smithy::{SmithyReader, SmithyWriter};
use somedoc::write::{write_document, OutputFormat};
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn lint_file(cmd: FileCommand) -> ModelResult<Vec<ActionIssue>> {
    standard_model_lint(&read_model(cmd.input_file)?, false)
}

pub fn validate_file(cmd: FileCommand) -> ModelResult<Vec<ActionIssue>> {
    standard_model_validation(&read_model(cmd.input_file)?, false)
}

pub fn convert_file_format(cmd: TransformCommand) -> Result<(), Box<dyn Error>> {
    transform_file(cmd.input_file, cmd.output_file, cmd.namespace, None)
}

pub fn document_file(cmd: DocumentCommand) -> Result<(), Box<dyn Error>> {
    document_a_file(cmd.input_file, cmd.output_file, cmd.output_format)
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn read_model(input: FileArg) -> ModelResult<Model> {
    let reader = match input.format {
        FileFormat::Json => read_json,
        FileFormat::Smithy => read_smithy,
        _ => {
            return Err(ErrorKind::InvalidRepresentation("read".to_string()).into());
        }
    };
    let mut file: Box<dyn Read> = match input.file_name {
        None => Box::new(std::io::stdin()),
        Some(file_name) => Box::new(File::open(file_name)?),
    };
    let mut content: Vec<u8> = Vec::default();
    let _ = file.read_to_end(&mut content).unwrap();
    reader(content)
}

fn write_model(
    model: Model,
    output: FileArg,
    namespace: Option<String>,
) -> Result<(), Box<dyn Error>> {
    let mut file: Box<dyn Write> = match output.file_name {
        None => Box::new(std::io::stdout()),
        Some(file_name) => Box::new(File::open(file_name)?),
    };

    match output.format {
        FileFormat::Json => write_json(&mut file, model),
        FileFormat::Smithy => write_smithy(
            &mut file,
            model,
            NamespaceID::from_str(&namespace.unwrap()).unwrap(),
        ),
        FileFormat::Uml => write_uml(&mut file, model),
    }
}

fn transform_file(
    input: FileArg,
    output: FileArg,
    namespace: Option<String>,
    transform_fn: Option<&dyn Fn(Model) -> Result<Model, Box<dyn Error>>>,
) -> Result<(), Box<dyn Error>> {
    let model = read_model(input)?;

    let model = if let Some(transform_fn) = transform_fn {
        transform_fn(model)?
    } else {
        model
    };

    write_model(model, output, namespace)
}

fn document_a_file(
    input: FileArg,
    output: FileArg,
    format: OutputFormat,
) -> Result<(), Box<dyn Error>> {
    let model = read_model(input)?;
    let document = describe_model(&model)?;

    let mut file: Box<dyn Write> = match output.file_name {
        None => Box::new(std::io::stdout()),
        Some(file_name) => Box::new(File::open(file_name)?),
    };
    write_document(&document, format, &mut file)?;
    Ok(())
}

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

fn write_smithy(
    w: &mut impl Write,
    model: Model,
    namespace: NamespaceID,
) -> Result<(), Box<dyn Error>> {
    let mut writer = SmithyWriter::new(namespace);
    writer.write(w, &model)?;
    Ok(())
}

fn write_uml(w: &mut impl Write, model: Model) -> Result<(), Box<dyn Error>> {
    let mut writer = PlantUmlWriter::default();
    writer.write(w, &model)?;
    Ok(())
}
