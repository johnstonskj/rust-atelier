use atelier_core::model::builder::{ModelBuilder, SimpleShapeBuilder, StructureBuilder};
use atelier_core::model::{Model, ShapeID};
use atelier_core::Version;
use std::str::FromStr;

fn make_example_model() -> Model {
    ModelBuilder::new("smithy.example", Some(Version::V10))
        .uses("foo.baz#Bar")
        .shape(SimpleShapeBuilder::string("MyString").into())
        .shape(
            StructureBuilder::new("MyStructure")
                .member("a", "MyString")
                .member("b", "smithy.example#MyString")
                .member("c", "Bar")
                .member("d", "foo.baz#Bar")
                .member("e", "foo.baz#MyString")
                .member("f", "String")
                .member("g", "MyBoolean")
                .member("h", "InvalidShape")
                .into(),
        )
        .into()
}

#[test]
fn test_resolve_unqualified_shape_id() {
    let model = make_example_model();
    assert_eq!(
        model.resolve_id(&ShapeID::from_str("MyString").unwrap(), true),
        Some(ShapeID::from_str("smithy.example#MyString").unwrap())
    );
    assert_eq!(
        model.resolve_id(&ShapeID::from_str("MyString").unwrap(), false),
        Some(ShapeID::from_str("smithy.example#MyString").unwrap())
    );
}

#[test]
fn test_resolve_qualified_shape_id() {
    let model = make_example_model();
    assert_eq!(
        model.resolve_id(&ShapeID::from_str("smithy.example#MyString").unwrap(), true),
        Some(ShapeID::from_str("smithy.example#MyString").unwrap())
    );
    assert_eq!(
        model.resolve_id(
            &ShapeID::from_str("smithy.example#MyString").unwrap(),
            false
        ),
        Some(ShapeID::from_str("smithy.example#MyString").unwrap())
    );
}

#[test]
fn test_resolve_unqualified_reference() {
    let model = make_example_model();
    assert_eq!(
        model.resolve_id(&ShapeID::from_str("Bar").unwrap(), true),
        Some(ShapeID::from_str("foo.baz#Bar").unwrap())
    );
    assert_eq!(
        model.resolve_id(&ShapeID::from_str("Bar").unwrap(), true),
        Some(ShapeID::from_str("foo.baz#Bar").unwrap())
    );
}

#[test]
fn test_resolve_qualified_reference() {
    let model = make_example_model();
    assert_eq!(
        model.resolve_id(&ShapeID::from_str("foo.baz#Bar").unwrap(), true),
        Some(ShapeID::from_str("foo.baz#Bar").unwrap())
    );
    assert_eq!(
        model.resolve_id(&ShapeID::from_str("foo.baz#Bar").unwrap(), false),
        Some(ShapeID::from_str("foo.baz#Bar").unwrap())
    );
}

#[test]
fn test_resolve_unqualified_unknown() {
    let model = make_example_model();
    assert_eq!(
        model.resolve_id(&ShapeID::from_str("InvalidShape").unwrap(), true),
        None
    );
    assert_eq!(
        model.resolve_id(&ShapeID::from_str("InvalidShape").unwrap(), false),
        None
    );
}

#[test]
fn test_resolve_qualified_unknown() {
    let model = make_example_model();
    assert_eq!(
        model.resolve_id(&ShapeID::from_str("foo.baz#MyString").unwrap(), true),
        None
    );
    assert_eq!(
        model.resolve_id(&ShapeID::from_str("foo.baz#MyString").unwrap(), false),
        Some(ShapeID::from_str("foo.baz#MyString").unwrap())
    );
}

#[test]
fn test_resolve_unqualified_prelude() {
    let model = make_example_model();
    assert_eq!(
        model.resolve_id(&ShapeID::from_str("String").unwrap(), true),
        Some(ShapeID::from_str("smithy.api#String").unwrap())
    );
    assert_eq!(
        model.resolve_id(&ShapeID::from_str("String").unwrap(), false),
        Some(ShapeID::from_str("smithy.api#String").unwrap())
    );
}

#[test]
fn test_resolve_qualified_prelude() {
    let model = make_example_model();
    assert_eq!(
        model.resolve_id(&ShapeID::from_str("smithy.api#Boolean").unwrap(), true),
        Some(ShapeID::from_str("smithy.api#Boolean").unwrap())
    );
    assert_eq!(
        model.resolve_id(&ShapeID::from_str("smithy.api#Boolean").unwrap(), false),
        Some(ShapeID::from_str("smithy.api#Boolean").unwrap())
    );
}

// ------------------------------------------------------------------------------------------------

#[test]
fn test_resolve_unqualified_member() {
    let model = make_example_model();
    assert_eq!(
        model.resolve_id(&ShapeID::from_str("MyStructure$a").unwrap(), true),
        Some(ShapeID::from_str("smithy.example#MyStructure$a").unwrap())
    );
    assert_eq!(
        model.resolve_id(&ShapeID::from_str("MyStructure$a").unwrap(), false),
        Some(ShapeID::from_str("smithy.example#MyStructure$a").unwrap())
    );
}

#[test]
fn test_resolve_qualified_member() {
    let model = make_example_model();
    assert_eq!(
        model.resolve_id(
            &ShapeID::from_str("smithy.example#MyStructure$a").unwrap(),
            true
        ),
        Some(ShapeID::from_str("smithy.example#MyStructure$a").unwrap())
    );
    assert_eq!(
        model.resolve_id(
            &ShapeID::from_str("smithy.example#MyStructure$a").unwrap(),
            false
        ),
        Some(ShapeID::from_str("smithy.example#MyStructure$a").unwrap())
    );
}

#[test]
fn test_resolve_unqualified_unknown_member() {
    let model = make_example_model();
    assert_eq!(
        model.resolve_id(&ShapeID::from_str("MyStructure$notPresent").unwrap(), true),
        None
    );
    assert_eq!(
        model.resolve_id(&ShapeID::from_str("MyStructure$notPresent").unwrap(), false),
        None
    );
}

#[test]
fn test_resolve_qualified_unknown_member() {
    let model = make_example_model();
    assert_eq!(
        model.resolve_id(
            &ShapeID::from_str("smithy.example#MyStructure$notPresent").unwrap(),
            true
        ),
        None
    );
    assert_eq!(
        model.resolve_id(
            &ShapeID::from_str("smithy.example#MyStructure$notPresent").unwrap(),
            false
        ),
        Some(ShapeID::from_str("smithy.example#MyStructure$notPresent").unwrap())
    );
}
