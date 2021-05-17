/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::ErrorKind;
use crate::model::{Identifier, NamespaceID, ShapeID};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ShapeName {
    Qualified(ShapeID),
    Local(Identifier),
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for ShapeName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ShapeName::Qualified(id) => id.to_string(),
                ShapeName::Local(id) => id.to_string(),
            }
        )
    }
}

impl From<ShapeID> for ShapeName {
    fn from(id: ShapeID) -> Self {
        Self::Qualified(id)
    }
}

impl From<Identifier> for ShapeName {
    fn from(id: Identifier) -> Self {
        Self::Local(id)
    }
}

impl FromStr for ShapeName {
    type Err = crate::error::Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        if ShapeID::is_valid(id) {
            Ok(Self::Qualified(ShapeID::from_str(id).unwrap()))
        } else if Identifier::is_valid(id) {
            Ok(Self::Local(Identifier::from_str(id).unwrap()))
        } else {
            Err(ErrorKind::InvalidShapeID(id.to_string()).into())
        }
    }
}

impl ShapeName {
    is_as! { qualified, Qualified, ShapeID }

    pub fn eq_qualified(&self, other: &ShapeID) -> bool {
        match self {
            ShapeName::Qualified(id) => id == other,
            _ => false,
        }
    }

    is_as! { local, Local, Identifier }

    pub fn eq_local(&self, other: &Identifier) -> bool {
        match self {
            ShapeName::Local(id) => id == other,
            _ => false,
        }
    }

    pub fn namespace(&self) -> Option<&NamespaceID> {
        match self {
            ShapeName::Qualified(id) => Some(id.namespace()),
            _ => None,
        }
    }

    pub fn shape_name(&self) -> &Identifier {
        match self {
            ShapeName::Qualified(id) => id.shape_name(),
            ShapeName::Local(id) => id,
        }
    }

    pub fn member_name(&self) -> Option<&Identifier> {
        match self {
            ShapeName::Qualified(id) => id.member_name().as_ref(),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
