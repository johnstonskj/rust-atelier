/*!
One-line description.

More detailed description, with

# Example

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
extern crate log;

#[macro_use]
extern crate paste;

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
#[macro_use]
mod macros;

pub mod evaluate;

pub mod projection;
pub use projection::Projection;
