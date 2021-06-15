use crate::{
    DocumentCommand, File as FileArg, FileFormat, Files as FilesArg, MultiFileCommand,
    TransformCommand,
};
use atelier_lib::actions::{standard_model_lint, standard_model_validation};
use atelier_lib::core::action::ActionIssue;
use atelier_lib::core::error::{Error as ModelError, ErrorKind, Result as ModelResult};
use atelier_lib::core::io::read_model_from_string;
use atelier_lib::core::io::ModelWriter;
use atelier_lib::core::model::{Model, NamespaceID};
use atelier_lib::format::document::writer::describe_model;
use atelier_lib::format::json::{JsonReader, JsonWriter};
use atelier_lib::format::plant_uml::writer::PlantUmlWriter;
use atelier_lib::format::smithy::{SmithyReader, SmithyWriter};
//use somedoc::error::ErrorKind;
use somedoc::write::{write_document, OutputFormat};
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn lint_file(cmd: MultiFileCommand) -> ModelResult<Vec<ActionIssue>> {
    standard_model_lint(&read_model(cmd.input_files)?, false)
}

pub fn validate_file(cmd: MultiFileCommand) -> ModelResult<Vec<ActionIssue>> {
    standard_model_validation(&read_model(cmd.input_files)?, false)
}

pub fn convert_file_format(cmd: TransformCommand) -> Result<(), Box<dyn Error>> {
    transform_file(cmd.input_files, cmd.output_file, cmd.namespace, None)
}

pub fn document_file(cmd: DocumentCommand) -> Result<(), Box<dyn Error>> {
    document_a_file(cmd.input_files, cmd.output_file, cmd.output_format)
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn read_model(input: FilesArg) -> ModelResult<Model> {
    let is_multi = input.file_names.len() > 1;
    // validate input parameters
    match (&input.format, is_multi) {
        (FileFormat::Smithy, _) | (FileFormat::Json, false) => {}
        (FileFormat::Json, true) => {
            return Err(ErrorKind::InvalidRepresentation(
                "json input format accepts no more than one input file".to_string(),
            )
            .into());
        }
        _ => {
            return Err(ErrorKind::InvalidRepresentation("read".to_string()).into());
        }
    };
    let content = if input.file_names.is_empty() {
        let mut content: String = String::default();
        let _ = std::io::stdin().read_to_string(&mut content)?;
        vec![content]
    } else {
        let mut files_data = Vec::new();
        for path in input.file_names.iter() {
            //let c = std::fs::read_to_string(path);
            files_data.push(std::fs::read_to_string(path)?);
        }
        files_data
    };
    match &input.format {
        FileFormat::Smithy => read_strings(content),
        FileFormat::Json => read_json(content.get(0).unwrap()),
        _ => Err(ErrorKind::InvalidRepresentation("read".to_string()).into()),
    }
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
        FileFormat::Smithy => match namespace {
            Some(namespace) => write_smithy(&mut file, model, NamespaceID::from_str(&namespace)?),
            None => Err(Box::new(atelier_lib::core::action::ActionIssue::error(
                "convert",
                "conversion from json requires a namespace selector",
            ))),
        },
        FileFormat::Uml => write_uml(&mut file, model),
    }
}

fn transform_file(
    input: FilesArg,
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
    input: FilesArg,
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

fn read_json(content: &String) -> Result<Model, ModelError> {
    read_model_from_string(&mut JsonReader::default(), content)
}

fn read_strings(content: Vec<String>) -> Result<Model, ModelError> {
    let mut r = SmithyReader::default();
    r.merge(content)
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
