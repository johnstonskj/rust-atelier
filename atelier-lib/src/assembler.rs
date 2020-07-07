/*!
Provides the model assembly capability, to merge files into a single in-memory `Model`.

# Example

TBD

*/

use crate::core::error::{Error, ErrorKind};
#[cfg(feature = "json")]
use crate::format::json;
#[cfg(feature = "smithy")]
use crate::format::smithy;
use atelier_core::io::{ModelReader, ModelWriter};
use atelier_core::model::Model;
use std::collections::HashSet;
use std::convert::TryInto;
use std::fs::{read_dir, File};
use std::path::PathBuf;

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
            extensions: [
                #[cfg(feature = "json")]
                json::io::FILE_EXTENSION,
                #[cfg(feature = "smithy")]
                smithy::io::FILE_EXTENSION,
            ]
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
        let models: Result<Vec<Model>, Error> = self
            .file_names
            .iter()
            .map(|file_name| ModelAssembler::read_model_from_file(&file_name))
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
    pub fn add_file(&mut self, file_name: PathBuf) -> Result<(), Error> {
        if file_name.is_file() && file_name.exists() && file_name.extension().is_some() {
            let extension = file_name.extension().unwrap();
            let extension = extension.to_string_lossy();
            if self.extensions.contains(extension.as_ref()) {
                self.file_names.insert(file_name);
            } else {
                return Err(ErrorKind::InvalidRepresentation(extension.to_string()).into());
            }
        }
        Ok(())
    }

    ///
    /// Add all files with known file extensions to the assembler for later processing.
    ///
    pub fn add_files_in(&mut self, dir_name: PathBuf) -> Result<(), Error> {
        if dir_name.is_dir() && dir_name.exists() {
            for entry in read_dir(dir_name)? {
                let entry = entry?;
                self.add_file(entry.path())?;
            }
        }
        Ok(())
    }

    ///
    /// Read a model from a file, this will only process a single file at a time.
    ///
    pub fn read_model_from_file(path: &PathBuf) -> Result<Model, Error> {
        match path.extension() {
            None => Err(ErrorKind::InvalidRepresentation("unknown".to_string()).into()),
            Some(ext) => {
                let ext = ext.to_string_lossy();
                let mut file = File::open(path).unwrap();

                match ext.as_ref() {
                    #[cfg(feature = "json")]
                    json::io::FILE_EXTENSION => {
                        let mut reader = json::io::JsonReader::default();
                        reader.read(&mut file)
                    }
                    #[cfg(feature = "smithy")]
                    smithy::io::FILE_EXTENSION => {
                        let mut reader = smithy::io::SmithyReader::default();
                        reader.read(&mut file)
                    }
                    _ => {
                        // Peek at file, is it JSON?
                        Err(ErrorKind::InvalidRepresentation("unknown".to_string()).into())
                    }
                }
            }
        }
    }

    ///
    /// Write a model to a file, this will only process a single file at a time.
    ///
    pub fn write_model_to_file(path: &PathBuf, model: &Model) -> Result<(), Error> {
        match path.extension() {
            None => Err(ErrorKind::InvalidRepresentation("unknown".to_string()).into()),
            Some(ext) => {
                let ext = ext.to_string_lossy();
                let mut file = File::open(path).unwrap();

                match ext.as_ref() {
                    #[cfg(feature = "json")]
                    json::io::FILE_EXTENSION => {
                        let mut writer = json::io::JsonWriter::default();
                        writer.write(&mut file, model)
                    }
                    #[cfg(feature = "smithy")]
                    smithy::io::FILE_EXTENSION => {
                        let mut writer = smithy::io::SmithyWriter::default();
                        writer.write(&mut file, model)
                    }
                    crate::core::io::plant_uml::FILE_EXTENSION => {
                        let mut writer = crate::core::io::plant_uml::PlantUmlWriter::default();
                        writer.write(&mut file, model)
                    }
                    _ => Err(ErrorKind::InvalidRepresentation("unknown".to_string()).into()),
                }
            }
        }
    }
}
