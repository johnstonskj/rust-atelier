use atelier_core::builder::{
    ModelBuilder, ShapeTraits, SimpleShapeBuilder, TraitBuilder, ValueBuilder,
};
use atelier_core::model::shapes::HasTraits;
use atelier_core::model::values::{Value, ValueMap};
use atelier_core::model::{Model, ShapeID};
use atelier_core::prelude::{prelude_shape_named, TRAIT_SINCE, TRAIT_TAGS};
use atelier_core::Version;

// ------------------------------------------------------------------------------------------------
// Merge succeeds
// ------------------------------------------------------------------------------------------------

#[test]
fn merge_concat_arrays() {
    let mut model: Model = ModelBuilder::new(Version::V10, "smithy.example")
        .simple_shape(
            SimpleShapeBuilder::integer("counter")
                .tagged(&["tag-1", "tag-2"])
                .into(),
        )
        .into();
    let shape_id = ShapeID::new_unchecked("smithy.example", "counter", None);
    let shape = model.shape_mut(&shape_id).unwrap();
    let trait_id = prelude_shape_named(TRAIT_TAGS).unwrap();
    assert!(shape
        .apply_with_value(
            trait_id.clone(),
            Some(ValueBuilder::array().string("tag-3").string("tag-1").into())
        )
        .is_ok());
    let value = shape.trait_named(&trait_id);
    assert_eq!(
        value.unwrap(),
        &Some(Value::Array(vec![
            Value::String("tag-1".into()),
            Value::String("tag-2".into()),
            Value::String("tag-3".into()),
            Value::String("tag-1".into())
        ]))
    );
}

#[test]
fn merge_same_string() {
    let mut model: Model = ModelBuilder::new(Version::V10, "smithy.example")
        .simple_shape(
            SimpleShapeBuilder::integer("counter")
                .since("2021-04-30")
                .into(),
        )
        .into();
    let shape_id = ShapeID::new_unchecked("smithy.example", "counter", None);
    let shape = model.shape_mut(&shape_id).unwrap();
    let trait_id = prelude_shape_named(TRAIT_SINCE).unwrap();
    assert!(shape
        .apply_with_value(trait_id.clone(), Some(Value::String("2021-04-30".into())))
        .is_ok());
    let value = shape.trait_named(&trait_id);
    assert_eq!(value.unwrap(), &Some(Value::String("2021-04-30".into())));
}

#[test]
fn merge_same_number() {
    let mut model: Model = ModelBuilder::new(Version::V10, "smithy.example")
        .simple_shape(
            SimpleShapeBuilder::integer("counter")
                .apply_trait(
                    TraitBuilder::new("smithy.example#revision")
                        .integer(1)
                        .clone(),
                )
                .into(),
        )
        .into();
    let shape_id = ShapeID::new_unchecked("smithy.example", "counter", None);
    let shape = model.shape_mut(&shape_id).unwrap();
    let trait_id = ShapeID::new_unchecked("smithy.example", "revision", None);
    assert!(shape
        .apply_with_value(trait_id.clone(), Some(Value::Number(1.into())))
        .is_ok());
    let value = shape.trait_named(&trait_id);
    assert_eq!(value.unwrap(), &Some(Value::Number(1.into())));
}

#[test]
fn merge_same_boolean() {
    let mut model: Model = ModelBuilder::new(Version::V10, "smithy.example")
        .simple_shape(
            SimpleShapeBuilder::integer("counter")
                .apply_trait(
                    TraitBuilder::new("smithy.example#revised")
                        .boolean(true)
                        .clone(),
                )
                .into(),
        )
        .into();
    let shape_id = ShapeID::new_unchecked("smithy.example", "counter", None);
    let shape = model.shape_mut(&shape_id).unwrap();
    let trait_id = ShapeID::new_unchecked("smithy.example", "revised", None);
    assert!(shape
        .apply_with_value(trait_id.clone(), Some(Value::Boolean(true)))
        .is_ok());
    let value = shape.trait_named(&trait_id);
    assert_eq!(value.unwrap(), &Some(Value::Boolean(true)));
}

#[test]
fn merge_same_object() {
    let mut object = ValueMap::new();
    object.insert("bool".into(), Value::Boolean(true));
    object.insert("name".into(), Value::String("example-model".into()));

    let mut model: Model = ModelBuilder::new(Version::V10, "smithy.example")
        .simple_shape(
            SimpleShapeBuilder::integer("counter")
                .apply_trait(
                    TraitBuilder::new("smithy.example#someTrait")
                        .object(object.clone())
                        .clone(),
                )
                .into(),
        )
        .into();
    let shape_id = ShapeID::new_unchecked("smithy.example", "counter", None);
    let shape = model.shape_mut(&shape_id).unwrap();
    let trait_id = ShapeID::new_unchecked("smithy.example", "someTrait", None);
    assert!(shape
        .apply_with_value(trait_id.clone(), Some(Value::Object(object.clone())))
        .is_ok());
    let value = shape.trait_named(&trait_id);
    assert_eq!(value.unwrap(), &Some(Value::Object(object)));
}

