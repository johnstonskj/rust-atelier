use crate::{Command, DocumentCommand, FileFormat, Options, TransformCommand};
use search_path::SearchPath;
use somedoc::write::OutputFormat;
use std::error::Error;
use std::path::PathBuf;
use structopt::StructOpt;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, StructOpt)]
#[structopt(name = "cargo-atelier", about = "Tools for the Smithy IDL.")]
struct CommandLine {
    /// The level of logging to perform; from off to trace
    #[structopt(long, short = "v", parse(from_occurrences))]
    verbose: i8,

    #[cfg(feature = "color")]
    /// Turn off color in the output
    #[structopt(long)]
    no_color: bool,

    #[structopt(subcommand)]
    cmd: SubCommand,
}

#[derive(Debug, StructOpt)]
struct FileInput {
    /// A file, or directory containing files, to read.
    #[structopt(long, short)]
    in_file: Vec<PathBuf>,

    /// If set, the standard SMITHY_PATH environment variable is used as a search path.
    #[structopt(long, short, conflicts_with = "search_env")]
    default_search_env: bool,

    /// The name of an environment variable to use as a search path.
    #[structopt(long, short, conflicts_with = "default_search_env")]
    search_env: Option<String>,
}

#[derive(Debug, StructOpt)]
enum SubCommand {
    /// Run standard linter rules on a model file
    Lint {
        #[structopt(flatten)]
        file_input: FileInput,
    },
    /// Run standard validators on a model file
    Validate {
        #[structopt(flatten)]
        file_input: FileInput,
    },
    /// Convert model from one representation to another
    Convert {
        #[structopt(flatten)]
        file_input: FileInput,

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
        #[structopt(flatten)]
        file_input: FileInput,

        /// The file to write to [default: <stdout>]
        #[structopt(long, short)]
        out_file: Option<PathBuf>,

        /// The documentation format supported by the `somedoc` crate
        #[structopt(short, long, default_value = "markdown")]
        write_format: OutputFormat,
    },
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn parse() -> Result<Command, Box<dyn Error>> {
    let args = CommandLine::from_iter(check_command_line());

    let options = Options {
        use_color: !args.no_color,
    };

    match args.cmd {
        SubCommand::Lint {
            file_input:
                FileInput {
                    in_file,
                    default_search_env,
                    search_env,
                },
        } => Ok(Command::Lint(
            in_file,
            make_search_path(default_search_env, search_env),
            options,
        )),
        SubCommand::Validate {
            file_input:
                FileInput {
                    in_file,
                    default_search_env,
                    search_env,
                },
        } => Ok(Command::Validate(
            in_file,
            make_search_path(default_search_env, search_env),
            options,
        )),
        SubCommand::Convert {
            file_input:
                FileInput {
                    in_file,
                    default_search_env,
                    search_env,
                },
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
            make_search_path(default_search_env, search_env),
            options,
        )),
        SubCommand::Document {
            file_input:
                FileInput {
                    in_file,
                    default_search_env,
                    search_env,
                },
            out_file,
            write_format,
        } => Ok(Command::Document(
            DocumentCommand {
                input_files: in_file,
                output_file: out_file,
                output_format: write_format,
            },
            make_search_path(default_search_env, search_env),
            options,
        )),
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn check_command_line() -> Vec<String> {
    use std::env;
    let mut args = env::args().collect::<Vec<String>>();

    if let Some(command) = args.get(1) {
        // The following is true if this is run as a cargo sub-command.
        if command == "atelier" {
            args.remove(1);
        }
    }
    args
}

fn make_search_path(default_search_env: bool, search_env: Option<String>) -> Option<SearchPath> {
    if default_search_env {
        Some(SearchPath::default())
    } else {
        search_env.map(|search_env| SearchPath::new_or_default(&search_env))
    }
}
