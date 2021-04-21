/*!
A Simple IRI (URN) naming scheme for Smithy shape identifiers, it requires the shape identifiers be
absolute and not relative.

# Mapping

1. The URI scheme MUST be exactly `urn`.
1. The URN scheme MUST be exactly `smithy`.
1. The _namespace-specific string_ (NSS) MUST be formatted as follows.
   1. The identifier's namespace component.
   1. The colon character, `':'`.
   1. The identifier's shape name component.
   1. **If** the Shape ID represents a member shape:
      1. The forward slash character, `'/'`.
      1. The identifier's member name component.

# Examples

* `example.namespace#shape` becomes `urn:smithy:example.namespace:shape`
* `example.namespace#shape$member` becomes `urn:smithy:example.namespace:shape/member`
*/

use atelier_core::model::ShapeID;
use atelier_core::syntax::{SHAPE_ID_ABSOLUTE_SEPARATOR, SHAPE_ID_MEMBER_SEPARATOR};
use rdftk_iri::{IRIRef, Scheme, IRI};
use std::str::FromStr;
use std::sync::Arc;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

const SHAPE_URN_ABSOLUTE_SEPARATOR: char = ':';

const SHAPE_URN_MEMBER_SEPARATOR: char = '/';

const URN_NID: &str = "smithy:";

///
/// Convert a Smithy `ShapeID` into an `IRIRef`.
///
pub fn shape_to_iri(value: &ShapeID) -> IRIRef {
    Arc::from(
        IRI::from_str(&format!(
            "urn:{}{}",
            URN_NID,
            value
                .to_string()
                .replace(
                    SHAPE_ID_ABSOLUTE_SEPARATOR,
                    &SHAPE_URN_ABSOLUTE_SEPARATOR.to_string()
                )
                .replace(
                    SHAPE_ID_MEMBER_SEPARATOR,
                    &SHAPE_URN_MEMBER_SEPARATOR.to_string()
                )
        ))
        .unwrap(),
    )
}

///
/// Convert an `IRIRef` into a Smithy `ShapeID`. This will fail if the provided `IRI` does not
/// conform to the format described in the module documentation.
///
pub fn iri_to_shape(iri: IRIRef) -> Result<ShapeID, String> {
    if iri.scheme() == &Some(Scheme::from_str("urn").unwrap()) {
        let path = iri.path().to_string();
        if let Some(path) = path.strip_prefix(URN_NID) {
            if path
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '.' || c == ':' || c == '/')
            {
                match ShapeID::from_str(
                    &path
                        .replace(
                            SHAPE_URN_ABSOLUTE_SEPARATOR,
                            &SHAPE_ID_ABSOLUTE_SEPARATOR.to_string(),
                        )
                        .replace(
                            SHAPE_URN_MEMBER_SEPARATOR,
                            &SHAPE_ID_MEMBER_SEPARATOR.to_string(),
                        ),
                ) {
                    Ok(shape_id) => Ok(shape_id),
                    Err(_) => Err(String::from("Could not parse into a Shape ID")),
                }
            } else {
                Err(String::from("Smithy URN contains invalid characters"))
            }
        } else {
            Err(String::from("Not a Smithy URN"))
        }
    } else {
        Err(String::from("Not a URN"))
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
        let urn: IRIRef = shape_to_iri(&ShapeID::new_unchecked("example.namespace", "shape", None));
        assert_eq!(
            urn.to_string(),
            "urn:smithy:example.namespace:shape".to_string()
        )
    }

    #[test]
    fn test_urn_formatted_member() {
        let urn: IRIRef = shape_to_iri(&ShapeID::new_unchecked(
            "example.namespace",
            "shape",
            Some("member"),
        ));
        assert_eq!(
            urn.to_string(),
            "urn:smithy:example.namespace:shape/member".to_string()
        )
    }

    #[test]
    fn test_parse_formatted_shape() {
        let result = IRI::from_str("urn:smithy:example.namespace:shape");
        assert!(result.is_ok());
        let urn = result.unwrap();
        assert_eq!(
            urn.to_string(),
            "urn:smithy:example.namespace:shape".to_string()
        )
    }

    #[test]
    fn test_parse_formatted_member() {
        let result = IRI::from_str("urn:smithy:example.namespace:shape/member");
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
            let result = iri_to_shape(Arc::from(IRI::from_str(test).unwrap()));
            println!("{:?}", result);
            assert!(result.is_err());
        }
    }
}
