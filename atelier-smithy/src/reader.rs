use crate::parser;
use atelier_core::error::Result;
use atelier_core::io::ModelReader;
use atelier_core::model::Model;
use std::io::Read;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This struct implements the `ModelReader` trait to read a [Model](../atelier_core/model/struct.Model.html)
/// from the [Smithy IDL](https://awslabs.github.io/smithy/1.0/spec/core/idl.html) representation.
///
/// Currently the Smithy reader takes no parameters and so is constructed simply using `Default`:
///
/// ```rust
/// use atelier_smithy::SmithyReader;
///
/// let reader = SmithyReader::default();
/// ```
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
