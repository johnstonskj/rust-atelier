use crate::error;
use crate::syntax::{
    SHAPE_ID_ABSOLUTE_SEPARATOR, SHAPE_ID_MEMBER_SEPARATOR, SHAPE_ID_NAMESPACE_SEPARATOR,
};
use regex::Regex;
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
pub struct NamespaceID(String);

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
/// * `Namespace`; this is a list of `Identifier` components.
///   * Followed by the separator character `CHAR_SHAPE_ID_ABSOLUTE`.
/// * `Shape name`; an `Identifier` value.
/// * `Member name`; an optional `Identifier` value.
///   * Preceded by the separator character `CHAR_SHAPE_ID_MEMBER`.
///
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ShapeID {
    namespace: NamespaceID,
    shape_name: Identifier,
    member_name: Option<Identifier>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

lazy_static! {
    static ref RE_IDENTIFIER: Regex = Regex::new(r"^_*[[:alpha:]][_[[:alnum:]]]*$").unwrap();
    static ref RE_NAMESPACE: Regex =
        Regex::new(r"^_*[[:alpha:]][_[[:alnum:]]]*(\._*[[:alpha:]][_[[:alnum:]]]*)*$").unwrap();
    static ref RE_SHAPE_ID: Regex = Regex::new(
        r"(?x)
        ^
        (_*[[:alpha:]][_[[:alnum:]]]**(\._*[[:alpha:]][_[[:alnum:]]]*)*)\#
        (_*[[:alpha:]][_[[:alnum:]]]*)
        (\$(_*[[:alpha:]][_[[:alnum:]]]*))?
        $"
    )
    .unwrap();
}

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
    pub fn new_unchecked(s: &str) -> Self {
        Self(s.to_string())
    }

    ///
    /// Returns `true` if the provided string is a valid identifier representation, else `false`.
    /// This is preferred to calling `from_str()` and determining success or failure.
    ///
    pub fn is_valid(s: &str) -> bool {
        RE_IDENTIFIER.is_match(s)
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for NamespaceID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for NamespaceID {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_valid(s) {
            Ok(Self(s.to_string()))
        } else {
            Err(error::ErrorKind::InvalidShapeID(s.to_string()).into())
        }
    }
}

impl PartialEq for NamespaceID {
    ///
    /// § 2.4.2. Shape ID member names
    /// While shape IDs used within a model are case-sensitive, no two shapes in the model can have the same case-insensitive shape ID.
    ///
    fn eq(&self, other: &Self) -> bool {
        self.0.to_lowercase() == other.0.to_lowercase()
    }
}

impl NamespaceID {
    pub fn new_unchecked(s: &str) -> Self {
        Self(s.to_string())
    }

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
    pub fn make_shape(&self, shape_name: Identifier) -> ShapeID {
        ShapeID::new(self.clone(), shape_name, None)
    }

    ///
    /// Returns a new absolute `ShapeID` with the shape name and member name appended to the
    /// current namespace.
    ///
    pub fn make_member(&self, shape_name: Identifier, member_name: Identifier) -> ShapeID {
        ShapeID::new(self.clone(), shape_name, Some(member_name))
    }

    ///
    /// Return the current namespace as an iterator over the individual identifiers within it.
    ///
    pub fn split(&self) -> impl Iterator<Item = Identifier> + '_ {
        self.0
            .split(SHAPE_ID_NAMESPACE_SEPARATOR)
            .map(|s| Identifier::new_unchecked(s))
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for ShapeID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.namespace, SHAPE_ID_ABSOLUTE_SEPARATOR)?;
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
        if let Some(result) = RE_SHAPE_ID.captures(s) {
            let namespace = NamespaceID(result.get(1).unwrap().as_str().to_string());
            let shape_name = Identifier(result.get(3).unwrap().as_str().to_string());
            let member_name = match result.get(5) {
                Some(v) => Some(Identifier(v.as_str().to_string())),
                None => None,
            };
            Ok(Self {
                namespace,
                shape_name,
                member_name,
            })
        } else {
            Err(error::ErrorKind::InvalidShapeID(s.to_string()).into())
        }
    }
}

impl ShapeID {
    ///
    /// Returns `true` if the provided string is a valid shape identifier representation, else `false`.
    /// This is preferred to calling `from_str()` and determining success or failure.
    ///
    pub fn is_valid(s: &str) -> bool {
        RE_SHAPE_ID.is_match(s)
    }

    ///
    /// Construct a new `ShapeID` from the complete set of given components.
    ///
    pub fn new(
        namespace: NamespaceID,
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
    /// Construct a new `ShapeID` from the complete set of given components.
    ///
    pub fn new_unchecked(namespace: &str, shape_name: &str, member_name: Option<&str>) -> Self {
        Self::new(
            NamespaceID::new_unchecked(namespace),
            Identifier::new_unchecked(shape_name),
            match member_name {
                None => None,
                Some(member_name) => Some(Identifier::new_unchecked(member_name)),
            },
        )
    }

    ///
    /// Constructs a new absolute `ShapeID` with the given namespace and shape name.
    ///
    pub fn shape(namespace: &str, shape_name: &str) -> Self {
        Self::new(
            namespace.parse().unwrap(),
            shape_name.parse().unwrap(),
            None,
        )
    }
    ///
    /// Constructs a new absolute `ShapeID` with the given namespace, shape name, and member name.
    ///
    pub fn member(namespace: &str, shape_name: &str, member_name: &str) -> Self {
        Self::new(
            namespace.parse().unwrap(),
            shape_name.parse().unwrap(),
            Some(member_name.parse().unwrap()),
        )
    }

    ///
    /// Returns the current namespace component.
    ///
    pub fn namespace(&self) -> &NamespaceID {
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
    /// Returns `true` if this `ShapeID` has a member name component, else `false`.
    ///
    pub fn is_member(&self) -> bool {
        self.member_name.is_some()
    }

    ///
    /// Returns `true` if the shape `other` shares the same namespace as `self`, else `false`.
    ///
    pub fn is_in_same_namespace(&self, other: &Self) -> bool {
        self.namespace == other.namespace
    }

    ///
    /// Returns `true` if the shape `other` is a valid member identifier if `self` is a valid shape.
    /// This implies, 1) that self is not a member name, 2) other and self have the same namespace
    /// and shape name, 3) `other` has a member name.
    ///
    pub fn is_valid_member(&self, other: &Self) -> bool {
        !self.is_member()
            && other.is_member()
            && self.is_in_same_namespace(other)
            && self.shape_name == other.shape_name
    }

    ///
    /// Return a new shape ID with the current namespace shape name and any member_name unchanged
    /// but with the shape name set to the new provided value.
    ///
    pub fn make_shape(&self, shape_name: Identifier) -> Self {
        Self {
            shape_name,
            ..self.clone()
        }
    }

    ///
    /// Return a new shape ID with the current namespace shape name unchanged but with the member
    /// name set.
    ///
    pub fn make_member(&self, member_name: Identifier) -> Self {
        Self {
            member_name: Some(member_name),
            ..self.clone()
        }
    }
}
