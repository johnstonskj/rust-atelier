# Using the Builder API


The following example demonstrates the builder interface to create the same service as the example
above. Hopefully this is more readable as it tends to be less repetative, uses  `&str` for
identifiers, and includes helper functions for common traits for example. It provides this better
_construction experience_ (there are no read methods on builder objects) by compromising two aspects:

1. The API itself is very repetative; this means the same method may be on multiple objects, but
   makes it easier to use. For example, you want to add the documentation trait to a shape, so you can:
    1. construct a `Trait` entity using the core model and the `Builder::add_trait` method,
    1. use the `TraitBuilder::documentation` method which also takes the string to use as the trait
       value and returns a new `TraitBuilder`, or
    1. use the `Builder::documentation` method that hides all the details of a trait and just takes
       a string.
1. It hides a lot of the `Identifier` and `ShapeID` construction and so any of those calls to
   `from_str` may fail when the code unwraps the result. This means the builder can panic in ways
   the core model does not.
   
```rust
use atelier_core::builder::traits::ErrorSource;
use atelier_core::builder::values::{ArrayBuilder, ObjectBuilder};
use atelier_core::builder::{
    traits, ListBuilder, MemberBuilder, ModelBuilder, OperationBuilder, ResourceBuilder,
    ServiceBuilder, ShapeTraits, SimpleShapeBuilder, StructureBuilder, TraitBuilder,
};
use atelier_core::model::{Identifier, Model, ShapeID};
use atelier_core::Version;
use std::convert::TryInto;

let model: Model = ModelBuilder::new(Version::V10, "example.motd")
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
    .try_into().unwrap();
```
