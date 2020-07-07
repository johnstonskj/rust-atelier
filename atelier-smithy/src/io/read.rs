use crate::io::parser;
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
        parser::parse(&content)
    }
}
