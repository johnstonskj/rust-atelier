/*!
Provides both `JsonReader` and `JsonWriter` implementations.
*/

///
/// The extension to use when reading from, or writing to, files of this type.
///
pub const FILE_EXTENSION: &str = "json";

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
pub mod reader;
pub use reader::JsonReader;

#[doc(hidden)]
pub mod writer;
pub use writer::JsonWriter;

mod syntax;
