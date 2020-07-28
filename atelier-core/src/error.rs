/*!
Standard `Error`, `ErrorKind`, and `Result` types.
*/

#![allow(missing_docs)]

use crate::action::ActionIssue;
use crate::model::identity::ShapeID;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

error_chain! {
    errors {
        #[doc("Invalid version number")]
        InvalidVersionNumber(v: String) {
            description("Invalid version number")
            display("Invalid version number: '{}'", v)
        }
        #[doc("Invalid shape ID format")]
        InvalidShapeID(id: String) {
            description("Invalid shape ID format")
            display("Invalid shape ID format: '{}'", id)
        }
        #[doc("Expected an absolute shape ID")]
        AbsoluteShapeIDExpected(id: ShapeID) {
            description("Expected an absolute shape ID")
            display("Expected an absolute shape ID: '{}'", id)
        }
        #[doc("Expected a shape, not member, ID")]
        ShapeIDExpected(id: ShapeID) {
            description("Expected a shape, not member, ID")
            display("Expected a shape, not member, ID: '{}'", id)
        }
        #[doc("Expected a member, not shape, ID")]
        MemberIDExpected(id: ShapeID) {
            description("Expected a member, not shape, ID")
            display("Expected a member, not shape, ID: '{}'", id)
        }
        #[doc("Invalid shape kind variant")]
        InvalidShapeVariant(expecting: String) {
            description("Invalid shape kind variant")
            display("Invalid shape kind variant, expecting a `ShapeKind::{}`", expecting)
        }
        #[doc("Invalid value variant")]
        InvalidValueVariant(expecting: String) {
            description("Invalid value variant")
            display("Invalid value variant, expecting a `Value::{}`", expecting)
        }
        #[doc("Invalid error source, expecting 'client' or 'server'")]
        InvalidErrorSource(src: String) {
            description("Invalid error source, expecting 'client' or 'server'")
            display("Invalid error source, expecting 'client' or 'server', not '{}'", src)
        }
        #[doc("Requested action is not supported by the selected representation")]
        InvalidRepresentation(repr: String) {
            description("Requested action is not supported by the selected representation")
            display("Requested action is not supported by the selected representation '{}'", repr)
        }
        #[doc("An error occurred serializing a model")]
        Serialization(repr: String) {
            description("An error occurred serializing a model")
            display("An error occurred serializing a model into {}", repr)
        }
        #[doc("An error occurred de-serializing a model")]
        Deserialization(representation: String, location: String, context: Option<String>) {
            description("An error occurred de-serializing a model")
            display("An error occurred de-serializing a model from {} at location '{}' (context '{:?}')", representation, location, context)
        }
        #[doc("A reference to an unknown shape ID was encountered")]
        UnknownShape(s: String) {
            description("A reference to an unknown shape ID was encountered")
            display("A reference to an unknown shape ID was encountered: {}", s)
        }
        #[doc("An unknown member ID was encountered")]
        UnknownMember(s: String) {
            description("An unknown member ID was encountered")
            display("An unknown member ID was encountered: {}", s)
        }
        #[doc("An unknown type-as-string encountered")]
        UnknownType(s: String) {
            description("An unknown type-as-string encountered")
            display("An unknown type-as-string encountered: {}", s)
        }
        #[doc("Reporting issues found by an Action.")]
        ActionIssue(reasons: Vec<ActionIssue>) {
            description("Reporting issues found by an Action.")
            display("Reporting issues found by an Action: {:?}", reasons)
        }
    }

    foreign_links {
        Fmt(::std::fmt::Error);
        Io(::std::io::Error);
    }
}

///
/// The identification of an error's source used by the `error` trait.
///
#[derive(Clone, Debug, PartialEq)]
pub enum ErrorSource {
    /// The error originated in the client.
    Client,
    /// The error originated in the server.
    Server,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for ErrorSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ErrorSource::Client => "client",
                ErrorSource::Server => "server",
            }
        )
    }
}

impl FromStr for ErrorSource {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "client" => Ok(Self::Client),
            "server" => Ok(Self::Server),
            _ => Err(ErrorKind::InvalidErrorSource(s.to_string()).into()),
        }
    }
}
