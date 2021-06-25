use crate::{Command, DocumentCommand, FileFormat, Options, TransformCommand};
use somedoc::write::OutputFormat;
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
        in_file: Vec<PathBuf>,
    },
    /// Run standard validators on a model file
    Validate {
        /// The file to read [default: <stdin>]
        #[structopt(long, short)]
        in_file: Vec<PathBuf>,
    },
    /// Convert model from one representation to another
    Convert {
        /// The file to read [default: <stdin>]
        #[structopt(long, short)]
        in_file: Vec<PathBuf>,

        /// The file to write to [default: <stdout>]
        #[structopt(long, short)]
        out_file: Option<PathBuf>,

        /// The representation of the output file
        #[structopt(short, long, default_value = "json")]
        write_format: FileFormat,

        /// The namespace to write, if a format is constrained to one
        #[structopt(short, long)]
        namespace: Option<String>,
    },
    /// Create human-readable documentation from a model
    Document {
        /// The file to read [default: <stdin>]
        #[structopt(long, short)]
        in_file: Vec<PathBuf>,

        /// The file to write to [default: <stdout>]
        #[structopt(long, short)]
        out_file: Option<PathBuf>,

        /// The documentation format supported by the `somedoc` crate
        #[structopt(short, long, default_value = "markdown")]
        write_format: OutputFormat,
    },
}

#[derive(Debug)]
pub struct CommandLineError {}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn parse() -> Result<Command, Box<dyn Error>> {
    let args = CommandLine::from_args();

    let options = Options {
        use_color: !args.no_color,
    };

    match args.cmd {
        SubCommand::Lint { in_file } => Ok(Command::Lint(in_file, options)),
        SubCommand::Validate { in_file } => Ok(Command::Validate(in_file, options)),
        SubCommand::Convert {
            in_file,
            out_file,
            write_format,
            namespace,
        } => Ok(Command::Convert(
            TransformCommand {
                input_files: in_file,
                output_file: out_file,
                output_format: write_format,
                namespace,
            },
            options,
        )),
        SubCommand::Document {
            in_file,
            out_file,
            write_format,
        } => Ok(Command::Document(
            DocumentCommand {
                input_files: in_file,
                output_file: out_file,
                output_format: write_format,
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
