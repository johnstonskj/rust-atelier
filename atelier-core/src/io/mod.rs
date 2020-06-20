/*!
Traits for reading and writing models in different formats.
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
/// Read a model from the string-like value `s` using the given `ModelReader`.
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
/// Write the `model` into a string `s` using the given `ModelWriter`.
///
pub fn write_model_to_string<'a>(w: &mut impl ModelWriter<'a>, model: &'a Model) -> Result<String> {
    use std::io::Cursor;
    let mut buffer = Cursor::new(Vec::new());
    w.write(&mut buffer, model)?;
    Ok(String::from_utf8(buffer.into_inner()).unwrap())
}
