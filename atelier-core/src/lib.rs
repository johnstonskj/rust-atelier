/*!
This crate provides a Rust native implementaton of the the AWS [Smithy](https://github.com/awslabs/smithy)
semantic model and foundational capabilities..

The semantic model (a component of the [Smithy framework](https://awslabs.github.io/smithy/1.0/spec/core/model.html#smithy-framework))
is the core representation used by tools in the Smithy build process.
This crate provides an implementation of the semantic model the for the Atelier set of crates, and
core traits for other crates. Specifically it provides:

1. The semantic [model](model/index.html) itself  that represents a Smithy model. This API is the
   in-memory representation shared by all Atelier crates and tools.
1. A set of model [builder](builder/index.html)s that allow for a more _fluent_ and less repetative
   construction of a core model.
1. A pair of traits for model [io](io/index.html) and helper functions for reading/writing models.
1. The [prelude](prelude/index.html) module contains the set of shapes defined in the Smithy specification.
1. Traits for model [actions](action/index.html) used to implement linters, validators, and transformations.
1. Traits for [reading/writing](io/index.html) model files in different representations.
1. A common [error](error/index.html) module to be used by all Atelier crates.


# The Semantic Model API Example

The following example demonstrates the core model API to create a model for a simple service. The
service, `MessageOfTheDay` has a single resource `Message`. The resource has an identifier for the
date, but the `read` operation does not make the date member required and so will return the message
for the current date.

This API acts as a set of generic data objects and as such has a tendency to be verbose in the
construction of models. The need to create a lot of `Identifier` and `ShapeID` instances, for example,
does impact the readability. It is important to note, that while there is a discussion in the Smithy
[specification](https://awslabs.github.io/smithy/1.0/spec/core/model.html#shape-id) contains the
notion of both _absolute_ and _relative_ shape identifiers it is important to note that relative
identifiers **are not** supported in the semantic model. All names in the semantic model **must**
be resolved to an absolute name.

For more information, see [the Rust Atelier book](https://rust-atelier.dev/using/model_api.html).

```rust
use atelier_core::model::identity::{HasIdentity, Identifier};
use atelier_core::model::shapes::{
    HasTraits, MemberShape, Operation, Resource, Service, Shape,
    ShapeKind, Simple, StructureOrUnion, TopLevelShape,
};
use atelier_core::model::values::Value;
use atelier_core::model::{Model, NamespaceID};
use atelier_core::prelude::PRELUDE_NAMESPACE;
use atelier_core::Version;

let prelude: NamespaceID = PRELUDE_NAMESPACE.parse().unwrap();
let namespace: NamespaceID = "example.motd".parse().unwrap();

// ----------------------------------------------------------------------------------------
let mut date = TopLevelShape::new(
    namespace.make_shape("Date".parse().unwrap()),
    ShapeKind::Simple(Simple::String),
);
date
    .apply_with_value(
        prelude.make_shape("pattern".parse().unwrap()),
        Value::String(r"^\d\d\d\d\-\d\d-\d\d$".to_string()).into()
    )
    .unwrap();

// ----------------------------------------------------------------------------------------
let shape_name = namespace.make_shape("BadDateValue".parse().unwrap());
let mut body = StructureOrUnion::new();
body.add_member(
    shape_name.make_member("errorMessage".parse().unwrap()),
    prelude.make_shape("String".parse().unwrap()),
);
let mut error = TopLevelShape::new(shape_name, ShapeKind::Structure(body));
error
    .apply_with_value(
        prelude.make_shape("error".parse().unwrap()),
        Some("client".to_string().into()),
    )
    .unwrap();

// ----------------------------------------------------------------------------------------
let shape_name = namespace.make_shape("GetMessageOutput".parse().unwrap());
let mut output = StructureOrUnion::new();
let mut message = MemberShape::new(
    shape_name.make_member("message".parse().unwrap()),
    prelude.make_shape("String".parse().unwrap()),
);
message
    .apply(prelude.make_shape("required".parse().unwrap()))
    .unwrap();
let _ = output.add_a_member(message);
let output = TopLevelShape::new(
    namespace.make_shape("GetMessageOutput".parse().unwrap()),
    ShapeKind::Structure(output),
);

// ----------------------------------------------------------------------------------------
let shape_name = namespace.make_shape("GetMessageInput".parse().unwrap());
let mut input = StructureOrUnion::new();
input.add_member(
    shape_name.make_member("date".parse().unwrap()),
    date.id().clone(),
);
let input = TopLevelShape::new(
    namespace.make_shape("GetMessageInput".parse().unwrap()),
    ShapeKind::Structure(input),
);

// ----------------------------------------------------------------------------------------
let mut get_message = Operation::default();
get_message.set_input_shape(&input);
get_message.set_output_shape(&output);
get_message.add_error_shape(&error);
let mut get_message = TopLevelShape::new(
    namespace.make_shape("GetMessage".parse().unwrap()),
    ShapeKind::Operation(get_message),
);
get_message
    .apply(prelude.make_shape("readonly".parse().unwrap()))
    .unwrap();

// ----------------------------------------------------------------------------------------
let mut message = Resource::default();
message.add_identifier(Identifier::new_unchecked("date"), date.id().clone());
message.set_read_operation_shape(&get_message);
let message = TopLevelShape::new(
    namespace.make_shape("Message".parse().unwrap()),
    ShapeKind::Resource(message),
);

// ----------------------------------------------------------------------------------------
let mut service = Service::new("2020-06-21");
service.add_resource_shape(&message);
let mut service = TopLevelShape::new(
    namespace.make_shape("MessageOfTheDay".parse().unwrap()),
    ShapeKind::Service(service),
);
service
    .apply_with_value(
        prelude.make_shape("documentation".parse().unwrap()),
        Value::String("Provides a Message of the day.".to_string()).into(),
    )
    .unwrap();

// ----------------------------------------------------------------------------------------
let mut model = Model::new(Version::V10);
model.add_shape(message);
model.add_shape(date);
model.add_shape(get_message);
model.add_shape(input);
model.add_shape(output);
model.add_shape(error);

println!("{:#?}", model);
```

# The Model Builder API Example

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

For more information, see [the Rust Atelier book](https://rust-atelier.dev/using/builder_api.html).

```rust
use atelier_core::error::ErrorSource;
use atelier_core::builder::values::{ArrayBuilder, ObjectBuilder};
use atelier_core::builder::{
    traits, ListBuilder, MemberBuilder, ModelBuilder, OperationBuilder, ResourceBuilder,
    ServiceBuilder, ShapeTraits, SimpleShapeBuilder, StructureBuilder, TraitBuilder,
};
use atelier_core::model::{Identifier, Model, ShapeID};
use atelier_core::Version;

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
    .into();
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
extern crate error_chain;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

#[macro_use]
extern crate paste;

use std::fmt::{Display, Formatter};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Versions of the Smithy specification.
///
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Hash, Copy)]
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

#[doc(hidden)]
#[macro_use]
mod macros;

pub mod action;

pub mod builder;

pub mod error;

pub mod io;

pub mod model;

pub mod prelude;

pub mod syntax;
