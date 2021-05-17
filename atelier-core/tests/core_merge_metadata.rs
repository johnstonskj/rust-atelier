use atelier_core::builder::ModelBuilder;
use atelier_core::model::values::{Value, ValueMap};
use atelier_core::model::Model;
use atelier_core::Version;
use std::convert::TryInto;

// ------------------------------------------------------------------------------------------------
// Merge succeeds
// ------------------------------------------------------------------------------------------------

#[test]
fn merge_concat_arrays() {
    let mut model: Model = ModelBuilder::new(Version::V10, "smithy.example")
        .meta_data(
            "name".into(),
            Value::Array(vec![Value::Number(101.into()), Value::Number(102.into())]),
        )
        .try_into()
        .unwrap();
    assert!(model
        .add_metadata(
            "name".into(),
            Value::Array(vec![Value::Number(201.into()), Value::Number(202.into())])
        )
        .is_ok());
    let value = model.metadata_value("name");
    assert_eq!(
        value.unwrap(),
        &Value::Array(vec![
            Value::Number(101.into()),
            Value::Number(102.into()),
            Value::Number(201.into()),
            Value::Number(202.into())
        ])
    );
}

#[test]
fn merge_same_string() {
    let mut model: Model = ModelBuilder::new(Version::V10, "smithy.example")
        .meta_data("name".into(), Value::String("example-model".into()))
        .try_into()
        .unwrap();
    assert!(model
        .add_metadata("name".into(), Value::String("example-model".into()))
        .is_ok());
    let value = model.metadata_value("name");
    assert_eq!(value.unwrap(), &Value::String("example-model".into()));
}

#[test]
fn merge_same_number() {
    let mut model: Model = ModelBuilder::new(Version::V10, "smithy.example")
        .meta_data("name".into(), Value::Number(101.into()))
        .try_into()
        .unwrap();
    assert!(model
        .add_metadata("name".into(), Value::Number(101.into()))
        .is_ok());
    let value = model.metadata_value("name");
    assert_eq!(value.unwrap(), &Value::Number(101.into()));
}

#[test]
fn merge_same_boolean() {
    let mut model: Model = ModelBuilder::new(Version::V10, "smithy.example")
        .meta_data("name".into(), Value::Boolean(true))
        .try_into()
        .unwrap();
    assert!(model
        .add_metadata("name".into(), Value::Boolean(true))
        .is_ok());
    let value = model.metadata_value("name");
    assert_eq!(value.unwrap(), &Value::Boolean(true));
}

#[test]
fn merge_same_object() {
    let mut object = ValueMap::new();
    object.insert("bool".into(), Value::Boolean(true));
    object.insert("name".into(), Value::String("example-model".into()));
    let mut model: Model = ModelBuilder::new(Version::V10, "smithy.example")
        .meta_data("name".into(), Value::Object(object.clone()))
        .try_into()
        .unwrap();
    assert!(model
        .add_metadata("name".into(), Value::Object(object.clone()))
        .is_ok());
    let value = model.metadata_value("name");
    assert_eq!(value.unwrap(), &Value::Object(object.clone()));
}

// ------------------------------------------------------------------------------------------------
// Merge fails
// ------------------------------------------------------------------------------------------------

#[test]
fn merge_conflict_types() {
    let mut model: Model = ModelBuilder::new(Version::V10, "smithy.example")
        .meta_data("name".into(), Value::String("example-model".into()))
        .try_into()
        .unwrap();
    assert!(model
        .add_metadata("name".into(), Value::Boolean(false))
        .is_err());
}

#[test]
fn merge_conflict_string() {
    let mut model: Model = ModelBuilder::new(Version::V10, "smithy.example")
        .meta_data("name".into(), Value::String("example-model".into()))
        .try_into()
        .unwrap();
    assert!(model
        .add_metadata("name".into(), Value::String("another-name".into()))
        .is_err());
    let value = model.metadata_value("name");
    assert_eq!(value.unwrap(), &Value::String("example-model".into()));
}

#[test]
fn merge_conflict_number() {
    let mut model: Model = ModelBuilder::new(Version::V10, "smithy.example")
        .meta_data("name".into(), Value::Number(101.into()))
        .try_into()
        .unwrap();
    assert!(model
        .add_metadata("name".into(), Value::Number(202.into()))
        .is_err());
    let value = model.metadata_value("name");
    assert_eq!(value.unwrap(), &Value::Number(101.into()));
}

#[test]
fn merge_conflict_boolean() {
    let mut model: Model = ModelBuilder::new(Version::V10, "smithy.example")
        .meta_data("name".into(), Value::Boolean(true))
        .try_into()
        .unwrap();
    assert!(model
        .add_metadata("name".into(), Value::Boolean(false))
        .is_err());
    let value = model.metadata_value("name");
    assert_eq!(value.unwrap(), &Value::Boolean(true));
}

#[test]
fn merge_conflict_object() {
    let mut object = ValueMap::new();
    object.insert("bool".into(), Value::Boolean(true));
    object.insert("name".into(), Value::String("example-model".into()));
    let mut model: Model = ModelBuilder::new(Version::V10, "smithy.example")
        .meta_data("name".into(), Value::Object(object))
        .try_into()
        .unwrap();

    let mut object = ValueMap::new();
    object.insert("bool".into(), Value::Boolean(false));
    object.insert("name".into(), Value::String("example-model".into()));
    assert!(model
        .add_metadata("name".into(), Value::Object(object))
        .is_err());
}
