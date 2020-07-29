/*!
Future home of the RDF reader and writer for Smithy.

For the mapping between RDF and Smithy, see the module [`model`](model/index.html).

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
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
pub mod urn;
pub use urn::SmithyUrn;

pub mod model;

pub mod reader;

pub mod writer;

pub mod vocabulary;
