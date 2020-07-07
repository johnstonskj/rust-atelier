/*!
Child modules that implement `ModelReader` and `ModelWriter` for specific representations.
*/

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[cfg(feature = "json")]
pub use atelier_json as json;

#[cfg(feature = "openapi")]
pub use atelier_openapi as openapi;

#[cfg(feature = "smithy")]
pub use atelier_smithy as smithy;
