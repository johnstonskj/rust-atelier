use atelier_core::model::shapes::Simple;
use atelier_core::model::shapes::{HasTraits, ShapeKind, TopLevelShape};
use atelier_core::model::{Model, ShapeID};
use atelier_core::prelude::{prelude_shape_named, TRAIT_BOX, TRAIT_PRIVATE, TRAIT_SENSITIVE};
use atelier_core::Version;

fn make_real_shape(boxed: bool) -> TopLevelShape {
    let mut shape = TopLevelShape::new(
        ShapeID::new_unchecked("example", "Identifier", None),
        ShapeKind::Simple(Simple::String),
    );
    let _ = shape.apply(prelude_shape_named(TRAIT_SENSITIVE).unwrap());
    if boxed {
        let _ = shape.apply(prelude_shape_named(TRAIT_BOX).unwrap());
    }
    shape
}

fn make_unresolved_shape(boxed: bool) -> TopLevelShape {
    let mut shape = TopLevelShape::new(
        ShapeID::new_unchecked("example", "Identifier", None),
        ShapeKind::Unresolved,
    );
    let _ = shape.apply(prelude_shape_named(TRAIT_PRIVATE).unwrap());
    if boxed {
        let _ = shape.apply(prelude_shape_named(TRAIT_BOX).unwrap());
    }
    shape
}

// ------------------------------------------------------------------------------------------------
// Merge succeeds
// ------------------------------------------------------------------------------------------------

#[test]
fn merge_unresolved_same_left_right() {
    let mut model = Model::new(Version::V10);

    assert!(model.add_shape(make_unresolved_shape(false)).is_ok());

    assert!(model.add_shape(make_unresolved_shape(false)).is_ok());

    let shape = model
        .shape(&ShapeID::new_unchecked("example", "Identifier", None))
        .unwrap();

    assert_eq!(shape.traits().len(), 1);

    assert!(shape.body().is_unresolved())
}

#[test]
fn merge_unresolved_diff_left_right() {
    let mut model = Model::new(Version::V10);

    assert!(model.add_shape(make_unresolved_shape(false)).is_ok());

    assert!(model.add_shape(make_unresolved_shape(true)).is_ok());

    let shape = model
        .shape(&ShapeID::new_unchecked("example", "Identifier", None))
        .unwrap();

    assert_eq!(shape.traits().len(), 2);

    assert!(shape.body().is_unresolved())
}

#[test]
fn merge_unresolved_same_left() {
    let mut model = Model::new(Version::V10);

    assert!(model.add_shape(make_unresolved_shape(false)).is_ok());

    assert!(model.add_shape(make_real_shape(false)).is_ok());

    let shape = model
        .shape(&ShapeID::new_unchecked("example", "Identifier", None))
        .unwrap();

    assert_eq!(shape.traits().len(), 2);

    assert!(shape.body().is_simple())
}

#[test]
fn merge_unresolved_diff_left() {
    let mut model = Model::new(Version::V10);

    assert!(model.add_shape(make_unresolved_shape(false)).is_ok());

    assert!(model.add_shape(make_real_shape(true)).is_ok());

    let shape = model
        .shape(&ShapeID::new_unchecked("example", "Identifier", None))
        .unwrap();

    assert_eq!(shape.traits().len(), 3);

    assert!(shape.body().is_simple())
}

#[test]
fn merge_unresolved_same_right() {
    let mut model = Model::new(Version::V10);

    assert!(model.add_shape(make_real_shape(false)).is_ok());

    assert!(model.add_shape(make_unresolved_shape(false)).is_ok());

    let shape = model
        .shape(&ShapeID::new_unchecked("example", "Identifier", None))
        .unwrap();

    assert_eq!(shape.traits().len(), 2);

    assert!(shape.body().is_simple())
}

#[test]
fn merge_unresolved_diff_right() {
    let mut model = Model::new(Version::V10);

    assert!(model.add_shape(make_real_shape(false)).is_ok());

    assert!(model.add_shape(make_unresolved_shape(true)).is_ok());

    let shape = model
        .shape(&ShapeID::new_unchecked("example", "Identifier", None))
        .unwrap();

    assert_eq!(shape.traits().len(), 3);

    assert!(shape.body().is_simple())
}
