/*!
A simple implementation of the `ModelWriter` trait that simply uses the Debug trait. This allows
the easy swapping in of a sanity check as different reader/writer implementations are used.
*/

use crate::io::ModelWriter;
use crate::model::Model;
use std::io::Write;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Simple implementation of the `ModelWriter` trait that uses the fact that all the core model
/// structures and enumerations implement `Debug`.
///
#[derive(Debug)]
pub struct DebugWriter {}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for DebugWriter {
    fn default() -> Self {
        Self {}
    }
}

impl ModelWriter for DebugWriter {
    fn write(&mut self, w: &mut impl Write, model: &Model) -> crate::error::Result<()> {
        write!(w, "{:#?}", model)?;
        Ok(())
    }
}
