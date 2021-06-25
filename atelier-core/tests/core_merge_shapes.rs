use atelier_core::model::shapes::{HasTraits, ShapeKind, StructureOrUnion, TopLevelShape};
use atelier_core::model::shapes::{ListOrSet, Map, Simple};
use atelier_core::model::values::Value;
use atelier_core::model::{Identifier, Model, ShapeID};
use atelier_core::prelude::{prelude_shape_named, TRAIT_DOCUMENTATION};
use atelier_core::Version;

// ------------------------------------------------------------------------------------------------
// Merge succeeds
// ------------------------------------------------------------------------------------------------

fn make_simple_shape(shape_name: &str, kind: Simple, doc_string: Option<&str>) -> TopLevelShape {
    let shape_id = ShapeID::new_unchecked("example.smithy", shape_name, None);
    let mut shape = TopLevelShape::new(shape_id, ShapeKind::Simple(kind));

    if let Some(doc_string) = doc_string {
        let trait_id = prelude_shape_named(TRAIT_DOCUMENTATION).unwrap();
        let _ = shape
            .apply_with_value(trait_id, Some(Value::String(doc_string.into())))
            .unwrap();
    }

    shape
}

enum CollectionShape {
    List,
    Set,
    Map,
}

fn make_collection_shape(
    shape_name: &str,
    kind: CollectionShape,
    target_shape: &str,
) -> TopLevelShape {
    let shape_id = ShapeID::new_unchecked("example.smithy", shape_name, None);
    let target_id = prelude_shape_named(target_shape).unwrap();
    let body = match kind {
        CollectionShape::List => ShapeKind::List(ListOrSet::new(target_id)),
        CollectionShape::Set => ShapeKind::Set(ListOrSet::new(target_id)),
        CollectionShape::Map => ShapeKind::Map(Map::new(target_id.clone(), target_id)),
    };
    TopLevelShape::new(shape_id, body)
}

enum StructureShape {
    Structure,
    Union,
}

fn make_structure_shape(
    shape_name: &str,
    kind: StructureShape,
    members: &[(&str, &str)],
) -> TopLevelShape {
    let shape_id = ShapeID::new_unchecked("example.smithy", shape_name, None);
    let mut inner = StructureOrUnion::new();
    for (name, target) in members {
        inner.add_member(
            Identifier::new_unchecked(name),
            prelude_shape_named(target).unwrap(),
        )
    }
    let body = match kind {
        StructureShape::Structure => ShapeKind::Structure(inner),
        StructureShape::Union => ShapeKind::Union(inner),
    };
    TopLevelShape::new(shape_id, body)
}

#[test]
fn merge_same_simple_shape() {
    let mut model = Model::new(Version::V10);

    assert!(model
        .add_shape(make_simple_shape("simpleString", Simple::String, None))
        .is_ok());

    assert!(model
        .add_shape(make_simple_shape("simpleString", Simple::String, None))
        .is_ok());
}

#[test]
fn merge_same_simple_shape_with_traits() {
    let mut model = Model::new(Version::V10);

    assert!(model
        .add_shape(make_simple_shape(
            "simpleString",
            Simple::String,
            Some("a simple string")
        ))
        .is_ok());

    assert!(model
        .add_shape(make_simple_shape(
            "simpleString",
            Simple::String,
            Some("a simple string")
        ))
        .is_ok());
}

#[test]
fn merge_same_simple_shape_with_traits_from_left() {
    let mut model = Model::new(Version::V10);

    assert!(model
        .add_shape(make_simple_shape(
            "simpleString",
            Simple::String,
            Some("a simple string")
        ))
        .is_ok());

    assert!(model
        .add_shape(make_simple_shape("simpleString", Simple::String, None))
        .is_ok());

    let shape_id = ShapeID::new_unchecked("example.smithy", "simpleString", None);
    let shape = model.shape(&shape_id).unwrap();
    assert_eq!(shape.traits().len(), 1);
}

#[test]
fn merge_same_simple_shape_with_traits_from_right() {
    let mut model = Model::new(Version::V10);

    assert!(model
        .add_shape(make_simple_shape("simpleString", Simple::String, None))
        .is_ok());

    assert!(model
        .add_shape(make_simple_shape(
            "simpleString",
            Simple::String,
            Some("a simple string")
        ))
        .is_ok());

    let shape_id = ShapeID::new_unchecked("example.smithy", "simpleString", None);
    let shape = model.shape(&shape_id).unwrap();
    assert_eq!(shape.traits().len(), 1);
}

#[test]
fn merge_same_list_shape() {
    let mut model = Model::new(Version::V10);

    assert!(model
        .add_shape(make_collection_shape(
            "MyCollection",
            CollectionShape::List,
            "String"
        ))
        .is_ok());

    assert!(model
        .add_shape(make_collection_shape(
            "MyCollection",
            CollectionShape::List,
            "String"
        ))
        .is_ok());
}