// ------------------------------------------------------------------------------------------------
// Merge fails
// ------------------------------------------------------------------------------------------------

#[test]
fn merge_conflict_types() {
    let mut model: Model = ModelBuilder::new(Version::V10, "smithy.example")
        .simple_shape(
            SimpleShapeBuilder::integer("counter")
                .since("2021-04-30")
                .into(),
        )
        .into();
    let shape_id = ShapeID::new_unchecked("smithy.example", "counter", None);
    let shape = model.shape_mut(&shape_id).unwrap();
    let trait_id = prelude_shape_named(TRAIT_SINCE).unwrap();
    assert!(shape
        .apply_with_value(trait_id.clone(), Some(Value::Boolean(true)))
        .is_err());
    let value = shape.trait_named(&trait_id);
    assert_eq!(value.unwrap(), &Some(Value::String("2021-04-30".into())));
}

#[test]
fn merge_conflict_string() {
    let mut model: Model = ModelBuilder::new(Version::V10, "smithy.example")
        .simple_shape(
            SimpleShapeBuilder::integer("counter")
                .since("2021-04-30")
                .into(),
        )
        .into();
    let shape_id = ShapeID::new_unchecked("smithy.example", "counter", None);
    let shape = model.shape_mut(&shape_id).unwrap();
    let trait_id = prelude_shape_named(TRAIT_SINCE).unwrap();
    assert!(shape
        .apply_with_value(trait_id.clone(), Some(Value::String("not-a-date".into())))
        .is_err());
    let value = shape.trait_named(&trait_id);
    assert_eq!(value.unwrap(), &Some(Value::String("2021-04-30".into())));
}

#[test]
fn merge_conflict_number() {
    let mut model: Model = ModelBuilder::new(Version::V10, "smithy.example")
        .simple_shape(
            SimpleShapeBuilder::integer("counter")
                .apply_trait(
                    TraitBuilder::new("smithy.example#revision")
                        .integer(1)
                        .clone(),
                )
                .into(),
        )
        .into();
    let shape_id = ShapeID::new_unchecked("smithy.example", "counter", None);
    let shape = model.shape_mut(&shape_id).unwrap();
    let trait_id = ShapeID::new_unchecked("smithy.example", "revision", None);
    assert!(shape
        .apply_with_value(trait_id.clone(), Some(Value::Number(22.into())))
        .is_err());
    let value = shape.trait_named(&trait_id);
    assert_eq!(value.unwrap(), &Some(Value::Number(1.into())));
}

#[test]
fn merge_conflict_boolean() {
    let mut model: Model = ModelBuilder::new(Version::V10, "smithy.example")
        .simple_shape(
            SimpleShapeBuilder::integer("counter")
                .apply_trait(
                    TraitBuilder::new("smithy.example#revised")
                        .boolean(true)
                        .clone(),
                )
                .into(),
        )
        .into();
    let shape_id = ShapeID::new_unchecked("smithy.example", "counter", None);
    let shape = model.shape_mut(&shape_id).unwrap();
    let trait_id = ShapeID::new_unchecked("smithy.example", "revised", None);
    assert!(shape
        .apply_with_value(trait_id.clone(), Some(Value::Boolean(false)))
        .is_err());
    let value = shape.trait_named(&trait_id);
    assert_eq!(value.unwrap(), &Some(Value::Boolean(true)));
}

#[test]
fn merge_conflict_object() {
    let mut object = ValueMap::new();
    object.insert("bool".into(), Value::Boolean(true));
    object.insert("name".into(), Value::String("example-model".into()));

    let mut model: Model = ModelBuilder::new(Version::V10, "smithy.example")
        .simple_shape(
            SimpleShapeBuilder::integer("counter")
                .apply_trait(
                    TraitBuilder::new("smithy.example#someTrait")
                        .object(object.clone())
                        .clone(),
                )
                .into(),
        )
        .into();
    let shape_id = ShapeID::new_unchecked("smithy.example", "counter", None);
    let shape = model.shape_mut(&shape_id).unwrap();
    let trait_id = ShapeID::new_unchecked("smithy.example", "someTrait", None);
    let mut new_object = ValueMap::new();
    new_object.insert("bool".into(), Value::Boolean(false));
    new_object.insert("name".into(), Value::String("example-model".into()));
    assert!(shape
        .apply_with_value(trait_id.clone(), Some(Value::Object(new_object)))
        .is_err());
    let value = shape.trait_named(&trait_id);
    assert_eq!(value.unwrap(), &Some(Value::Object(object)));
}
