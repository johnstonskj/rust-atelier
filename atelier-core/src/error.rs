/*!
Standard `Error`, `ErrorKind`, and `Result` types.
*/

#![allow(missing_docs)]

use crate::action::ActionIssue;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

error_chain! {
    errors {
        #[doc("invalid version number")]
        InvalidVersionNumber(v: String) {
            description("invalid version number")
            display("invalid version number: '{}'", v)
        }
        #[doc("invalid shape ID format")]
        InvalidShapeID(id: String) {
            description("invalid shape ID format")
            display("invalid shape ID format: '{}'", id)
        }
        #[doc("invalid value variant")]
        InvalidValueVariant(expecting: String) {
            description("invalid value variant")
            display("invalid value variant, expecting a `Value::{}`", expecting)
        }
        #[doc("invalid error source, expecting 'client' or 'server'")]
        InvalidErrorSource(src: String) {
            description("invalid error source, expecting 'client' or 'server'")
            display("invalid error source, expecting 'client' or 'server', not {}", src)
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

///
/// Allows any value that implements `Display` to be the message in a panic.
///
pub trait AndPanic: Display {
    ///
    /// Call `panic!` using the string serialization of the current value.
    ///
    fn panic(&self) -> ! {
        panic!(self.to_string())
    }
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub(crate) fn invalid_value_variant(var: &str) -> ! {
    ErrorKind::InvalidValueVariant(var.to_string()).panic()
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl AndPanic for ErrorKind {}

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
