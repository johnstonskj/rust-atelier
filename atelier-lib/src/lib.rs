/*!
Combined crate for all Atelier sub-crates incorporated as features. Atelier is a Rust native
library, and tools, for the AWS [Smithy](https://github.com/awslabs/smithy) Interface Definition
Language.

# Features

The aim of this crate is to provide a single client interface over a set of crates that provide
different Atelier capabilities. The following table shows the mapping from individual crate to the
combined module path in this library. The column _Default_ indicates those that are included in the
default feature, although the core will be included regardless of any feature selection.

| Feature name | Default | Individual crate                                     | Target module path     | Purpose                                               |
|--------------|---------|------------------------------------------------------|------------------------|-------------------------------------------------------|
| N/A          | **Yes** | [atelier_core](https://docs.rs/atelier_core)         | `::core`               | Semantic model, builders, and API traits.             |
| "assembler"  | **Yes** | [atelier_assembler](https://docs.rs/atelier_assembler) | `::assembler`        | Model assembly from multiple files.                   |
| "describe"   | **Yes** | [atelier_describe](https://docs.rs/atelier_describe) | `::format::document`   | Writing markdown documentation.                       |
|              |         |                                                      | `::format::graphml`    | Writing GraphML visualizations.                       |
|              |         |                                                      | `::format::plant_uml`  | Writing UML visualizations.                           |
| "json"       | **Yes** | [atelier_json](https://docs.rs/atelier_json)         | `::format::json`       | Reading and Writing JSON AST representation.          |
| "openapi"    | No      | [atelier_openapi](https://docs.rs/atelier_openapi)   | `::format::openapi`    | Reading and Writing OpenAPI representations.          |
| "rdf"        | No      | [atelier_rdf](https://docs.rs/atelier_rdf)           | `::format::rdf`        | Reading and Writing RDF representations.              |
| "smithy"     | **Yes** | [atelier_smithy](https://docs.rs/atelier_smithy)     | `::format::smithy`     | Reading and Writing the Smithy native representation. |

This crate also provides some pre-defined [action](actions/index.html) functions for linting and
validating models.

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

pub use atelier_core as core;

pub mod actions;

#[cfg(feature = "assembler")]
pub use atelier_assembler as assembler;

#[cfg(any(
    feature = "describe",
    feature = "json",
    feature = "openapi",
    feature = "rdf",
    feature = "smithy",
))]
pub mod format;
