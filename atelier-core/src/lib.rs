/*!
Rust native core model for the AWS Smithy IDL.
*/

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

pub mod registry;
