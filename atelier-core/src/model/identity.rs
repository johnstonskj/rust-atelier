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

lazy_static! {
    static ref RE_IDENTIFIER: Regex =
        Regex::new(r"^([[:alpha:]]|_[[:alpha:]])[_[[:alnum:]]]*$").unwrap();
    static ref RE_NAMESPACE: Regex = Regex::new(
        r"^([[:alpha:]]|_[[:alpha:]])[_[[:alnum:]]]*(\.([[:alpha:]]|_[[:alpha:]])[_[[:alnum:]]]*)*$"
    )
    .unwrap();
    static ref RE_SHAPE_ID: Regex = Regex::new(
        r"(?x)
        ^
        ((([[:alpha:]]|_[[:alpha:]])[_[[:alnum:]]]*(\.([[:alpha:]]|_[[:alpha:]])[_[[:alnum:]]]*)*)\#)?
        (([[:alpha:]]|_[[:alpha:]])[_[[:alnum:]]]*)
        (\$(([[:alpha:]]|_[[:alpha:]])[_[[:alnum:]]]*))?
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
    ///
    /// Returns `true` if the provided string is a valid identifier representation, else `false`.
    /// This is preferred to calling `from_str()` and determining success or failure.
    ///
    pub fn is_valid(s: &str) -> bool {
        RE_IDENTIFIER.is_match(s)
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
        if let Some(result) = RE_SHAPE_ID.captures(s) {
            let namespace = match result.get(2) {
                Some(v) => Some(Namespace(v.as_str().to_string())),
                None => None,
            };
            let shape_name = Identifier(result.get(6).unwrap().as_str().to_string());
            let member_name = match result.get(9) {
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

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    const ID_GOOD: &[&str] = &["a", "aBc", "_aBc", "a1", "a1c", "a_c", "a_"];
    const ID_BAD: &[&str] = &["", "_", "1", "1a", "_1", "a!"];

    const NAMESPACE_GOOD: &[&str] = &["aBc", "aBc.dEf", "aBc.dEf.gHi"];
    const NAMESPACE_BAD: &[&str] = &["", ".aBc", "aBc."];

    const SHAPE_ID_GOOD: &[&str] = &[
        "aBc",
        "aBc#dEf",
        "aBc.dEf#gHi",
        "aBc$xYz",
        "aBc#dEf$xYz",
        "aBc.dEf#gHi$xYz",
    ];
    const SHAPE_ID_BAD: &[&str] = &[""];

    #[test]
    fn test_regexes() {
        // ----------------------------------------------------------------------------------------
        for id in ID_GOOD {
            assert!(RE_IDENTIFIER.is_match(id));
        }

        for id in ID_BAD {
            assert!(!RE_IDENTIFIER.is_match(id));
        }

        // ----------------------------------------------------------------------------------------
        for id in NAMESPACE_GOOD {
            assert!(RE_NAMESPACE.is_match(id));
        }

        for id in NAMESPACE_BAD {
            assert!(!RE_NAMESPACE.is_match(id));
        }

        // ----------------------------------------------------------------------------------------
        for id in SHAPE_ID_GOOD {
            assert!(RE_SHAPE_ID.is_match(id));
        }

        for id in SHAPE_ID_BAD {
            assert!(!RE_SHAPE_ID.is_match(id));
        }
    }

    #[test]
    fn test_is_value() {
        // ----------------------------------------------------------------------------------------
        for id in ID_GOOD {
            assert!(Identifier::is_valid(id));
        }

        for id in ID_BAD {
            assert!(!Identifier::is_valid(id));
        }

        // ----------------------------------------------------------------------------------------
        for id in NAMESPACE_GOOD {
            assert!(Namespace::is_valid(id));
        }

        for id in NAMESPACE_BAD {
            assert!(!Namespace::is_valid(id));
        }

        // ----------------------------------------------------------------------------------------
        for id in SHAPE_ID_GOOD {
            assert!(ShapeID::is_valid(id));
        }

        for id in SHAPE_ID_BAD {
            assert!(!ShapeID::is_valid(id));
        }
    }

    #[test]
    fn test_from_str() {
        // ----------------------------------------------------------------------------------------
        for id in ID_GOOD {
            assert!(Identifier::from_str(id).is_ok());
        }

        let shape_id = ShapeID::from_str("SomeShapeName").unwrap();
        assert!(shape_id.namespace().is_none());
        assert_eq!(
            shape_id.shape_name().to_string(),
            "SomeShapeName".to_string()
        );
        assert!(shape_id.member_name().is_none());

        let shape_id = ShapeID::from_str("com.example#SomeShapeName").unwrap();
        assert_eq!(
            shape_id.namespace(),
            &Some(Namespace::from_str("com.example").unwrap())
        );
        assert_eq!(
            shape_id.shape_name().to_string(),
            "SomeShapeName".to_string()
        );
        assert!(shape_id.member_name().is_none());

        let shape_id = ShapeID::from_str("com.example#SomeShapeName$aMember").unwrap();
        assert_eq!(
            shape_id.namespace(),
            &Some(Namespace::from_str("com.example").unwrap())
        );
        assert_eq!(
            shape_id.shape_name().to_string(),
            "SomeShapeName".to_string()
        );
        assert_eq!(
            shape_id.member_name(),
            &Some(Identifier::from_str("aMember").unwrap())
        );

        for id in ID_BAD {
            assert!(Identifier::from_str(id).is_err());
        }

        // ----------------------------------------------------------------------------------------
        for id in NAMESPACE_GOOD {
            assert!(Namespace::from_str(id).is_ok());
        }

        for id in NAMESPACE_BAD {
            assert!(Namespace::from_str(id).is_err());
        }

        // ----------------------------------------------------------------------------------------
        for id in SHAPE_ID_GOOD {
            assert!(ShapeID::from_str(id).is_ok());
        }

        for id in SHAPE_ID_BAD {
            assert!(ShapeID::from_str(id).is_err());
        }
    }
}
