/*!
One-line description.

More detailed description, with

# Example

*/

use crate::{Command, File, FileCommand, FileFormat, Options, TransformCommand};
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

    #[cfg(feature = "color")]
    /// Turn off color in the output
    #[structopt(long, short)]
    no_color: bool,

    #[structopt(subcommand)]
    cmd: SubCommand,
}

#[derive(Debug, StructOpt)]
pub(crate) enum SubCommand {
    /// Run standard linter rules on a model file
    Lint {
        /// The file to read [default: <stdin>]
        #[structopt(long, short)]
        in_file: Option<PathBuf>,

        /// The representation of the input file
        #[structopt(short, long, default_value = "smithy")]
        read_format: FileFormat,
    },
    /// Run standard validators on a model file
    Validate {
        /// The file to read [default: <stdin>]
        #[structopt(long, short)]
        in_file: Option<PathBuf>,

        /// The representation of the input file,
        #[structopt(short, long, default_value = "smithy")]
        read_format: FileFormat,
    },
    /// Convert model from one representation to another
    Convert {
        /// The file to read [default: <stdin>]
        #[structopt(long, short)]
        in_file: Option<PathBuf>,

        /// The representation of the input file
        #[structopt(short, long, default_value = "smithy")]
        read_format: FileFormat,

        /// The file to write to [default: <stdout>]
        #[structopt(long, short)]
        out_file: Option<PathBuf>,

        /// The representation of the output file
        #[structopt(short, long, default_value = "json")]
        write_format: FileFormat,
    },
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

    let options = Options {
        use_color: !args.no_color,
    };

    match args.cmd {
        SubCommand::Lint {
            in_file,
            read_format,
        } => Ok(Command::Lint(
            FileCommand {
                input_file: File {
                    file_name: in_file,
                    format: read_format,
                },
            },
            options,
        )),
        SubCommand::Validate {
            in_file,
            read_format,
        } => Ok(Command::Validate(
            FileCommand {
                input_file: File {
                    file_name: in_file,
                    format: read_format,
                },
            },
            options,
        )),
        SubCommand::Convert {
            in_file,
            read_format,
            out_file,
            write_format,
        } => Ok(Command::Convert(
            TransformCommand {
                input_file: File {
                    file_name: in_file,
                    format: read_format,
                },
                output_file: File {
                    file_name: out_file,
                    format: write_format,
                },
            },
            options,
        )),
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
