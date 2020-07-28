/*!
Provides the ability to read and write [Smithy](https://github.com/awslabs/smithy) models in their
own native language representation.

# Example

TBD
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
extern crate pest_derive;

// ------------------------------------------------------------------------------------------------
// Public Values
// ------------------------------------------------------------------------------------------------

///
/// The file extension used by Smithy IDL files.
///
pub const FILE_EXTENSION: &str = "smithy";

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
pub mod reader;
pub use reader::SmithyReader;

#[doc(hidden)]
pub mod writer;
pub use writer::SmithyWriter;

mod parser;
