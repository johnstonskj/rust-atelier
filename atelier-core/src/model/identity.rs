/*!
Identifier types used across model structures.

*/

use crate::error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[allow(clippy::derive_hash_xor_eq)]
#[derive(Clone, Debug, Eq, Hash)]
pub struct Identifier(String);

#[allow(clippy::derive_hash_xor_eq)]
#[derive(Clone, Debug, Eq, Hash)]
pub struct Namespace(String);

///
/// ```abnf
/// com.foo.baz#ShapeName$memberName
/// \_________/ \_______/ \________/
///      |          |          |
///  Namespace  Shape name  Member name
/// ```
///
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ShapeID {
    namespace: Option<Namespace>,
    shape_name: Identifier,
    member_name: Option<Identifier>,
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

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Identifier {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_valid(s) {
            Ok(Self(s.to_string()))
        } else {
            Err(error::ErrorKind::InvalidShapeID(s.to_string()).into())
        }
    }
}

impl PartialEq for Identifier {
    ///
    /// § 2.4.2. Shape ID member names
    /// While shape IDs used within a model are case-sensitive, no two shapes in the model can have the same case-insensitive shape ID.
    ///
    fn eq(&self, other: &Self) -> bool {
        self.0.to_lowercase() == other.0.to_lowercase()
    }
}

impl Identifier {
    pub fn is_valid(s: &str) -> bool {
        !s.is_empty()
            && s.starts_with(|c: char| c.is_alphabetic() || c == '_')
            && s.chars().all(|c: char| c.is_alphanumeric() || c == '_')
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for Namespace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Namespace {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_valid(s) {
            Ok(Self(s.to_string()))
        } else {
            Err(error::ErrorKind::InvalidShapeID(s.to_string()).into())
        }
    }
}

impl PartialEq for Namespace {
    ///
    /// § 2.4.2. Shape ID member names
    /// While shape IDs used within a model are case-sensitive, no two shapes in the model can have the same case-insensitive shape ID.
    ///
    fn eq(&self, other: &Self) -> bool {
        self.0.to_lowercase() == other.0.to_lowercase()
    }
}

impl Namespace {
    pub fn is_valid(s: &str) -> bool {
        for id in s.split('.') {
            if !Identifier::is_valid(id) {
                return false;
            }
        }
        true
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for ShapeID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(namespace) = &self.namespace {
            write!(f, "{}#", namespace)?;
        }
        write!(f, "{}", self.shape_name)?;
        if let Some(member_name) = &self.member_name {
            write!(f, "${}", member_name)?;
        }
        Ok(())
    }
}

impl FromStr for ShapeID {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split('#').map(|s| s.to_string()).collect::<Vec<String>>();
        let (namespace, rest) = if parts.len() == 1 {
            (None, parts.get(0).unwrap())
        } else if parts.len() == 2 {
            let namespace = parts.get(0).unwrap();
            (Some(Namespace::from_str(namespace)?), parts.get(1).unwrap())
        } else {
            return Err(error::ErrorKind::InvalidShapeID(s.to_string()).into());
        };

        let parts = rest
            .split('$')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let (shape_name, member_name) = if parts.len() <= 2 {
            let shape_name = Identifier::from_str(parts.get(0).unwrap())?;
            let member_name = if parts.len() == 1 {
                None
            } else {
                Some(Identifier::from_str(parts.get(1).unwrap())?)
            };
            (shape_name, member_name)
        } else {
            return Err(error::ErrorKind::InvalidShapeID(s.to_string()).into());
        };

        Ok(Self {
            namespace,
            shape_name,
            member_name,
        })
    }
}

impl ShapeID {
    pub fn new(namespace: &str, shape_name: &str, member_name: &str) -> Self {
        Self {
            namespace: Some(namespace.parse().unwrap()),
            shape_name: shape_name.parse().unwrap(),
            member_name: Some(member_name.parse().unwrap()),
        }
    }

    pub fn shape(shape_name: &str) -> Self {
        Self {
            namespace: None,
            shape_name: shape_name.parse().unwrap(),
            member_name: None,
        }
    }

    pub fn with_namespace(namespace: &str, shape_name: &str) -> Self {
        Self {
            namespace: Some(namespace.parse().unwrap()),
            shape_name: shape_name.parse().unwrap(),
            member_name: None,
        }
    }

    pub fn with_member_name(shape_name: &str, member_name: &str) -> Self {
        Self {
            namespace: None,
            shape_name: shape_name.parse().unwrap(),
            member_name: Some(member_name.parse().unwrap()),
        }
    }

    pub fn namespace(&self) -> &Option<Namespace> {
        &self.namespace
    }

    pub fn shape_name(&self) -> &Identifier {
        &self.shape_name
    }

    pub fn member_name(&self) -> &Option<Identifier> {
        &self.member_name
    }

    pub fn is_absolute(&self) -> bool {
        self.namespace.is_some()
    }

    pub fn is_relative(&self) -> bool {
        self.namespace.is_none()
    }

    pub fn into_absolute<S>(self, namespace: Namespace) -> Self {
        Self {
            namespace: Some(namespace),
            ..self
        }
    }

    pub fn into_relative(self) -> Self {
        Self {
            namespace: None,
            ..self
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
