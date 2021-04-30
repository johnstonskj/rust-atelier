/*!
This crate provides a number of transformations and writers for documentation forms of a model.
As these are purely documentation, or visualizations, of a model there is no meaningful read
mapping for these formats.

Currently supported:

* mod [document](document/index.html) - creates Markdown textual documentation.
* mod [graphml](graphml/index.html) - creates a GraphML visualization.
* mod [plant_uml](plant_uml/index.html) - creates a UML visualization.

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

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod document;

pub mod graphml;

pub mod plant_uml;
