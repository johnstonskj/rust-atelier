use crate::error;
use crate::syntax::{
    SHAPE_ID_ABSOLUTE_SEPARATOR, SHAPE_ID_MEMBER_SEPARATOR, SHAPE_ID_NAMESPACE_SEPARATOR,
};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A component of `ShapeID`, it represents an internal unqualified identifier.
///
/// Corresponds to the `identifier` production in §2.4.1,
///   [Shape ID ABNF](https://awslabs.github.io/smithy/1.0/spec/core/lexical-structure.html#shape-id-abnf),
///   of the Smithy 1.0 Specification.
///
#[allow(clippy::derive_hash_xor_eq)]
#[derive(Clone, Debug, Eq, Hash)]
pub struct Identifier(String);

///
/// A component of `ShapeID`, it represents the namespace of a model, or the namespace for a
/// qualified identifier. The separator character is `CHAR_NAMESPACE_SEPARATOR`.
///
/// Corresponds to the `namespace` production in §2.4.1,
///   [Shape ID ABNF](https://awslabs.github.io/smithy/1.0/spec/core/lexical-structure.html#shape-id-abnf),
///   of the Smithy 1.0 Specification.
///
#[allow(clippy::derive_hash_xor_eq)]
#[derive(Clone, Debug, Eq, Hash)]
pub struct Namespace(String);

///
/// The complete shape identifier type used across model structures, it is qualified with a namespace
/// and may also include an inner member identifier.
///
/// Corresponds to the `shape_id` production in §2.4.1,
///   [Shape ID ABNF](https://awslabs.github.io/smithy/1.0/spec/core/lexical-structure.html#shape-id-abnf),
///   of the Smithy 1.0 Specification.
///
/// ```abnf
/// com.foo.baz#ShapeName$memberName
/// \_________/ \_______/ \________/
/// |          |          |
/// Namespace  Shape name  Member name
/// ```
///
/// * `ShapeID`; comprises the 3-tuple described above, with components as follows:
/// * `Namespace`; the optional `Namespace` struct is a list of `Identifier` components.
///   * Followed by the separator character `CHAR_SHAPE_ID_ABSOLUTE`.
/// * `Shape name`; an `Identifier` value.
/// * `Member name`; an optional `Identifier` value.
///   * Preceded by the separator character `CHAR_SHAPE_ID_MEMBER`.
///
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ShapeID {
    namespace: Option<Namespace>,
    shape_name: Identifier,
    member_name: Option<Identifier>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

const CHAR_UNDERSCORE: char = '_';

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
    ///
    /// Returns `true` if the provided string is a valid identifier representation, else `false`.
    /// This is preferred to calling `from_str()` and determining success or failure.
    ///
    pub fn is_valid(s: &str) -> bool {
        !s.is_empty()
            && s.starts_with(|c: char| c.is_alphabetic() || c == CHAR_UNDERSCORE)
            && s.chars()
                .all(|c: char| c.is_alphanumeric() || c == CHAR_UNDERSCORE)
    }

    ///
    /// Returns a new relative `ShapeID` with the member name appended to the current shape.
    ///
    pub fn to_member(&self, member_name: Identifier) -> ShapeID {
        ShapeID {
            namespace: None,
            shape_name: self.clone(),
            member_name: Some(member_name),
        }
    }

    ///
    /// Returns a new absolute `ShapeID` with the namespace prepended to the current shape.
    ///
    pub fn to_absolute(&self, ns: Namespace) -> ShapeID {
        ShapeID {
            namespace: Some(ns),
            shape_name: self.clone(),
            member_name: None,
        }
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
    ///
    /// Returns `true` if the provided string is a valid namespace representation, else `false`.
    /// This is preferred to calling `from_str()` and determining success or failure.
    ///
    pub fn is_valid(s: &str) -> bool {
        for id in s.split(SHAPE_ID_NAMESPACE_SEPARATOR) {
            if !Identifier::is_valid(id) {
                return false;
            }
        }
        true
    }

    ///
    /// Returns a new absolute `ShapeID` with the shape name appended to the current namespace.
    ///
    pub fn to_shape(&self, shape_name: Identifier) -> ShapeID {
        ShapeID {
            namespace: Some(self.clone()),
            shape_name,
            member_name: None,
        }
    }

    ///
    /// Returns a new absolute `ShapeID` with the shape name and member name appended to the
    /// current namespace.
    ///
    pub fn to_member(&self, shape_name: Identifier, member_name: Identifier) -> ShapeID {
        ShapeID {
            namespace: Some(self.clone()),
            shape_name,
            member_name: Some(member_name),
        }
    }

    ///
    /// Return the current namespace as an iterator over the individual identifiers within it.
    ///
    pub fn split(&self) -> impl Iterator<Item = Identifier> + '_ {
        self.0
            .split(SHAPE_ID_NAMESPACE_SEPARATOR)
            .map(|s| Identifier(s.to_string()))
    }
}

