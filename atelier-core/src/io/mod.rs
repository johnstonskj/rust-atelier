/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::Result;
use crate::model::Model;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait ModelWriter: Default {
    fn write(&mut self, w: &mut impl std::io::Write, model: &Model) -> Result<()>;
}

pub trait ModelReader: Default {
    fn read(&mut self, r: &mut impl std::io::Read) -> Result<Model>;
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn read_model_from_string<S>(r: &mut impl ModelReader, s: S) -> Result<Model>
where
    S: AsRef<[u8]>,
{
    use std::io::Cursor;
    let mut buffer = Cursor::new(s);
    r.read(&mut buffer)
}

pub fn write_model_to_string(w: &mut impl ModelWriter, model: &Model) -> Result<String> {
    use std::io::Cursor;
    let mut buffer = Cursor::new(Vec::new());
    w.write(&mut buffer, model)?;
    Ok(String::from_utf8(buffer.into_inner()).unwrap())
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
