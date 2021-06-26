/*!
This module provides a writer for a UML visualization of a model.
*/

///
/// The extension to use when reading from, or writing to, files of this type.
///
pub const FILE_EXTENSION: &str = "uml";
///
/// The name to report in errors in this representation.
///
pub const REPRESENTATION_NAME: &str = "PlantUML";

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod writer;
