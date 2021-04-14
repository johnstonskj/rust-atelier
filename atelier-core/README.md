# Atelier: crate atelier_core

This crate provides a Rust native core model for the AWS [Smithy](https://github.com/awslabs/smithy) Interface 
Definition Language.

[![crates.io](https://img.shields.io/crates/v/atelier_core.svg)](https://crates.io/crates/atelier_core)
[![docs.rs](https://docs.rs/atelier_core/badge.svg)](https://docs.rs/atelier_core)

This crate is the foundation for the Atelier set of crates, and provides the following components:

1. The model structures themselves that represents a Smithy model This is the in-memory representation shared by all 
   Atelier crates and tools.
1. The model builder structures that allow for a fluent and easy construction of a core model.
1. The prelude model containing the set of shapes defined in the Smithy specification.
1. Traits for reading/writing models in different representations.
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
    ServiceBuilder, ShapeTraits, SimpleShapeBuilder, StructureBuilder, TraitBuilder,
};
use atelier_core::model::{Identifier, Model, ShapeID};

fn make_model() -> Model {
    ModelBuilder::new(Version::V10, "example.motd")
        .service(
            ServiceBuilder::new("MessageOfTheDay", "2020-06-21")
                .documentation("Provides a Message of the day.")
                .resource("Message")
                .into(),
        )
        .resource(
            ResourceBuilder::new("Message")
                .identifier("date", "Date")
                .read("GetMessage")
                .into(),
        )
        .simple_shape(
            SimpleShapeBuilder::string("Date")
                .apply_trait(traits::pattern(r"^\d\d\d\d\-\d\d-\d\d$"))
                .into(),
        )
        .operation(
            OperationBuilder::new("GetMessage")
                .readonly()
                .input("GetMessageInput")
                .output("GetMessageOutput")
                .error("BadDateValue")
                .into(),
        )
        .structure(
            StructureBuilder::new("GetMessageInput")
                .member("date", "Date")
                .into(),
        )
        .structure(
            StructureBuilder::new("GetMessageOutput")
                .add_member(MemberBuilder::string("message").required().into())
                .into(),
        )
        .structure(
            StructureBuilder::new("BadDateValue")
                .error_source(ErrorSource::Client)
                .add_member(MemberBuilder::string("errorMessage").required().into())
                .into(),
        )
        .into()
}
```
*/

## Runnable Examples

The example `weather_builder.rs` in the `examples` directory uses the complete example from the Smithy quick start guide. The
`examples` directory also includes a pair of stand-alone examples, using the semantic model and builder APIs, for the
_message of the day_ service shown in the example above.

As usual these can be executed via cargo in the following manner. 

```bash
$ cargo run --example weather_builder
$ cargo run --example motd_core
$ cargo run --example motd_builder
```

## Changes

**Version 0.2.5**

* Added public `shape_selector!` macro.
* Added `PartialEq` to all model types to enable more testing.  
* Tidied up builder interfaces and added some more trait functions.
* Renamed `ExpressionListBuilder` to `SelectorBuilder`.

**Version 0.2.3**

* Added initial builder types for constructing `Selector` models.
* Renamed `ShapeType::All` to `ShapeType::Any`.

**Version 0.2.3**

* Added support for constructing `Selector` models.

**Version 0.2.2**

* Refactor: added `HasIdentity` and `HasTraits` traits to allow doc writer to be polymorphic.
* Added: used prelude constants in trait builder.

**Version 0.2.1**

* Added `MutableModelVisitor` trait.

**Version 0.2.0**

* Major refactor after agreement on the separation of semantic model with Smithy team.
  * Only included the semantic model elements
  * Reworked trait module
  * Made all shape IDs absolute
* Made Builder API create and validate shape IDs.
* Moved UML writer to lib.

**Version 0.1.5**

* Added `UnwelcomeTerms` linter.
* API changes for `ModelReader` and `ModelWriter`;
  * removed `representation` method
* Made `ActionIssue` an `std::error::Error`.
* Added `ModelVisitor` to `action` module.
* Added `Model::merge` method.

**Version 0.1.4**

* Completed the `NoOrphanedReferences` validator.
* Added `CorrectTypeReferences` validator.
* Added `NamingConventions` linter.
* Added functions `run_linter_actions` and `run_validation_actions`, and removed `ValidateAll` type.
* Using Regex for identity parsing.
* Moved `REPRESENTATION` to `representation()` on `ModelReader` and `ModelWriter` traits.

**Version 0.1.3**

* Made debug and uml `Writer` implementations features.
* Added module `syntax` for all string constants.
* Added all defined `ShapeID`s to the module `prelude`.
* Implemented the foundation for actions, also started on validation.
* Implemented `Model::resolve_id` according to the Smithy spec.

**Version 0.1.2**

* Updated the model and builder APIs to support JSON and Smithy readers:
  * added the `HasMembers` trait for a more un-typed API,
* Finished the API documentation.


**Version 0.1.1**

* Updated the model and builder APIs to be more consistent:
  * documented method patterns, and ensured they were applied,
  * moved from per-type `build` methods to use `Into<T>`,
* Added the majority of API documentation.

**Version 0.1.0**

* First release.
* Initial types for manipulation of Smithy Models, _not_ including selector expressions.
* Initial builder types for fluent construction of models.
* Able to construct the example weather service using the builder API.

## TODO

1. Complete the prelude model.
1. More documentation
1. More tests
