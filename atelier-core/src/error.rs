/*!
Standard `Error`, `ErrorKind`, and `Result` types.
*/

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
        InvalidShapeID(v: String) {
            description("invalid shape ID format")
            display("invalid shape ID format: '{}'", v)
        }
        #[doc("invalid value variant")]
        InvalidValueVariant(expecting: String) {
            description("invalid value variant")
            display("invalid value variant, expecting a `Value::{}`", expecting)
        }
        #[doc("invalid error source, expecting 'client' or 'server'")]
        InvalidErrorSource(s: String) {
            description("invalid error source, expecting 'client' or 'server'")
            display("invalid error source, expecting 'client' or 'server', not {}", s)
        }
    }

    foreign_links {
        Fmt(::std::fmt::Error);
        Io(::std::io::Error);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ErrorSource {
    Client,
    Server,
}

pub trait AndPanic: Display {
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
