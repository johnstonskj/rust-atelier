use atelier_core::error::{Error, ErrorKind};
use atelier_core::model::ShapeID;
use std::convert::TryFrom;
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

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

const URN_SCHEME: &str = "urn:smithy:";

impl TryFrom<ShapeID> for SmithyUrn {
    type Error = Error;

    fn try_from(value: ShapeID) -> Result<Self, Self::Error> {
        if value.is_absolute() {
            Ok(Self(value))
        } else {
            Err(ErrorKind::AbsoluteShapeIDExpected(value.to_string()).into())
        }
    }
}

impl TryFrom<&ShapeID> for SmithyUrn {
    type Error = Error;

    fn try_from(value: &ShapeID) -> Result<Self, Self::Error> {
        Self::try_from(value.clone())
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
        if s.starts_with(URN_SCHEME) {
            let shape_id = &s[URN_SCHEME.len()..];
            Ok(Self(ShapeID::from_str(
                &shape_id.replace(':', "#").replace('/', "$"),
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
            self.0.to_string().replace('#', ":").replace('$', "/")
        )
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use atelier_core::error::Error;
    use std::convert::TryInto;

    #[test]
    fn test_urn_relative_shape() {
        let result: Result<SmithyUrn, Error> =
            ShapeID::new_unchecked(None, "shape", None).try_into();
        assert!(result.is_err());
    }

    #[test]
    fn test_urn_formatted_shape() {
        let result: Result<SmithyUrn, Error> =
            ShapeID::new_unchecked(Some("example.namespace"), "shape", None).try_into();
        assert!(result.is_ok());
        let urn = result.unwrap();
        assert_eq!(
            urn.to_string(),
            "urn:smithy:example.namespace:shape".to_string()
        )
    }

    #[test]
    fn test_urn_formatted_member() {
        let result: Result<SmithyUrn, Error> =
            ShapeID::new_unchecked(Some("example.namespace"), "shape", Some("member")).try_into();
        assert!(result.is_ok());
        let urn = result.unwrap();
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
        for test in tests.into_iter() {
            let result = SmithyUrn::from_str(test);
            assert!(result.is_err());
        }
    }
}
