use crate::parser;
use atelier_core::error::Result;
use atelier_core::io::ModelReader;
use atelier_core::model::Model;
use std::io::Read;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Read a [Model](../atelier_core/model/struct.Model.html) from the Smithy native representation.
///
#[derive(Debug)]
pub struct SmithyReader;

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for SmithyReader {
    fn default() -> Self {
        Self {}
    }
}

impl ModelReader for SmithyReader {
    fn read(&mut self, r: &mut impl Read) -> Result<Model> {
        let mut content: String = String::new();
        let _ = r.read_to_string(&mut content)?;
        parser::parse_model(&content)
    }
}

impl SmithyReader {
    /// Merge multiple model files to create a single model
    pub fn merge<S: AsRef<str>>(&mut self, model_files: Vec<S>) -> Result<Model> {
        let mut model = Model::default();
        for partial in model_files.iter() {
            let m = parser::parse_model(partial.as_ref())?;
            model.merge(m)?;
        }
        Ok(model)
    }
}
