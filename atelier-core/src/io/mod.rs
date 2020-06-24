/*!
Traits for reading and writing models in different formats. Separate crates implement the ability
to handle different representations, such as the original Smithy, JSON AST, and OpenAPI.

# Example Model Writer

The example below is pretty much the implementation of the `debug` module, it writes the model
using the `Debug` implementation associated with those objects.

```rust
# use atelier_core::io::ModelWriter;
# use atelier_core::model::Model;
# use atelier_core::error::Result;
# use std::io::Write;
#[derive(Debug)]
pub struct Debugger {}

impl Default for Debugger {
    fn default() -> Self {
        Self {}
    }
}

impl<'a> ModelWriter<'a> for Debugger {
    fn write(&mut self, w: &mut impl Write, model: &'a Model) -> Result<()> {
        write!(w, "{:#?}", model)?;
        Ok(())
    }
}
```

*/

use crate::error::Result;
use crate::model::Model;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Trait implemented to write a model in a specific representation.
///
pub trait ModelWriter<'a>: Default {
    ///
    /// Write the `model` to given the implementation of `Write`.
    ///
    fn write(&mut self, w: &mut impl std::io::Write, model: &'a Model) -> Result<()>;
}

///
/// Trait implemented to read a model from a specific representation.
///
pub trait ModelReader: Default {
    ///
    ///  Read a model from the given implementation of `Read`.
    ///
    fn read(&mut self, r: &mut impl std::io::Read) -> Result<Model>;
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Read a model from the string-like value `s` using the given `ModelReader`. This is simply a
/// short-cut that saves some repetitive boiler-plate.
///
pub fn read_model_from_string<S>(r: &mut impl ModelReader, s: S) -> Result<Model>
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
pub fn write_model_to_string<'a>(w: &mut impl ModelWriter<'a>, model: &'a Model) -> Result<String> {
    use std::io::Cursor;
    let mut buffer = Cursor::new(Vec::new());
    w.write(&mut buffer, model)?;
    Ok(String::from_utf8(buffer.into_inner()).unwrap())
}

///
/// Simple debug tools, including an implementation of the `ModelWriter` trait. This allows the easy
/// swapping in ofa sanity check as different reader/writer implementations are used.
///
pub mod debug {
    use crate::io::ModelWriter;
    use crate::model::Model;
    use std::io::Write;

    ///
    /// Simple implementation of the `ModelWriter` trait that uses the fact that all the core model
    /// structures and enumerations implement `Debug`.
    ///
    #[derive(Debug)]
    pub struct Debugger {}

    impl Default for Debugger {
        fn default() -> Self {
            Self {}
        }
    }

    impl<'a> ModelWriter<'a> for Debugger {
        fn write(&mut self, w: &mut impl Write, model: &'a Model) -> crate::error::Result<()> {
            write!(w, "{:#?}", model)?;
            Ok(())
        }
    }
}
