use search_path::SearchPath;
use somedoc::write::OutputFormat;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct Options {
    pub use_color: bool,
}

#[derive(Debug)]
pub enum Command {
    Lint(Vec<PathBuf>, Option<SearchPath>, Options),
    Validate(Vec<PathBuf>, Option<SearchPath>, Options),
    Convert(TransformCommand, Option<SearchPath>, Options),
    Document(DocumentCommand, Option<SearchPath>, Options),
}

#[derive(Debug)]
pub enum FileFormat {
    Json,
    Smithy,
    Uml,
}

#[derive(Debug)]
pub struct TransformCommand {
    pub input_files: Vec<PathBuf>,
    pub output_file: Option<PathBuf>,
    pub output_format: FileFormat,
    pub namespace: Option<String>,
}

#[derive(Debug)]
pub struct DocumentCommand {
    pub input_files: Vec<PathBuf>,
    pub output_file: Option<PathBuf>,
    pub output_format: OutputFormat,
}

#[derive(Debug)]
pub struct FormatStringError {
    failed: String,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for FileFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FileFormat::Json => "json",
                FileFormat::Smithy => "smithy",
                FileFormat::Uml => "uml",
            }
        )
    }
}

impl FromStr for FileFormat {
    type Err = FormatStringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(FileFormat::Json),
            "smithy" => Ok(FileFormat::Smithy),
            "uml" => Ok(FileFormat::Uml),
            _ => Err(FormatStringError::new(s)),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for FormatStringError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Value '{}' is not a valid format", self.failed)
    }
}

impl Error for FormatStringError {}

impl FormatStringError {
    pub fn new(failed: &str) -> Self {
        Self {
            failed: failed.to_string(),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod actions;

pub mod command_line;

pub mod report;
