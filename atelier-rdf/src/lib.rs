/*!
This crate provides a mapping to allow for the mapping of Smithy semantic models to and from
the W3C's [Resource Description Framework (RDF)](https://www.w3.org/RDF/). This allows for tools
to integrate Smithy models into other knowledge frameworks and to enrich the model with additional
facts from other tools. It also allows for inferencing over Smithy models using ontology languages
such as the W3C [Web Ontology Language (OWL)](https://www.w3.org/OWL/).

For a detailed specification for the RDF mapping, see the
[Atelier Book](https://rust-atelier.dev/reference/rdf.html).

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
// Public Values
// ------------------------------------------------------------------------------------------------

///
/// The name to report in errors in this representation.
///
pub const REPRESENTATION_NAME: &str = "RDF";

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod urn;

#[doc(hidden)]
pub mod reader;

pub mod writer;

pub mod vocabulary;
