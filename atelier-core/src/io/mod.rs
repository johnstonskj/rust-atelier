/*!
Traits for reading and writing models in different formats. Separate crates implement the ability
to handle different representations, such as the original Smithy, JSON AST, and OpenAPI.

This module also provides some useful `Writer` implementations, all are features, all are included
by default.

* **debug**; uses the `Debug` implementation of Model to write out the internal structure.
* **uml**; uses [PlantUML](https://plantuml.com/) to generate diagrams of a model structure.

# Example Model Writer

The example below is pretty much the implementation of the `debug` module, it writes the model
using the `Debug` implementation associated with those objects.

```rust
# use atelier_core::io::ModelWriter;
# use atelier_core::model::Model;
# use atelier_core::error::Result as ModelResult;
# use std::io::Write;
#[derive(Debug)]
pub struct Debugger {}

impl Default for Debugger {
    fn default() -> Self {
        Self {}
    }
}

impl ModelWriter for Debugger {
    fn write(&mut self, w: &mut impl Write, model: &Model) -> ModelResult<()> {
        write!(w, "{:#?}", model)?;
        Ok(())
    }
}
```

*/

use crate::error::Result as ModelResult;
use crate::model::Model;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Trait implemented to write a model in a specific representation. It is expected that
/// implementations of this trait would ensure that the model is complete unless they can
/// specifically serialize an incomplete model (the Smithy IDL can).
///
pub trait ModelWriter {
    ///
    /// Write the `model` to given the implementation of `Write`.
    ///
    fn write(&mut self, w: &mut impl std::io::Write, model: &Model) -> ModelResult<()>;
}

///
/// Trait implemented to read a model from a specific representation.
///
pub trait ModelReader {
    ///
    ///  Read a model from the given implementation of `Read`.
    ///
    fn read(&mut self, r: &mut impl std::io::Read) -> ModelResult<Model>;
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Read a model from the string-like value `s` using the given `ModelReader`. This is simply a
/// short-cut that saves some repetitive boiler-plate.
///
pub fn read_model_from_string<S>(r: &mut impl ModelReader, s: S) -> ModelResult<Model>
where
    S: AsRef<[u8]>,
{
    use std::io::Cursor;
    let mut buffer = Cursor::new(s);
    r.read(&mut buffer)
}

///
/// Write the `model` into a string `s` using the given `ModelWriter`. This is simply a
/// short-cut that saves some repetitive boiler-plate.
///
pub fn write_model_to_string<'a>(
    w: &mut impl ModelWriter,
    model: &'a Model,
) -> ModelResult<String> {
    use std::io::Cursor;
    let mut buffer = Cursor::new(Vec::new());
    w.write(&mut Box::new(&mut buffer), model)?;
    Ok(String::from_utf8(buffer.into_inner()).unwrap())
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod debug;
