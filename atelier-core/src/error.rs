/*!
Standard `Error`, `ErrorKind`, and `Result` types.
*/

use std::fmt::Display;

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
    }

    foreign_links {
        Fmt(::std::fmt::Error);
        Io(::std::io::Error);
    }
}

pub trait AndPanic: Display {
    fn panic(&self) -> ! {
        panic!(self.to_string())
    }
}

impl AndPanic for ErrorKind {}

pub(crate) fn invalid_value_variant(var: &str) -> ! {
    ErrorKind::InvalidValueVariant(var.to_string()).panic()
}