// ------------------------------------------------------------------------------------------------

impl From<Identifier> for ShapeID {
    fn from(shape_name: Identifier) -> Self {
        Self {
            namespace: None,
            shape_name,
            member_name: None,
        }
    }
}

impl Display for ShapeID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(namespace) = &self.namespace {
            write!(f, "{}{}", namespace, SHAPE_ID_ABSOLUTE_SEPARATOR)?;
        }
        write!(f, "{}", self.shape_name)?;
        if let Some(member_name) = &self.member_name {
            write!(f, "{}{}", SHAPE_ID_MEMBER_SEPARATOR, member_name)?;
        }
        Ok(())
    }
}

impl FromStr for ShapeID {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s
            .split(SHAPE_ID_ABSOLUTE_SEPARATOR)
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let (namespace, rest) = if parts.len() == 1 {
            (None, parts.get(0).unwrap())
        } else if parts.len() == 2 {
            let namespace = parts.get(0).unwrap();
            (Some(Namespace::from_str(namespace)?), parts.get(1).unwrap())
        } else {
            return Err(error::ErrorKind::InvalidShapeID(s.to_string()).into());
        };

        let parts = rest
            .split(SHAPE_ID_MEMBER_SEPARATOR)
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
    ///
    /// Construct a new `ShapeID` from the complete set of given components.
    ///
    pub fn new(
        namespace: Option<Namespace>,
        shape_name: Identifier,
        member_name: Option<Identifier>,
    ) -> Self {
        Self {
            namespace,
            shape_name,
            member_name,
        }
    }

    ///
    /// Constructs a new relative `ShapeID` with the given shape name.
    ///
    pub fn shape(shape_name: &str) -> Self {
        Self {
            namespace: None,
            shape_name: shape_name.parse().unwrap(),
            member_name: None,
        }
    }

    ///
    /// Constructs a new absolute `ShapeID` with the given namespace and shape name.
    ///
    pub fn absolute_shape(namespace: &str, shape_name: &str) -> Self {
        Self {
            namespace: Some(namespace.parse().unwrap()),
            shape_name: shape_name.parse().unwrap(),
            member_name: None,
        }
    }

    ///
    /// Constructs a new relative `ShapeID` with the given shape name and member name.
    ///
    pub fn member(shape_name: &str, member_name: &str) -> Self {
        Self {
            namespace: None,
            shape_name: shape_name.parse().unwrap(),
            member_name: Some(member_name.parse().unwrap()),
        }
    }

    ///
    /// Constructs a new absolute `ShapeID` with the given namespace, shape name, and member name.
    ///
    pub fn absolute_member(namespace: &str, shape_name: &str, member_name: &str) -> Self {
        Self {
            namespace: Some(namespace.parse().unwrap()),
            shape_name: shape_name.parse().unwrap(),
            member_name: Some(member_name.parse().unwrap()),
        }
    }

    ///
    /// Returns the current namespace component.
    ///
    pub fn namespace(&self) -> &Option<Namespace> {
        &self.namespace
    }

    ///
    /// Returns the current shape name component.
    ///
    pub fn shape_name(&self) -> &Identifier {
        &self.shape_name
    }

    ///
    /// Returns the current member name component.
    ///
    pub fn member_name(&self) -> &Option<Identifier> {
        &self.member_name
    }

    ///
    /// Returns `true` if this `ShapeID` has a namespace component, else `false`.
    ///
    pub fn is_absolute(&self) -> bool {
        self.namespace.is_some()
    }

    ///
    /// Returns `true` if this `ShapeID` does not have a namespace component, else `false`.
    ///
    pub fn is_relative(&self) -> bool {
        self.namespace.is_none()
    }

    ///
    /// Returns `true` if this `ShapeID` has a member name component, else `false`.
    ///
    pub fn is_member(&self) -> bool {
        self.member_name.is_some()
    }

    ///
    /// Return a new shape ID with the current namespace shape name unchanged but with the member
    /// name set.
    ///
    pub fn to_member(&self, member_name: Identifier) -> Self {
        Self {
            member_name: Some(member_name),
            ..self.clone()
        }
    }

    ///
    /// Return a new shape ID with the current shape and member IDs unchanged but with the namespace
    /// included.
    ///
    pub fn to_absolute(&self, namespace: Namespace) -> Self {
        Self {
            namespace: Some(namespace),
            ..self.clone()
        }
    }

    ///
    /// Return a new shape ID with the current shape and member IDs unchanged but with any namespace
    /// removed.
    ///
    pub fn to_relative(&self) -> Self {
        if self.is_absolute() {
            Self {
                namespace: None,
                ..self.clone()
            }
        } else {
            self.clone()
        }
    }
}
