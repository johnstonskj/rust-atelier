/*!
This crate provides the model assembly capability, to merge files into a single in-memory `Model`.

A tool can add files one-by-one, or from a directory, and then process them all into a single model.
This implementation understands the different registered file extensions so that it can read files
in different representations and assemble them seamlessly.

For more information, see [the Rust Atelier book](https://rust-atelier.dev/using/assembly.html).

# Example

TBD

*/

#![warn(
    // ---------- Stylistic
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    // ---------- Public
    missing_debug_implementations,
    missing_docs,
    unreachable_pub,
    // ---------- Unsafe
    unsafe_code,
    // ---------- Unused
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
)]

use atelier_core::error::{Error, ErrorKind};
use atelier_core::io::ModelReader;
use atelier_core::model::Model;
use atelier_json as json;
use atelier_smithy as smithy;
use std::collections::HashSet;
use std::convert::TryInto;
use std::fs::{read_dir, File};
use std::path::{Path, PathBuf};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Assemble a single model by merging the sub-models represented by one or more files.
///
#[derive(Debug)]
pub struct ModelAssembler {
    extensions: HashSet<String>,
    file_names: HashSet<PathBuf>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for ModelAssembler {
    fn default() -> Self {
        Self {
            extensions: [json::FILE_EXTENSION, smithy::FILE_EXTENSION]
                .iter()
                .map(|s| s.to_string())
                .collect::<HashSet<String>>(),
            file_names: Default::default(),
        }
    }
}

impl TryInto<Model> for ModelAssembler {
    type Error = Error;

    fn try_into(self) -> Result<Model, Self::Error> {
        assert!(!self.file_names.is_empty());
        let models: Result<Vec<Model>, Self::Error> = self
            .file_names
            .iter()
            .map(|file_name| read_model_from_file(&file_name))
            .collect();
        match models {
            Ok(mut models) => {
                let mut merged = models.remove(0);
                for other in models {
                    merged.merge(other)?;
                }
                Ok(merged)
            }
            Err(err) => Err(err),
        }
    }
}

impl ModelAssembler {
    ///
    /// Add a single file path to the assembler for later processing.
    ///
    pub fn add_file(&mut self, file_name: &Path) -> Result<(), Error> {
        if file_name.is_file() && file_name.exists() && file_name.extension().is_some() {
            let extension = file_name.extension().unwrap();
            let extension = extension.to_string_lossy();
            if self.extensions.contains(extension.as_ref()) {
                let _ = self.file_names.insert(PathBuf::from(file_name));
            } else {
                return Err(ErrorKind::InvalidRepresentation(extension.to_string()).into());
            }
        }
        Ok(())
    }

    ///
    /// Add all files with known file extensions to the assembler for later processing.
    ///
    pub fn add_files_in(&mut self, dir_name: &Path) -> Result<(), Error> {
        if dir_name.is_dir() && dir_name.exists() {
            for entry in read_dir(dir_name)? {
                let entry = entry?;
                self.add_file(&entry.path())?;
            }
        }
        Ok(())
    }
}

fn read_model_from_file(path: &Path) -> Result<Model, Error> {
    match path.extension() {
        Some(ext) => {
            let ext = ext.to_string_lossy();
            let mut file = File::open(path).unwrap();

            match ext.as_ref() {
                json::FILE_EXTENSION => {
                    let mut reader = json::JsonReader::default();
                    reader.read(&mut file)
                }
                smithy::FILE_EXTENSION => {
                    let mut reader = smithy::SmithyReader::default();
                    reader.read(&mut file)
                }
                _ => {
                    // Peek at file, is it JSON?
                    Err(ErrorKind::InvalidRepresentation("unknown".to_string()).into())
                }
            }
        }
        _ => Err(ErrorKind::InvalidRepresentation("unknown".to_string()).into()),
    }
}
