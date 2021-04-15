/*!
This crate provides a mapping to allow for the mapping of Smithy semantic models to and from
the W3C's [Resource Description Framework (RDF)](https://www.w3.org/RDF/). This allows for tools
to integrate Smithy models into other knowledge frameworks and to enrich the model with additional
facts from other tools. It also allows for inferencing over Smithy models using ontology languages
such as the W3C [Web Ontology Language (OWL)](https://www.w3.org/OWL/).

This *model-to-model* mapping is performed by the `model_to_rdf` and `rdf_to_model` functions; for
the specifics of this mapping, see the module [`model`](model/index.html).

# Example - Mapping

The following simply constructs an RDF Graph from a provided model.

```rust
use atelier_core::model::Model;
use atelier_rdf::model::model_to_rdf;
# use atelier_core::Version;
# fn make_model() -> Model { Model::new(Version::default()) }

let model = make_model();
let rdf_graph = model_to_rdf(&model, None).unwrap();
```

# Example - Writer

This example writes the provided model in RDF's [Turtle](https://www.w3.org/TR/turtle/)
serialization representation.

```rust
use atelier_core::model::Model;
use atelier_core::io::ModelWriter;
use atelier_rdf::writer::RdfWriter;
use std::io::stdout;
# use atelier_core::Version;
# fn make_model() -> Model { Model::new(Version::default()) }

let model = make_model();
let mut writer = RdfWriter::default();
writer.write(&mut stdout(), &model).unwrap();
```
*/

#![warn(
    // ---------- Stylistic
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    // ---------- Public
    missing_debug_implementations,
    missing_docs,
    unreachable_pub,
    // ---------- Unsafe
    unsafe_code,
    // ---------- Unused
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
)]

#[macro_use]
extern crate lazy_static;

#[allow(unused_imports)]
#[macro_use]
extern crate paste;

#[macro_use]
extern crate rdftk_names;

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod urn;

pub mod model;

#[doc(hidden)]
pub mod reader;

pub mod writer;

pub mod vocabulary;
