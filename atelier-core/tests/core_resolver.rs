/*!

From [Relative shape ID resolution](https://awslabs.github.io/smithy/1.0/spec/core/idl.html#relative-shape-id-resolution)

```smithy
namespace smithy.example

use foo.baz#Bar

string MyString

structure MyStructure {
    // Resolves to smithy.example#MyString
    // There is a shape named MyString defined in the same namespace.
    a: MyString,

    // Resolves to smithy.example#MyString
    // Absolute shape IDs do not perform namespace resolution.
    b: smithy.example#MyString,

    // Resolves to foo.baz#Bar
    // The "use foo.baz#Bar" statement imported the Bar symbol,
    // allowing the shape to be referenced using a relative shape ID.
    c: Bar,

    // Resolves to smithy.api#String
    // No shape named String was imported through a use statement
    // the smithy.example namespace does not contain a shape named
    // String, and the prelude model contains a shape named String.
    d: String,

    // Resolves to smithy.example#MyBoolean.
    // There is a shape named MyBoolean defined in the same namespace.
    // Forward references are supported both within the same file and
    // across multiple files.
    e: MyBoolean,

    // Resolves to smithy.example#InvalidShape. A shape by this name has
    // not been imported through a use statement, a shape by this name
    // does not exist in the current namespace, and a shape by this name
    // does not exist in the prelude model.
    f: InvalidShape,
}

boolean MyBoolean
```
 */

use atelier_core::builder::{ModelBuilder, SimpleShapeBuilder, StructureBuilder};
use atelier_core::error::{Error, ErrorKind};
use atelier_core::model::shapes::ShapeKind;
use atelier_core::model::{HasIdentity, Model, ShapeID};
use atelier_core::Version;
use std::convert::TryInto;

#[test]
fn test_shapeid_resolution_valid() {
    let model: Model = ModelBuilder::new(Version::V10, "smithy.example")
        .uses("foo.baz#Bar")
        .simple_shape(SimpleShapeBuilder::string("MyString"))
        .structure(
            StructureBuilder::new("MyStructure")
                .member("a", "MyString")
                .member("b", "smithy.example#MyString")
                .member("c", "Bar")
                .member("d", "String")
                .member("e", "MyBoolean")
                .into(),
        )
        .simple_shape(SimpleShapeBuilder::boolean("MyBoolean"))
        .try_into()
        .unwrap();

    assert!(!model.is_complete());

    let mut top_level_shape_names: Vec<String> =
        model.shape_names().map(|s| s.to_string()).collect();
    top_level_shape_names.sort();
    assert_eq!(
        top_level_shape_names,
        vec![
            "smithy.example#MyBoolean".to_string(),
            "smithy.example#MyString".to_string(),
            "smithy.example#MyStructure".to_string()
        ]
    );

    let my_structure = model
        .shape(&ShapeID::new_unchecked(
            "smithy.example",
            "MyStructure",
            None,
        ))
        .unwrap();

    if let ShapeKind::Structure(my_structure) = my_structure.body() {
        println!("{:#?}", my_structure);
        let expected = [
            (
                ShapeID::new_unchecked("smithy.example", "MyStructure", Some("a")),
                ShapeID::new_unchecked("smithy.example", "MyString", None),
            ),
            (
                ShapeID::new_unchecked("smithy.example", "MyStructure", Some("b")),
                ShapeID::new_unchecked("smithy.example", "MyString", None),
            ),
            (
                ShapeID::new_unchecked("smithy.example", "MyStructure", Some("c")),
                ShapeID::new_unchecked("foo.baz", "Bar", None),
            ),
            (
                ShapeID::new_unchecked("smithy.example", "MyStructure", Some("d")),
                ShapeID::new_unchecked("smithy.api", "String", None),
            ),
            (
                ShapeID::new_unchecked("smithy.example", "MyStructure", Some("e")),
                ShapeID::new_unchecked("smithy.example", "MyBoolean", None),
            ),
        ];

        for (member_id, target) in expected.iter() {
            let member = my_structure
                .member(member_id.member_name().as_ref().unwrap())
                .unwrap();
            assert_eq!(member.id(), member_id);
            assert_eq!(member.target(), target);
        }
    } else {
        panic!("my_structure is not a structure")
    }
}

#[test]
fn test_shapeid_resolution_invalid() {
    let result: Result<Model, Error> = ModelBuilder::new(Version::V10, "smithy.example")
        .uses("foo.baz#Bar")
        .simple_shape(SimpleShapeBuilder::string("MyString"))
        .structure(
            StructureBuilder::new("MyStructure")
                .member("a", "MyString")
                .member("b", "smithy.example#MyString")
                .member("c", "Bar")
                .member("d", "String")
                .member("e", "MyBoolean")
                .member("f", "InvalidShape")
                .into(),
        )
        .simple_shape(SimpleShapeBuilder::boolean("MyBoolean"))
        .try_into();
    let expected: Result<Model, Error> =
        Err(ErrorKind::UnknownShape("InvalidShape".to_string()).into());
    assert_eq!(
        result.err().map(|e| e.to_string()),
        expected.err().map(|e| e.to_string())
    )
}
