/*!
Provides an [`rdf_to_model`](fn.rdf_to_model.html) function and [`RdfReader`](struct.RdfReader.html)
type that will read from an RDF source and construct a model.

# Example - rdf_to_model

Currently unimplemented.

# Example - RdfReader

Currently unimplemented.

*/

use atelier_core::error::Result as ModelResult;
use atelier_core::io::ModelReader;
use atelier_core::model::Model;
use rdftk_core::Graph;
use rdftk_iri::IRIRef;
use std::io::Read;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Simple implementation of the `ModelReader` trait that reads the RDF representation of a model.
///
#[derive(Debug)]
pub struct RdfReader {}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Convert an RDF graph into a Smithy semantic model.
///
pub fn rdf_to_model(_rdf_graph: &impl Graph, _model_iri: Option<IRIRef>) -> ModelResult<Model> {
    unimplemented!()
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl ModelReader for RdfReader {
    fn read(&mut self, _r: &mut impl Read) -> atelier_core::error::Result<Model> {
        unimplemented!()
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