#[test]
fn merge_same_set_shape() {
    let mut model = Model::new(Version::V10);

    assert!(model
        .add_shape(make_collection_shape(
            "MyCollection",
            CollectionShape::Set,
            "String"
        ))
        .is_ok());

    assert!(model
        .add_shape(make_collection_shape(
            "MyCollection",
            CollectionShape::Set,
            "String"
        ))
        .is_ok());
}

#[test]
fn merge_same_map_shape() {
    let mut model = Model::new(Version::V10);

    assert!(model
        .add_shape(make_collection_shape(
            "MyCollection",
            CollectionShape::Map,
            "String"
        ))
        .is_ok());

    assert!(model
        .add_shape(make_collection_shape(
            "MyCollection",
            CollectionShape::Map,
            "String"
        ))
        .is_ok());
}

#[test]
fn merge_same_structure_shape() {
    let mut model = Model::new(Version::V10);

    assert!(model
        .add_shape(make_structure_shape(
            "MyCollection",
            StructureShape::Structure,
            &[("name", "String"), ("age", "Integer")],
        ))
        .is_ok());

    assert!(model
        .add_shape(make_structure_shape(
            "MyCollection",
            StructureShape::Structure,
            &[("name", "String"), ("age", "Integer")],
        ))
        .is_ok());
}

#[test]
fn merge_same_union_shape() {
    let mut model = Model::new(Version::V10);

    assert!(model
        .add_shape(make_structure_shape(
            "MyCollection",
            StructureShape::Union,
            &[("name", "String"), ("age", "Integer")],
        ))
        .is_ok());

    assert!(model
        .add_shape(make_structure_shape(
            "MyCollection",
            StructureShape::Union,
            &[("name", "String"), ("age", "Integer")],
        ))
        .is_ok());
}

// ------------------------------------------------------------------------------------------------
// Merge fails
// ------------------------------------------------------------------------------------------------

#[test]
fn merge_conflict_different_shapes() {
    let mut model = Model::new(Version::V10);

    assert!(model
        .add_shape(make_simple_shape("simpleString", Simple::String, None))
        .is_ok());

    assert!(model
        .add_shape(make_simple_shape("simpleString", Simple::Integer, None))
        .is_err());
}

#[test]
fn merge_conflict_shape_with_different_traits() {
    let mut model = Model::new(Version::V10);

    assert!(model
        .add_shape(make_simple_shape(
            "simpleString",
            Simple::String,
            Some("a simple string")
        ))
        .is_ok());

    assert!(model
        .add_shape(make_simple_shape(
            "simpleString",
            Simple::String,
            Some("a simple string, or number")
        ))
        .is_err());
}

#[test]
fn merge_conflict_list_shape_targets() {
    let mut model = Model::new(Version::V10);

    assert!(model
        .add_shape(make_collection_shape(
            "MyCollection",
            CollectionShape::List,
            "String"
        ))
        .is_ok());

    assert!(model
        .add_shape(make_collection_shape(
            "MyCollection",
            CollectionShape::List,
            "Integer"
        ))
        .is_err());
}

#[test]
fn merge_conflict_set_shape_targets() {
    let mut model = Model::new(Version::V10);

    assert!(model
        .add_shape(make_collection_shape(
            "MyCollection",
            CollectionShape::Set,
            "String"
        ))
        .is_ok());

    assert!(model
        .add_shape(make_collection_shape(
            "MyCollection",
            CollectionShape::Set,
            "Integer"
        ))
        .is_err());
}

#[test]
fn merge_conflict_map_shape_targets() {
    let mut model = Model::new(Version::V10);

    assert!(model
        .add_shape(make_collection_shape(
            "MyCollection",
            CollectionShape::Map,
            "String"
        ))
        .is_ok());

    assert!(model
        .add_shape(make_collection_shape(
            "MyCollection",
            CollectionShape::Map,
            "Integer"
        ))
        .is_err());
}

#[test]
fn merge_conflict_structure_shape() {
    let mut model = Model::new(Version::V10);

    assert!(model
        .add_shape(make_structure_shape(
            "MyCollection",
            StructureShape::Structure,
            &[("name", "String"), ("age", "Integer")],
        ))
        .is_ok());

    assert!(model
        .add_shape(make_structure_shape(
            "MyCollection",
            StructureShape::Structure,
            &[("name", "String"), ("age", "Decimal")],
        ))
        .is_err());
}

#[test]
fn merge_conflict_union_shape() {
    let mut model = Model::new(Version::V10);

    assert!(model
        .add_shape(make_structure_shape(
            "MyCollection",
            StructureShape::Union,
            &[("name", "String"), ("age", "Integer")],
        ))
        .is_ok());

    assert!(model
        .add_shape(make_structure_shape(
            "MyCollection",
            StructureShape::Union,
            &[("name", "String"), ("age", "Decimal")],
        ))
        .is_err());
}
