/*!
This implementation of ModelWriter will output the provided model in it's RDF form, using the
Turtle serialization format. If you wish to use other serialization formats it is best to call
model_to_rdf and use one of the graph writer implementation in the
[rdfktk_io](https://github.com/johnstonskj/rust-rdftk/tree/master/rdftk_io) crate.

# Example

TBD

*/

use crate::model::model_to_rdf;
use atelier_core::io::ModelWriter;
use atelier_core::model::Model;
use rdftk_io::turtle::TurtleWriter;
use rdftk_io::GraphWriter;
use std::io::Write;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Simple implementation of the `ModelWriter` trait that writes the RDF representation of a model.
///
#[derive(Debug)]
pub struct RdfWriter {}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for RdfWriter {
    fn default() -> Self {
        Self {}
    }
}

impl ModelWriter for RdfWriter {
    fn write(&mut self, w: &mut impl Write, model: &Model) -> atelier_core::error::Result<()> {
        let rdf_graph = model_to_rdf(model, None)?;

        let writer = TurtleWriter::default();
        writer.write(w, &rdf_graph)?;

        Ok(())
    }
}
