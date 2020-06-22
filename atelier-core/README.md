# Atelier: crate atelier_core

This crate provides a Rust native core model for the AWS [Smithy](https://github.com/awslabs/smithy) Interface 
Definition Language.

[![crates.io](https://img.shields.io/crates/v/atelier_core.svg)](https://crates.io/crates/atelier_core)
[![docs.rs](https://docs.rs/atelier_core/badge.svg)](https://docs.rs/atelier_core)

This crate is the foundation for the Atelier set of crates, and provides the following components:

1. The model structures themselves that represents a Smithy model, this is the in-memory representation shared by all 
   Atelier crates and tools.
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

## Changes

**Version 0.1.1** (_in progress_)

TBD

**Version 0.1.0**

* First release.
* Initial types for manipulation of Smithy Models, _not_ including selector expressions.
* Initial builder types for fluent construction of models.
* Able to construct the example weather service using the builder API.

## TODO

1. Complete the prelude model.
1. Complete the selector expression types
1. Complete the builder types
1. Validation!
2. Complete macro-ize APIs for less code
3. More documentation
4. More tests
