/*!
This module provides a writer for Markdown documentation for a model.
*/

///
/// The file extension used by Markdown files.
///
pub const FILE_EXTENSION: &str = "md";

///
/// The name to report in errors in this representation.
///
pub const REPRESENTATION_NAME: &str = "Structured Documentation";

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod writer;
