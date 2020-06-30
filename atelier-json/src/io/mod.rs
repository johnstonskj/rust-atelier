/*!
Provides both `JsonReader` and `JsonWriter` implementations.
*/

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
