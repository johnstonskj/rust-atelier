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
    Lint(FileCommand, Options),
    Validate(FileCommand, Options),
    Convert(TransformCommand, Options),
}

#[derive(Debug)]
pub enum FileFormat {
    Json,
    Smithy,
    Uml,
    Documentation,
}

#[derive(Debug)]
pub struct File {
    pub file_name: Option<PathBuf>,
    pub format: FileFormat,
}

#[derive(Debug)]
pub struct FileCommand {
    pub input_file: File,
}

#[derive(Debug)]
pub struct TransformCommand {
    pub input_file: File,
    pub output_file: File,
    pub namespace: Option<String>,
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
                FileFormat::Documentation => "doc",
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
            "doc" => Ok(FileFormat::Documentation),
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
