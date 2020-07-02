/*!
One-line description.

More detailed description, with

# Example

*/

use crate::{Command, File, FileCommand, FileFormat};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use structopt::StructOpt;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, StructOpt)]
#[structopt(name = "cargo-atelier", about = "Tools for the Smithy IDL.")]
pub(crate) struct CommandLine {
    /// The level of logging to perform; from off to trace
    #[structopt(long, short = "v", parse(from_occurrences))]
    verbose: i8,

    #[structopt(subcommand)]
    cmd: SubCommand,
}

#[derive(Debug, StructOpt)]
pub(crate) enum SubCommand {
    /// Run standard linter rules on a model file
    Lint {
        /// The file to read, or <stdin>
        #[structopt(long, short)]
        in_file: Option<PathBuf>,

        /// The representation of the input file, the default is 'smithy'
        #[structopt(short, long, default_value = "smithy")]
        format: FileFormat,
    },
    /// Run standard validators on a model file
    Validate {
        /// The file to read, or <stdin>
        #[structopt(long, short)]
        in_file: Option<PathBuf>,

        /// The representation of the input file, must be present if no file specified
        #[structopt(short, long, default_value = "smithy")]
        format: FileFormat,
    },
    /// Convert file from one format to another
    Convert,
}

#[derive(Debug)]
pub struct CommandLineError {}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn parse() -> Result<Command, Box<dyn Error>> {
    let args = CommandLine::from_args();

    match args.cmd {
        SubCommand::Lint { in_file, format } => Ok(Command::Lint(FileCommand {
            input_file: File {
                file_name: in_file,
                format,
            },
        })),
        SubCommand::Validate { in_file, format } => Ok(Command::Validate(FileCommand {
            input_file: File {
                file_name: in_file,
                format,
            },
        })),
        _ => Err(CommandLineError::boxed()),
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for CommandLineError {
    fn default() -> Self {
        Self {}
    }
}

impl Display for CommandLineError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "An error occurred processing command line options")
    }
}

impl Error for CommandLineError {}

impl CommandLineError {
    pub fn boxed() -> Box<Self> {
        Box::new(Self::default())
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
