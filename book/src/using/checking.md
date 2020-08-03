# Checking Models

The following example is taken from the Smithy specification discussing
[relative name resolution](https://awslabs.github.io/smithy/1.0/spec/core/shapes.html#relative-shape-id-resolution).
The `run_validation_actions` function is commonly used to take a list of actions to be performed
on the model in sequence.

```rust
use atelier_core::action::validate::{
    run_validation_actions, CorrectTypeReferences
};
use atelier_core::action::Validator;
use atelier_core::builder::{
    ModelBuilder, ShapeTraits, SimpleShapeBuilder, StructureBuilder, TraitBuilder
};
use atelier_core::model::Model;
use atelier_core::Version;

let model: Model = ModelBuilder::new(Version::V10, "smithy.example")
    .uses("foo.baz#Bar")
    .structure(
        StructureBuilder::new("MyStructure")
            .member("a", "MyString")
            .member("b", "smithy.example#MyString")
            .member("d", "foo.baz#Bar")
            .member("e", "foo.baz#MyString")
            .member("f", "String")
            .member("g", "MyBoolean")
            .apply_trait(TraitBuilder::new("documentation"))
            .into(),
    )
    .simple_shape(SimpleShapeBuilder::string("MyString"))
    .simple_shape(SimpleShapeBuilder::boolean("MyBoolean"))
    .into();
let result = run_validation_actions(&mut [
        Box::new(CorrectTypeReferences::default()),
    ], &model, false);
```

This will result in the following list of validation errors. Note that the error is denoted against
shape or member identifier accordingly.

```text
[
    ActionIssue {
        reporter: "CorrectTypeReferences",
        level: Info,
        message: "The simple shape (smithy.example#MyBoolean) is simply a synonym, did you mean to add any constraint traits?",
        locus: Some(
            ShapeID {
                namespace: NamespaceID(
                    "smithy.example",
                ),
                shape_name: Identifier(
                    "MyBoolean",
                ),
                member_name: None,
            },
        ),
    },
    ActionIssue {
        reporter: "CorrectTypeReferences",
        level: Info,
        message: "The simple shape (smithy.example#MyString) is simply a synonym, did you mean to add any constraint traits?",
        locus: Some(
            ShapeID {
                namespace: NamespaceID(
                    "smithy.example",
                ),
                shape_name: Identifier(
                    "MyString",
                ),
                member_name: None,
            },
        ),
    },
    ActionIssue {
        reporter: "CorrectTypeReferences",
        level: Warning,
        message: "Structure member's type (foo.baz#MyString) cannot be resolved to a shape in this model.",
        locus: Some(
            ShapeID {
                namespace: NamespaceID(
                    "smithy.example",
                ),
                shape_name: Identifier(
                    "MyStructure",
                ),
                member_name: Some(
                    Identifier(
                        "e",
                    ),
                ),
            },
        ),
    },
]
```
