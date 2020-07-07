/*!
`ModelReader` and `ModelWriter` implementations for the Smithy native IDL format.
*/

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The file extension used by Smithy IDL files.
///
pub const FILE_EXTENSION: &str = "smithy";

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
pub mod read;
pub use read::SmithyReader;

#[doc(hidden)]
pub mod write;
pub use write::SmithyWriter;

mod parser;
