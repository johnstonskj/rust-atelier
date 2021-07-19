use crate::{DocumentCommand, FileFormat, TransformCommand};
use atelier_lib::actions::{standard_model_lint, standard_model_validation};
use atelier_lib::assembler::{FileTypeRegistry, ModelAssembler};
use atelier_lib::core::action::ActionIssue;
use atelier_lib::core::error::Result as ModelResult;
use atelier_lib::core::io::ModelWriter;
use atelier_lib::core::model::{Model, NamespaceID};
use atelier_lib::format::document::writer::describe_model;
use atelier_lib::format::json::JsonWriter;
use atelier_lib::format::plant_uml::writer::PlantUmlWriter;
use atelier_lib::format::smithy::SmithyWriter;
use search_path::SearchPath;
use somedoc::write::{write_document, OutputFormat};
use std::convert::TryFrom;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn lint_file(
    paths: Vec<PathBuf>,
    search_path: Option<SearchPath>,
) -> ModelResult<Vec<ActionIssue>> {
    standard_model_lint(&assemble_model(paths, search_path)?, false)
}

pub fn validate_file(
    paths: Vec<PathBuf>,
    search_path: Option<SearchPath>,
) -> ModelResult<Vec<ActionIssue>> {
    standard_model_validation(&assemble_model(paths, search_path)?, false)
}

pub fn convert_file_format(
    cmd: TransformCommand,
    search_path: Option<SearchPath>,
) -> Result<(), Box<dyn Error>> {
    transform_file(
        &assemble_model(cmd.input_files, search_path)?,
        cmd.output_file,
        cmd.output_format,
        cmd.namespace,
        None,
    )
}

pub fn document_file(
    cmd: DocumentCommand,
    search_path: Option<SearchPath>,
) -> Result<(), Box<dyn Error>> {
    document_a_file(
        &assemble_model(cmd.input_files, search_path)?,
        cmd.output_file,
        cmd.output_format,
    )
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn assemble_model(
    input_files: Vec<PathBuf>,
    search_path: Option<SearchPath>,
) -> ModelResult<Model> {
    let mut assembler = ModelAssembler::new(FileTypeRegistry::default(), search_path);
    input_files.iter().for_each(|pb| {
        assembler.push(pb);
    });
    Model::try_from(assembler)
}

fn write_model(
    model: &Model,
    output_file: Option<PathBuf>,
    output_format: FileFormat,
    namespace: Option<String>,
) -> Result<(), Box<dyn Error>> {
    let mut file: Box<dyn Write> = match output_file {
        None => Box::new(std::io::stdout()),
        Some(file_name) => Box::new(File::open(file_name)?),
    };

    match output_format {
        FileFormat::Json => write_json(&mut file, model),
        FileFormat::Smithy => {
            if let Some(namespace) = namespace {
                write_smithy(&mut file, model, NamespaceID::from_str(&namespace).unwrap())
            } else {
                let namespaces = &model.namespaces();
                if namespaces.len() == 1 {
                    write_smithy(
                        &mut file,
                        model,
                        (*namespaces.iter().next().unwrap()).clone(),
                    )
                } else {
                    panic!("A namespace value is required for writing Smithy IDL");
                }
            }
        }
        FileFormat::Uml => write_uml(&mut file, model),
    }
}

fn transform_file(
    input_model: &Model,
    output_file: Option<PathBuf>,
    output_format: FileFormat,
    namespace: Option<String>,
    transform_fn: Option<&dyn Fn(&Model) -> Result<Model, Box<dyn Error>>>,
) -> Result<(), Box<dyn Error>> {
    if let Some(transform_fn) = transform_fn {
        write_model(
            &transform_fn(input_model)?,
            output_file,
            output_format,
            namespace,
        )
    } else {
        write_model(input_model, output_file, output_format, namespace)
    }
}

fn document_a_file(
    input_model: &Model,
    output_file: Option<PathBuf>,
    output_format: OutputFormat,
) -> Result<(), Box<dyn Error>> {
    let document = describe_model(&input_model)?;

    let mut file: Box<dyn Write> = match output_file {
        None => Box::new(std::io::stdout()),
        Some(file_name) => Box::new(File::open(file_name)?),
    };
    write_document(&document, output_format, &mut file)?;
    Ok(())
}

fn write_json(w: &mut impl Write, model: &Model) -> Result<(), Box<dyn Error>> {
    let mut writer = JsonWriter::new(true);
    writer.write(w, &model)?;
    Ok(())
}

fn write_smithy(
    w: &mut impl Write,
    model: &Model,
    namespace: NamespaceID,
) -> Result<(), Box<dyn Error>> {
    let mut writer = SmithyWriter::new(namespace);
    writer.write(w, &model)?;
    Ok(())
}

fn write_uml(w: &mut impl Write, model: &Model) -> Result<(), Box<dyn Error>> {
    let mut writer = PlantUmlWriter::default();
    writer.write(w, &model)?;
    Ok(())
}
