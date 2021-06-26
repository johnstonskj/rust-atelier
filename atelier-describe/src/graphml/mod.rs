/*!
This module provides a writer for GraphML visualization of a model.
*/

///
/// The extension to use when reading from, or writing to, files of this type.
///
pub const FILE_EXTENSION: &str = "graphml";
///
/// The name to report in errors in this representation.
///
pub const REPRESENTATION_NAME: &str = "GraphML";

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod writer;
