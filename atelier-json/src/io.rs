/*!
One-line description.

More detailed description, with

# Example

*/

use atelier_core::error::{Result, ResultExt};
use atelier_core::io::ModelWriter;
use atelier_core::model::Model;
use serde_json::{to_writer_pretty, Map, Value};
use std::io::Write;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub struct JsonWriter;

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl<'a> Default for JsonWriter {
    fn default() -> Self {
        Self {}
    }
}

impl<'a> ModelWriter<'a> for JsonWriter {
    fn write(&mut self, w: &mut impl Write, model: &'a Model) -> Result<()> {
        let mut top: Map<String, Value> = Default::default();

        top.insert(
            "version".to_string(),
            Value::String(match model.version() {
                None => atelier_core::Version::default().to_string(),
                Some(v) => v.to_string(),
            }),
        );

        top.insert("shapes".to_string(), self.shapes(model));

        to_writer_pretty(w, &Value::Object(top)).chain_err(|| "Serialization error")
    }
}

impl<'a> JsonWriter {
    fn shapes(&self, _model: &'a Model) -> Value {
        #[allow(unused_mut)]
        let mut shape_map: Map<String, Value> = Default::default();
        Value::Object(shape_map)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
