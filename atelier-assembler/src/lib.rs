/*!
This crate provides the model assembly capability, to merge files into a single in-memory `Model`.

A tool can add files one-by-one, or from a directory, and then process them all into a single model.
This implementation understands the different registered file extensions so that it can read files
in different representations and assemble them seamlessly.

Note that if a path processed by the assembler is a directory then all files in the directory will
be added **recursively**.

# Example

The following is the simple, and most common, method of using the assembler. This uses the
default `FileTypeRegistry` and will search for all models in the set of paths specified in
the environment variable "`SMITHY_PATH`".

```rust
use atelier_assembler::ModelAssembler;
use atelier_core::error::Result;
use atelier_core::model::Model;
use std::convert::TryFrom;

// The default constructor will load all paths from the `SMITHY_PATH` environment variable.
let env_assembler = ModelAssembler::default();

let model: Result<Model> = Model::try_from(env_assembler);
```

For more information, see [the Rust Atelier book](https://rust-atelier.dev/using/assembly.html).

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

#[macro_use]
extern crate log;

use atelier_core::error::{Error, ErrorKind, Result};
use atelier_core::io::ModelReader;
use atelier_core::model::Model;
use atelier_json as json;
use atelier_smithy as smithy;
use search_path::SearchPath;
use std::collections::{BTreeMap, HashSet};
use std::convert::TryFrom;
use std::fmt::{Debug, Display, Formatter};
use std::fs::{read_dir, File};
use std::path::{Path, PathBuf};
use std::rc::Rc;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A Function type used to read a particular format into a model. This type is used by the
/// [`FileType`](struct.FileType.html).
///
pub type FileReader = fn(&mut File) -> Result<Model>;

///
/// A File type, this has a display name and a reader function. The [`FileTypeRegistry`](struct.FileTypeRegistry.html)
/// uses these to map one ot more file _types_ to reader functions ([`FileReader`](type.FileReader.html)).
///
#[derive(Clone)]
pub struct FileType {
    display_name: String,
    mime_type: Option<String>,
    reader_fn: FileReader,
}

///
/// A mapping from file extension to file type. Note that `FileTypeRegistry::default` will
/// always contain *at least* mappings for ".json" and ".smithy" file types. Note that file
/// extensions will always be compared in a case insensitive manner.
///
#[derive(Clone)]
pub struct FileTypeRegistry {
    by_extension: BTreeMap<String, Rc<FileType>>,
    by_mime_type: BTreeMap<String, Rc<FileType>>,
}

///
/// Assemble a single model by merging the sub-models represented by one or more files. The model
/// assembler uses an instance of [`FileTypeRegistry`](struct.FileTypeRegistry.html) to determine
/// the correct file mappings to load different representations. It may also scan an environment
/// variable for a set of paths to load models from as well as any specific files or directories
/// added via [`push`](struct.ModelAssembler.html#method.push) or
/// [`push_str`](struct.ModelAssembler.html#method.push_str).
///
#[derive(Clone, Debug)]
pub struct ModelAssembler {
    file_types: FileTypeRegistry,
    paths: HashSet<PathBuf>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Debug for FileType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for FileType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FileType{{{}{}}}",
            self.display_name,
            match &self.mime_type {
                None => String::new(),
                Some(mime_type) => format!(": {}", mime_type),
            }
        )
    }
}

impl FileType {
    ///
    /// Construct a new file type with the provided display name and reader function.
    ///
    pub fn new(name: &str, reader_fn: FileReader) -> Rc<Self> {
        Rc::new(Self {
            display_name: name.to_string(),
            mime_type: None,
            reader_fn,
        })
    }

    ///
    /// Construct a new file type with the provided display name, MIME type, and reader function.
    ///
    pub fn new_with_mime_type(name: &str, reader_fn: FileReader, mime_type: &str) -> Rc<Self> {
        Rc::new(Self {
            display_name: name.to_string(),
            mime_type: Some(mime_type.to_string()),
            reader_fn,
        })
    }

    ///
    /// Return this file type's display name.
    ///
    pub fn name(&self) -> &String {
        &self.display_name
    }

    ///
    /// Return this file type's MIME type, if present.
    ///
    pub fn mime_type(&self) -> &Option<String> {
        &self.mime_type
    }

    ///
    /// Return this file type's reader function.
    ///
    pub fn reader(&self) -> &FileReader {
        &self.reader_fn
    }
}

// ------------------------------------------------------------------------------------------------

impl Debug for FileTypeRegistry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.by_extension.keys()).finish()
    }
}

impl Default for FileTypeRegistry {
    fn default() -> Self {
        let mut new_self = Self::empty();

        new_self.register(
            FileType::new("JSON AST", |file| {
                let mut reader = json::JsonReader::default();
                reader.read(file)
            }),
            json::FILE_EXTENSION,
        );

        new_self.register(
            FileType::new("Smithy IDL", |file| {
                let mut reader = smithy::SmithyReader::default();
                reader.read(file)
            }),
            smithy::FILE_EXTENSION,
        );

        new_self
    }
}

impl FileTypeRegistry {
    ///
    /// Construct an empty registry.
    ///
    pub fn empty() -> Self {
        Self {
            by_extension: Default::default(),
            by_mime_type: Default::default(),
        }
    }

    ///
    /// Add a mapping from file extension to reader function.
    ///
    pub fn register(&mut self, file_type: Rc<FileType>, extension: &str) {
        let _ = self
            .by_extension
            .insert(extension.to_lowercase(), file_type.clone());
        if let Some(mime_type) = &file_type.mime_type {
            let _ = self
                .by_mime_type
                .insert(mime_type.to_lowercase(), file_type.clone());
        }
    }

    ///
    /// Add a mapping from file extension to reader function.
    ///
    pub fn register_all(&mut self, file_type: Rc<FileType>, extensions: &[&str]) {
        for extension in extensions {
            let _ = self
                .by_extension
                .insert(extension.to_lowercase(), file_type.clone());
        }
    }

    ///
    /// Returns `true` if there is a reader function for the provided extension, else `false`.
    ///
    pub fn contains(&self, extension: &str) -> bool {
        self.by_extension.contains_key(&extension.to_lowercase())
    }

    ///
    /// Returns the reader function for the provided extension, if present.
    ///
    pub fn get(&self, extension: &str) -> Option<&Rc<FileType>> {
        self.by_extension.get(&extension.to_lowercase())
    }

    ///
    /// Returns the reader function for the provided MIME type, if present.
    ///
    pub fn get_by_mime_type(&self, mime_type: &str) -> Option<&Rc<FileType>> {
        self.by_mime_type.get(&mime_type.to_lowercase())
    }

    ///
    /// Remove the mapping for the provided extension.
    ///
    pub fn remove(&mut self, extension: &str) -> Option<Rc<FileType>> {
        self.by_extension.remove(&extension.to_lowercase())
    }

    ///
    /// Return an iterator over the extensions currently supported.
    ///
    pub fn extensions(&self) -> impl Iterator<Item = &String> {
        self.by_extension.keys()
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for ModelAssembler {
    fn default() -> Self {
        Self::new(FileTypeRegistry::default(), Some(SearchPath::default()))
    }
}

impl TryFrom<ModelAssembler> for Model {
    type Error = Error;

    fn try_from(value: ModelAssembler) -> std::result::Result<Self, Self::Error> {
        let mut value = value;
        Model::try_from(&mut value)
    }
}

impl TryFrom<&mut ModelAssembler> for Model {
    type Error = Error;

    fn try_from(value: &mut ModelAssembler) -> std::result::Result<Self, Self::Error> {
        info!("Model::try_from::<ModelAssembler>(...)");
        if value.is_empty() {
            Ok(Model::default())
        } else {
            let models: std::result::Result<Vec<Model>, Self::Error> = value
                .expand_file_paths()
                .iter()
                .map(|file_name| value.read_model(&file_name))
                .collect();
            debug!(
                "Model::try_from::<ModelAssembler>(...): found models => {:#?}",
                &models
            );
            match models {
                Ok(mut models) => {
                    if models.is_empty() {
                        warn!(
                            "Model::try_from::<ModelAssembler>(...): No models found to assemble!"
                        );
                        Ok(Model::default())
                    } else {
                        let mut merged = models.remove(0);
                        for other in models {
                            merged.merge(other)?;
                        }
                        Ok(merged)
                    }
                }
                Err(err) => Err(err),
            }
        }
    }
}

impl ModelAssembler {
    ///
    /// Construct a new instance with a file type registry and search path. If the `search_path` is
    /// None then no environment variable is checked for paths.
    ///
    pub fn new(file_types: FileTypeRegistry, search_path: Option<SearchPath>) -> Self {
        info!("ModelAssembler::new()");
        let new_self = Self {
            file_types,
            paths: Default::default(),
        };
        match search_path {
            None => new_self,
            Some(search_path) => Self::include_search_path(new_self, search_path),
        }
    }

    ///
    /// Add a single file path to the assembler for later processing. The path may be a single file
    /// name or it may be a directory to scan.
    ///
    pub fn push(&mut self, path: &Path) -> &mut Self {
        info!("ModelAssembler::push({:?})", path);
        let _ = self.paths.insert(PathBuf::from(path));
        self
    }

    ///
    /// Add a single file path to the assembler for later processing. The string will be processed
    /// as a file path and handled in the same way as [`push`](method.push.html).
    ///
    pub fn push_str(&mut self, path: &str) -> &mut Self {
        info!("ModelAssembler::push_str({})", path);
        let _ = self.paths.insert(PathBuf::from(path));
        self
    }

    ///
    /// Returns `true` if there are no paths added to this assembler, else `false`.
    ///
    pub fn is_empty(&self) -> bool {
        self.paths.is_empty()
    }

    ///
    /// Returns the number of paths added to this assembler.
    ///
    pub fn len(&self) -> usize {
        self.paths.len()
    }

    ///
    /// Returns an iterator over all the paths added to this assembler.
    ///
    pub fn paths(&self) -> impl Iterator<Item = &Path> {
        self.paths.iter().map(|p| p.as_ref())
    }

    ///
    /// This processes all the paths added to the assembler and filters them for those with
    /// supported file extensions as well as finding files recursively in directory paths.
    ///
    pub fn expand_file_paths(&self) -> Vec<PathBuf> {
        info!("ModelAssembler::expand_file_paths()");
        let mut results = Vec::default();
        for path in &self.paths {
            self.expand_path(path, &mut results);
        }
        results
    }

    // --------------------------------------------------------------------------------------------

    fn include_search_path(self, search_path: SearchPath) -> Self {
        debug!("ModelAssembler::include_search_path",);
        let mut mut_self = self;
        mut_self.paths.extend(search_path.into_iter());
        mut_self
    }

    #[allow(clippy::ptr_arg)]
    fn expand_path(&self, path: &PathBuf, results: &mut Vec<PathBuf>) {
        info!("ModelAssembler::expand_path({:?})", path);
        if path.is_file() {
            if let Some(extension) = path.extension() {
                let extension = extension.to_string_lossy();
                if self.file_types.contains(extension.as_ref()) {
                    debug!("ModelAssembler::expand_path - adding file path {:?}", path);
                    let _ = results.push(path.clone());
                }
            }
        } else if path.is_dir() {
            debug!("ModelAssembler::expand_path - reading dir path {:?}", path);
            for entry in read_dir(path).unwrap() {
                let entry = entry.unwrap();
                self.expand_path(&entry.path(), results);
            }
        }
    }

    fn read_model(&self, path: &Path) -> Result<Model> {
        info!("ModelAssembler::read_model({:?})", path);
        if let Some(extension) = path.extension() {
            let extension = extension.to_string_lossy().to_lowercase();
            if let Some(file_type) = self.file_types.get(extension.as_ref()) {
                let mut file = File::open(path).unwrap();
                Ok(file_type.reader()(&mut file)?)
            } else {
                error!("ModelAssembler::read_model - not a known extension");
                Err(ErrorKind::InvalidRepresentation("unknown".to_string()).into())
            }
        } else {
            error!("ModelAssembler::read_model - has no extension");
            Err(ErrorKind::InvalidRepresentation("none".to_string()).into())
        }
    }
}
