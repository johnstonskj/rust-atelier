/*!
* This crate provides a Rust native core model for the AWS [Smithy](https://github.com/awslabs/smithy) Interface Definition Language.
*
* This crate is the foundation for the Atelier set of crates, and provides the following components:
*
* 1. The [model](model/index.html) elements themselves that represents a Smithy model. This API is the
*    in-memory representation shared by all Atelier crates and tools.
* 1. The model [builder](model/builder/index.html) API that allow for a more _fluent_ and less repetative construction of a
*    core model.
* 1. The [prelude](prelude/index.html) model containing the set of shapes defined in the Smithy specification.
* 1. Traits for [reading/writing](io/index.html) models in different representations.
* 1. Trait and simple implementation for a model [registry](registry/index.html).
* 1. A common [error](error/index.html) module to be used by all Atelier crates.
*
* ## Data Model
*
* The following is a diagrammatic representation of the core model. For the most part this is a
* direct transform from the ABNF in the specification, although some of the distinctions between
* different ID types (`Identifier`, `ShapeID`) are not illustrated. It also shows all the
* shape types as subclasses of `Shape`.
*
* ```text
* ┌───────────────┐
* │ «enumeration» │
* │   NodeValue   │
* ├───────────────┤                 ┌─────────┐
* │Array          │                 ○         ○ prelude
* │Object         │                ╱│╲        ┼
* │Number         │               ┌─────────────┐
* │Boolean        │metadata       │    Model    │
* │ShapeID        │┼○─────────────├─────────────┤
* │TextBlock      │               │namespace    │
* │String         │control_data   │             │┼──────┐       ┌─────────────┐
* │None           │┼○─────────────│             │       │       │   ShapeID   │
* └───────────────┘               └─────────────┘       │       ├─────────────┤
*   ┼     ┼     ┌───────────────┐        ┼              │      ╱│namespace?   │
*   │     │     │     Trait     │        │              └─────○─│shape_name   │
*   │     └─────├───────────────┤        │           references╲│member_name? │
*   │           │id             │        │                      │             │
*   │           └───────────────┘        │                      └─────────────┘
*   │             ╲│╱       ╲│╱          │                             ┼ id
*   │              ○         ○           │                             │
*   │     ┌────────┘         └───────┐   ○                             │
*   │     ┼                          ┼  ╱│╲ shapes                     │
* ┌───────────────┐               ┌─────────────┐                      │
* │    Member     │╲member┌──────┼│    Shape    │┼─────────────────────┘
* ├───────────────┤─○─────┘       └─────────────┘   ┌─────────────────────────┐
* │id             │╱                     △          │         Service         │
* └───────────────┘                      │          ├─────────────────────────┤
* ┌───────────────┐                      │          │version                  │
* │ «enumeration» │──────────────────────┼──────────│operations: [Operation]? │
* │    Simple     │ ┌────────────┐       │          │resources: [Resource]?   │
* ├───────────────┤ │    List    │       │          └─────────────────────────┘
* │Blob           │ ├────────────┤       │          ┌─────────────────────────┐
* │Boolean        │ │member      │───────┤          │        Operation        │
* │Document       │ └────────────┘       │          ├─────────────────────────┤
* │String         │ ┌────────────┐       │          │input: Structure?        │
* │Byte           │ │    Set     │       ├──────────│output: Structure?       │
* │Short          │ ├────────────┤       │          │errors: [Structure]?     │
* │Integer        │ │member      │───────┤          └─────────────────────────┘
* │Long           │ └────────────┘       │          ┌─────────────────────────┐
* │Float          │ ┌────────────┐       │          │        Resource         │
* │Double         │ │    Map     │       │          ├─────────────────────────┤
* │BigInteger     │ ├────────────┤       │          │identifiers?             │
* │BigDecimal     │ │key         │       │          │create: Operation?       │
* │Timestamp      │ │value       │───────┤          │put: Operation?          │
* └───────────────┘ └────────────┘       │          │read: Operation?         │
*                   ┌────────────┐       ├──────────│update: Operation?       │
*                   │ Structure  │───────┤          │delete: Operation?       │
*                   └────────────┘       │          │list: : Operation?       │
*                   ┌────────────┐       │          │operations: [Operation]? │
*                   │   Union    │───────┤          │collection_operations:   │
*                   └────────────┘       │          │    [Operation]?         │
*                   ┌────────────┐       │          │resources: [Resource]?   │
*                   │   Apply    │───────┘          └─────────────────────────┘
*                   └────────────┘
* ```
*
* # Model API Example
*
* The following example demonstrates the core model API to create a model for a simple service. The
* service, `MessageOfTheDay` has a single resource `Message`. The resource has an identifier for the
* date, but the `read` operation does not make the date member required and so will return the message
* for the current date.
*
* This API acts as a set of generic data objects and as such has a tendency to be verbose in the
* construction of models. The need to create a lot of `Identifier` and `ShapeID` instances, for example,
* does impact the readability.
*
* ```rust
* use atelier_core::model::shapes::{
*     Member, Operation, Resource, Service, Shape, ShapeBody, SimpleType, StructureOrUnion,
*     Trait, Valued,
* };
* use atelier_core::model::values::NodeValue;
* use atelier_core::model::{Annotated, Identifier, Model, Namespace, ShapeID};
* use atelier_core::Version;
* use std::str::FromStr;
*
* // ----------------------------------------------------------------------------------------
* let mut error = StructureOrUnion::new();
* error.add_member_value(
*     Identifier::from_str("errorMessage").unwrap(),
*     NodeValue::ShapeID(ShapeID::from_str("String").unwrap()),
* );
* let mut error = Shape::local(
*     Identifier::from_str("BadDateValue").unwrap(),
*     ShapeBody::Structure(error),
* );
* let mut error_trait = Trait::new(ShapeID::from_str("error").unwrap());
* error_trait.set_value(NodeValue::String("client".to_string()));
* error.add_trait(error_trait);
*
* // ----------------------------------------------------------------------------------------
* let mut output = StructureOrUnion::new();
* let mut message = Member::with_reference(
*     Identifier::from_str("message").unwrap(),
*     ShapeID::from_str("String").unwrap(),
* );
* let required = Trait::new(ShapeID::from_str("required").unwrap());
* message.add_trait(required);
* output.add_member(message);
* let output = Shape::local(
*     Identifier::from_str("GetMessageOutput").unwrap(),
*     ShapeBody::Structure(output),
* );
*
* // ----------------------------------------------------------------------------------------
* let mut input = StructureOrUnion::new();
* input.add_member_value(
*     Identifier::from_str("date").unwrap(),
*     NodeValue::ShapeID(ShapeID::from_str("Date").unwrap()),
* );
* let input = Shape::local(
*     Identifier::from_str("GetMessageInput").unwrap(),
*     ShapeBody::Structure(input),
* );
*
* // ----------------------------------------------------------------------------------------
* let mut get_message = Operation::default();
* get_message.set_input(ShapeID::from_str("GetMessageInput").unwrap());
* get_message.set_output(ShapeID::from_str("GetMessageOutput").unwrap());
* get_message.add_error(ShapeID::from_str("BadDateValue").unwrap());
* let mut get_message = Shape::local(
*     Identifier::from_str("GetMessage").unwrap(),
*     ShapeBody::Operation(get_message),
* );
* let required = Trait::new(ShapeID::from_str("readonly").unwrap());
* get_message.add_trait(required);
*
* // ----------------------------------------------------------------------------------------
* let mut date = Shape::local(
*     Identifier::from_str("Date").unwrap(),
*     ShapeBody::SimpleType(SimpleType::String),
* );
* let mut pattern_trait = Trait::new(ShapeID::from_str("pattern").unwrap());
* pattern_trait.set_value(NodeValue::String(r"^\d\d\d\d\-\d\d-\d\d$".to_string()));
* date.add_trait(pattern_trait);
*
* // ----------------------------------------------------------------------------------------
* let mut message = Resource::default();
* message.add_identifier(
*     Identifier::from_str("date").unwrap(),
*     ShapeID::from_str("Date").unwrap(),
* );
* message.set_read(ShapeID::from_str("GetMessage").unwrap());
* let message = Shape::local(
*     Identifier::from_str("Message").unwrap(),
*     ShapeBody::Resource(message),
* );
*
* // ----------------------------------------------------------------------------------------
* let mut service = Service::default();
* service.set_version("2020-06-21");
* service.add_resource(ShapeID::from_str("Message").unwrap());
* let mut service = Shape::local(
*     Identifier::from_str("MessageOfTheDay").unwrap(),
*     ShapeBody::Service(service),
* );
* let documentation = Trait::with_value(
*     ShapeID::from_str("documentation").unwrap(),
*     NodeValue::String("Provides a Message of the day.".to_string()),
* );
* service.add_trait(documentation);
*
* // ----------------------------------------------------------------------------------------
* let mut model = Model::new(Namespace::from_str("example.motd").unwrap(), Some(Version::V10));
* model.add_shape(message);
* model.add_shape(date);
* model.add_shape(get_message);
* model.add_shape(input);
* model.add_shape(output);
* model.add_shape(error);
* ```
*
* # Builder API Example
*
* The following example demonstrates the builder interface to create the same service as the example
* above. Hopefully this is more readable as it tends to be less repetative, uses  `&str` for
* identifiers, and includes helper functions for common traits for example. It provides this better
* _construction experience_ (there are no read methods on builder objects) by compromising two aspects:
*
* 1. The API itself is very repetative; this means the same method may be on multiple objects, but
* makes it easier to use. For example, you want to add the documentation trait to a shape, so you can:
*    1. construct a `Trait` entity using the core model and the `Builder::add_trait` method,
*    1. use the `TraitBuilder::documentation` method which also takes the string to use as the trait
*       value and returns a new `TraitBuilder`, or
*    1. use the `Builder::documentation` method that hides all the details of a trait and just takes
*       a string.
* 1. It hides a lot of the `Identifier` and `ShapeID` construction and so any of those calls to
*    `from_str` may fail when the code unwraps the result. This means the builder can panic in ways
*    the core model does not.
*
* ```rust
* use atelier_core::error::ErrorSource;
* use atelier_core::model::builder::values::{ArrayBuilder, ObjectBuilder};
* use atelier_core::model::builder::{
*     ShapeBuilder, ListBuilder, MemberBuilder, ModelBuilder, OperationBuilder, ResourceBuilder,
*     ServiceBuilder, SimpleShapeBuilder, StructureBuilder, TraitBuilder,
* };
* use atelier_core::model::{Identifier, Model, ShapeID};
* use atelier_core::Version;
*
* let model: Model = ModelBuilder::new("example.motd", Some(Version::V10))
*     .shape(
*         ServiceBuilder::new("MessageOfTheDay")
*             .documentation("Provides a Message of the day.")
*             .version("2020-06-21")
*             .resource("Message")
*             .into(),
*     )
*     .shape(
*         ResourceBuilder::new("Message")
*             .identifier("date", "Date")
*             .read("GetMessage")
*             .into(),
*     )
*     .shape(
*         SimpleShapeBuilder::string("Date")
*             .add_trait(TraitBuilder::pattern(r"^\d\d\d\d\-\d\d-\d\d$").into())
*             .into(),
*     )
*     .shape(
*         OperationBuilder::new("GetMessage")
*             .readonly()
*             .input("GetMessageInput")
*             .output("GetMessageOutput")
*             .error("BadDateValue")
*             .into(),
*     )
*     .shape(
*         StructureBuilder::new("GetMessageInput")
*             .add_member(
*                 MemberBuilder::new("date")
*                     .refers_to("Date")
*                     .into(),
*             )
*             .into(),
*     )
*     .shape(
*         StructureBuilder::new("GetMessageOutput")
*             .add_member(MemberBuilder::string("message").required().into())
*             .into(),
*     )
*     .shape(
*         StructureBuilder::new("BadDateValue")
*             .error(ErrorSource::Client)
*             .add_member(MemberBuilder::string("errorMessage").required().into())
*             .into(),
*     )
*     .into();
* ```
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

use crate::model::Model;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Versions of the Smithy specification.
///
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub enum Version {
    /// Version 1.0 (initial, and current)
    V10,
}

///
/// A trait implemented by tools that provide validation over a model.
///
pub trait Validator {
    ///
    /// Validate the model returning any error, or errors, it may contain.
    ///
    fn validate(&self, model: &Model) -> error::Result<()>;
}

///
/// A trait implemented by tools that transform one model into another.
///
/// This trait requires a corresponding validator to ensure the input model is correct according to
/// any rules required by the transformation itself.
///
pub trait Transformer: Validator {
    ///
    /// Transform a model into another, this will consume the original.
    ///
    fn transform(&self, model: Model) -> error::Result<Model>;
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

pub mod syntax;