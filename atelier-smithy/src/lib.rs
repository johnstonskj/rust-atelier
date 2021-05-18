/*!
* Provides the ability to read and write [Smithy](https://github.com/awslabs/smithy) models in their
* own native language representation.
*
* The crate provides two low-level parsers, `parse_model` and `parse_selector` that correspond to the
* core `Model` and `Selector` types. These are decouple to allow for the tool use cases where it is
* unnecessary or even undesirable to parse all selector expressions as well as those cases where
* selector expressions may be useful in non-model scenarios.
*
* This crate also provides implementations of both the core `ModelReader` and `ModelWriter` traits
* for Smithy IDL files.
*
* # Example - Model
*
* The following demonstrates the `SmithyReader` to parse a file into the in-memory model.
*
* ```rust
* use atelier_core::io::read_model_from_file;
* use atelier_smithy::SmithyReader;
* use std::path::PathBuf;
*
* let path = PathBuf::from("tests/good/waiters.smithy");
* let mut reader = SmithyReader::default();
* let result = read_model_from_file(&mut reader, path);
* assert!(result.is_ok());
* ```
*
* # Example - Selector
*
* The following example parses the simple selector expression `"*"`, denoting any shape, into it's
* in-memory model.
*
* ```rust
* use atelier_smithy::parse_selector;
*
* let result = parse_selector("*");
* assert!(result.is_ok());
* ```
*
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

#[macro_use]
extern crate log;

// ------------------------------------------------------------------------------------------------
// Public Values
// ------------------------------------------------------------------------------------------------

///
/// The file extension used by Smithy IDL files.
///
pub const FILE_EXTENSION: &str = "smithy";

///
/// The name to report in errors in this representation.
///
pub const REPRESENTATION_NAME: &str = "Smithy IDL";

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
pub mod reader;
pub use reader::SmithyReader;

#[doc(hidden)]
pub mod writer;
pub use writer::SmithyWriter;

#[doc(hidden)]
pub mod parser;
pub use parser::{parse_model, parse_selector};

mod syntax;
