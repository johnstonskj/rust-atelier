use atelier_core::error::{Error, ErrorKind};
use atelier_core::model::ShapeID;
use atelier_core::syntax::{SHAPE_ID_ABSOLUTE_SEPARATOR, SHAPE_ID_MEMBER_SEPARATOR};
use regex::Regex;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A Simple URI/URN naming scheme for Smithy shape identifiers, it requires the shape identifiers be
/// absolute and not relative.
///
/// The following rules apply:
///
/// * The URI scheme is `urn`.
/// * The URN scheme is `smithy`.
/// * The _namespace-specific string_ (NSS) is formatted as follows.
/// * The identifier's namespace component is next, followed by ":"
/// * The identifier's shape name is next
/// * If present the identifier's member name is next, prefixed with "/"
///
/// # Examples
///
/// * `example.namespace#shape` becomes `urn:smithy:example.namespace:shape`
/// * `example.namespace#shape$member` becomes `urn:smithy:example.namespace:shape/member`
///
#[derive(Debug)]
pub struct SmithyUrn(ShapeID);

/// The character separating a `Namespace` and `Identifier` in an absolute `ShapeID`.
pub const SHAPE_URN_ABSOLUTE_SEPARATOR: char = ':';

/// The character separating the shape name and member name in a `ShapeID`.
pub const SHAPE_URN_MEMBER_SEPARATOR: char = '/';

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

const URN_SCHEME: &str = "urn:smithy:";

lazy_static! {
    static ref RE_SHAPE_URN: Regex = Regex::new(
        r"(?x)
        ^urn:smithy:
        (_*[[:alpha:]][_[[:alnum:]]]**(\._*[[:alpha:]][_[[:alnum:]]]*)*):
        (_*[[:alpha:]][_[[:alnum:]]]*)
        (/(_*[[:alpha:]][_[[:alnum:]]]*))?
        $"
    )
    .unwrap();
}

impl From<ShapeID> for SmithyUrn {
    fn from(value: ShapeID) -> Self {
        Self(value)
    }
}

impl From<&ShapeID> for SmithyUrn {
    fn from(value: &ShapeID) -> Self {
        Self(value.clone())
    }
}

impl Into<ShapeID> for SmithyUrn {
    fn into(self) -> ShapeID {
        self.0
    }
}

impl FromStr for SmithyUrn {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_valid(s) {
            let shape_id = &s[URN_SCHEME.len()..];
            Ok(Self(ShapeID::from_str(
                &shape_id
                    .replace(
                        SHAPE_URN_ABSOLUTE_SEPARATOR,
                        &SHAPE_ID_ABSOLUTE_SEPARATOR.to_string(),
                    )
                    .replace(
                        SHAPE_URN_MEMBER_SEPARATOR,
                        &SHAPE_ID_MEMBER_SEPARATOR.to_string(),
                    ),
            )?))
        } else {
            Err(ErrorKind::InvalidShapeID(s.to_string()).into())
        }
    }
}

impl Display for SmithyUrn {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            URN_SCHEME,
            self.0
                .to_string()
                .replace(
                    SHAPE_ID_ABSOLUTE_SEPARATOR,
                    &SHAPE_URN_ABSOLUTE_SEPARATOR.to_string()
                )
                .replace(
                    SHAPE_ID_MEMBER_SEPARATOR,
                    &SHAPE_URN_MEMBER_SEPARATOR.to_string()
                )
        )
    }
}

impl SmithyUrn {
    ///
    /// Returns `true` if the provided string is a valid URN representation, else `false`.
    /// This is preferred to calling `from_str()` and determining success or failure.
    ///
    pub fn is_valid(s: &str) -> bool {
        RE_SHAPE_URN.is_match(s)
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_urn_formatted_shape() {
        let urn: SmithyUrn = ShapeID::new_unchecked("example.namespace", "shape", None).into();
        assert_eq!(
            urn.to_string(),
            "urn:smithy:example.namespace:shape".to_string()
        )
    }

    #[test]
    fn test_urn_formatted_member() {
        let urn: SmithyUrn =
            ShapeID::new_unchecked("example.namespace", "shape", Some("member")).into();
        assert_eq!(
            urn.to_string(),
            "urn:smithy:example.namespace:shape/member".to_string()
        )
    }

    #[test]
    fn test_parse_formatted_shape() {
        let result = SmithyUrn::from_str("urn:smithy:example.namespace:shape");
        assert!(result.is_ok());
        let urn = result.unwrap();
        assert_eq!(
            urn.to_string(),
            "urn:smithy:example.namespace:shape".to_string()
        )
    }

    #[test]
    fn test_parse_formatted_member() {
        let result = SmithyUrn::from_str("urn:smithy:example.namespace:shape/member");
        assert!(result.is_ok());
        let urn = result.unwrap();
        assert_eq!(
            urn.to_string(),
            "urn:smithy:example.namespace:shape/member".to_string()
        )
    }

    #[test]
    fn test_invalid_formatted_urn() {
        let tests = [
            "Urn:smithy:example.namespace:shape/member",
            "urn:Smithy:example.namespace:shape/member",
            "smithy:example.namespace:shape/member",
            "urn:example.namespace:shape/member",
            "urn:smithy:shape/member",
            "urn:smithy::shape/member",
            "urn:smithy:example.namespace:shape/",
            "urn:smithy:example.namespace:shape$member",
            "urn:smithy:example.namespace#shape/member",
            "urn:smithy:example.namespace:shape/member/other",
        ];
        for test in tests.iter() {
            println!("invalid? {}", test);
            let result = SmithyUrn::from_str(test);
            println!("{:?}", result);
            assert!(result.is_err());
        }
    }
}
