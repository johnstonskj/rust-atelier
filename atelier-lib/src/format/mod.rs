/*!
Child modules that implement `ModelReader` and `ModelWriter` for specific representations.
*/

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[cfg(feature = "describe")]
pub use atelier_describe::document;

#[cfg(feature = "describe")]
pub use atelier_describe::graphml;

#[cfg(feature = "describe")]
pub use atelier_describe::plant_uml;

#[cfg(feature = "json")]
pub use atelier_json as json;

#[cfg(feature = "openapi")]
pub use atelier_openapi as openapi;

#[cfg(feature = "rdf")]
pub use atelier_rdf as rdf;

#[cfg(feature = "smithy")]
pub use atelier_smithy as smithy;
