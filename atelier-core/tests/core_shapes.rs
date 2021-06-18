use atelier_core::{
    action::validate::{run_validation_actions, CorrectTypeReferences},
    builder::{ModelBuilder, OperationBuilder},
    model::{shapes::ShapeKind, Model, ShapeID},
    Version,
};
use pretty_assertions::assert_eq;
use std::convert::TryInto;
use std::str::FromStr;

#[test]
fn test_operation() {
    let model: Model = ModelBuilder::new(Version::V10, "smithy.example")
        .operation(
            OperationBuilder::new("MyOperation")
                .input("smithy.api#String")
                .output("smithy.api#Blob")
                .into(),
        )
        .try_into()
        .unwrap();

    let result = run_validation_actions(
        &mut [Box::new(CorrectTypeReferences::default())],
        &model,
        false,
    );
    assert!(result.is_ok());

    let top = model.shapes().find(|s| s.is_operation()).unwrap();
    if let ShapeKind::Operation(op) = top.body() {
        assert!(op.has_input());
        assert!(op.has_output());
        assert_eq!(
            op.input(),
            &Some(ShapeID::from_str("smithy.api#String").unwrap())
        );
        assert_eq!(
            op.output(),
            &Some(ShapeID::from_str("smithy.api#Blob").unwrap())
        );
        assert!(!op.has_errors());
    } else {
        assert!(false, "unexpected shape kind - expected operation")
    }
}
