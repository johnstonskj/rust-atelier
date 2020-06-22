/*!
This crate provides a Rust native core model for the AWS [Smithy](https://github.com/awslabs/smithy) Interface Definition Language.

This crate is the foundation for the Atelier set of crates, and provides the following components:

1. The model structures themselves that represents a Smithy model, this is intended to be the
   in-memory representation shared by all Atelier crates and tools.
1. The model builder structures that allow for a fluent and easy construction of a core model.
1. The prelude model containing the set of shapes defined in the Smithy specification.
1. Traits for reading/writing models in different representations.
1. Trait and simple implementation for a model registry.
1. A common `error` module to be used by all Atelier crates.

# Example

The following example demonstrates the builder interface to create a model for a simple service. The
service, `MessageOfTheDay` has a single resource `Message`. The resource has an identifier for the
date, but the `read` operation does not make the date member required and so will return the message
for the current date.

```rust
use atelier_core::error::ErrorSource;
use atelier_core::model::builder::values::{ArrayBuilder, ObjectBuilder};
use atelier_core::model::builder::{
    Builder, ListBuilder, MemberBuilder, ModelBuilder, OperationBuilder, ResourceBuilder,
    ServiceBuilder, SimpleShapeBuilder, StructureBuilder, TraitBuilder,
};
use atelier_core::model::{Identifier, Model, ShapeID};

let model = ModelBuilder::new("example.motd")
    .shape(
        ServiceBuilder::new("MessageOfTheDay")
            .documentation("Provides a Message of the day.")
             .version("2020-06-21")
            .resource("Message")
            .build(),
    )
    .shape(
        ResourceBuilder::new("Message")
            .identifier("date", "Date")
            .read("GetMessage")
            .build(),
    )
    .shape(
        SimpleShapeBuilder::string("Date")
            .add_trait(TraitBuilder::pattern(r"^\d\d\d\d\-\d\d-\d\d$").build())
            .build(),
    )
    .shape(
        OperationBuilder::new("GetMessage")
            .readonly()
            .input("GetMessageInput")
            .output("GetMessageOutput")
            .error("BadDateValue")
            .build(),
    )
    .shape(
        StructureBuilder::new("GetMessageInput")
            .add_member(
                MemberBuilder::new("date")
                    .refers_to("Date")
                    .build(),
            )
            .build(),
    )
    .shape(
        StructureBuilder::new("GetMessageOutput")
            .add_member(MemberBuilder::string("message").required().build())
            .build(),
    )
    .shape(
        StructureBuilder::new("BadDateValue")
            .error(ErrorSource::Client)
            .add_member(MemberBuilder::string("errorMessage").required().build())
            .build(),
    )
    .build();
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
    // missing_docs,
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
extern crate error_chain;

use std::fmt::{Display, Formatter};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Versions of the Smithy specification.
///
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub enum Version {
    /// Version 1.0 (initial, and current)
    V10,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for Version {
    fn default() -> Self {
        Self::current()
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "1.0")
    }
}

impl FromStr for Version {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "1.0" {
            Ok(Self::V10)
        } else {
            Err(error::ErrorKind::InvalidVersionNumber(s.to_string()).into())
        }
    }
}

impl Version {
    ///
    /// Returns the most current version of the Smithy specification.
    ///
    pub fn current() -> Self {
        Self::V10
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod error;

pub mod io;

pub mod model;

pub mod prelude;

pub mod registry;
