use atelier_core::model::identity::{Identifier, NamespaceID, ShapeID};
use std::str::FromStr;

const ID_GOOD: &[&str] = &["a", "aBc", "_aBc", "___aBc", "a1", "a1c", "a_c", "a_"];
const ID_BAD: &[&str] = &["", "_", "1", "1a", "_1", "a!"];

const NAMESPACE_GOOD: &[&str] = &["aBc", "aBc.dEf", "aBc.dEf.gHi"];
const NAMESPACE_BAD: &[&str] = &["", ".aBc", "aBc."];

const SHAPE_ID_GOOD: &[&str] = &["aBc#dEf", "aBc.dEf#gHi", "aBc#dEf$xYz", "aBc.dEf#gHi$xYz"];
const SHAPE_ID_BAD: &[&str] = &["", "aBc", "aBc$xYz"];

// ------------------------------------------------------------------------------------------------

#[test]
fn test_identifier_is_valid() {
    for id in ID_GOOD {
        assert!(Identifier::is_valid(id));
    }

    for id in ID_BAD {
        assert!(!Identifier::is_valid(id));
    }
}

#[test]
fn test_namespace_is_valid() {
    for id in NAMESPACE_GOOD {
        assert!(NamespaceID::is_valid(id));
    }

    for id in NAMESPACE_BAD {
        assert!(!NamespaceID::is_valid(id));
    }
}

#[test]
fn test_shape_id_is_valid() {
    for id in SHAPE_ID_GOOD {
        assert!(ShapeID::is_valid(id));
    }

    for id in SHAPE_ID_BAD {
        assert!(!ShapeID::is_valid(id));
    }
}

// ------------------------------------------------------------------------------------------------

#[test]
fn test_identifier_from_str() {
    for id in ID_GOOD {
        assert!(Identifier::from_str(id).is_ok());
    }

    for id in ID_BAD {
        assert!(Identifier::from_str(id).is_err());
    }
}

#[test]
fn test_namespace_from_str() {
    for id in NAMESPACE_GOOD {
        assert!(NamespaceID::from_str(id).is_ok());
    }

    for id in NAMESPACE_BAD {
        assert!(NamespaceID::from_str(id).is_err());
    }
}

#[test]
fn test_shape_id_from_str() {
    for id in SHAPE_ID_GOOD {
        assert!(ShapeID::from_str(id).is_ok());
    }

    for id in SHAPE_ID_BAD {
        assert!(ShapeID::from_str(id).is_err());
    }
}

// ------------------------------------------------------------------------------------------------

#[test]
fn test_shape_id_parts() {
    let shape_id = ShapeID::from_str("com.example#SomeShapeName").unwrap();
    assert_eq!(
        shape_id.namespace(),
        &NamespaceID::from_str("com.example").unwrap()
    );
    assert_eq!(
        shape_id.shape_name().to_string(),
        "SomeShapeName".to_string()
    );
    assert!(shape_id.member_name().is_none());

    let shape_id = ShapeID::from_str("com.example#SomeShapeName$aMember").unwrap();
    assert_eq!(
        shape_id.namespace(),
        &NamespaceID::from_str("com.example").unwrap()
    );
    assert_eq!(
        shape_id.shape_name().to_string(),
        "SomeShapeName".to_string()
    );
    assert_eq!(
        shape_id.member_name(),
        &Some(Identifier::from_str("aMember").unwrap())
    );
}
