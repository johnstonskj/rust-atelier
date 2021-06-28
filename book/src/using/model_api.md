# Using the Model API

The following example demonstrates the core model API to create a model for a simple service. The
service, `MessageOfTheDay` has a single resource `Message`. The resource has an identifier for the
date, but the `read` operation does not make the date member required and so will return the message
for the current date.

This API acts as a set of generic data objects and as such has a tendency to be verbose in the
construction of models. The need to create a lot of `Identifier` and `ShapeID` instances, for example,
does impact the readability. It is important to note, that while the Smithy
[specification](https://awslabs.github.io/smithy/1.0/spec/core/model.html#shape-id) describes
both _absolute_ and _relative_ shape identifiers, relative  identifiers **are not** supported in the semantic 
model. All names in the semantic model **must** be resolved to an absolute name.

```rust
use atelier_core::model::identity::{HasIdentity, Identifier};
use atelier_core::model::shapes::{
    HasTraits, MemberShape, Operation, Resource, Service, Shape,
    ShapeKind, Simple, StructureOrUnion, TopLevelShape,
};
use atelier_core::model::values::Value;
use atelier_core::model::{Model, NamespaceID};
use atelier_core::prelude::PRELUDE_NAMESPACE;
use atelier_core::Version;

let prelude: NamespaceID = PRELUDE_NAMESPACE.parse().unwrap();
let namespace: NamespaceID = "example.motd".parse().unwrap();

// ----------------------------------------------------------------------------------------
let mut date = TopLevelShape::new(
    namespace.make_shape("Date".parse().unwrap()),
    ShapeKind::Simple(Simple::String),
);
date
    .apply_with_value(
        prelude.make_shape("pattern".parse().unwrap()),
        Value::String(r"^\d\d\d\d\-\d\d-\d\d$".to_string()).into()
    )
    .unwrap();

// ----------------------------------------------------------------------------------------
let shape_name = namespace.make_shape("BadDateValue".parse().unwrap());
let mut body = StructureOrUnion::new();
body.add_member(
    "errorMessage".parse().unwrap(),
    prelude.make_shape("String".parse().unwrap()),
);
let mut error = TopLevelShape::new(shape_name, ShapeKind::Structure(body));
error
    .apply_with_value(
        prelude.make_shape("error".parse().unwrap()),
        Some("client".to_string().into()),
    )
    .unwrap();

// ----------------------------------------------------------------------------------------
let shape_name = namespace.make_shape("GetMessageOutput".parse().unwrap());
let mut output = StructureOrUnion::new();
let mut message = MemberShape::new(
    "message".parse().unwrap(),
    prelude.make_shape("String".parse().unwrap()),
);
message
    .apply(prelude.make_shape("required".parse().unwrap()))
    .unwrap();
let _ = output.add_a_member(message);
let output = TopLevelShape::new(
    namespace.make_shape("GetMessageOutput".parse().unwrap()),
    ShapeKind::Structure(output),
);

// ----------------------------------------------------------------------------------------
let shape_name = namespace.make_shape("GetMessageInput".parse().unwrap());
let mut input = StructureOrUnion::new();
input.add_member(
    "date".parse().unwrap(),
    date.id().clone(),
);
let input = TopLevelShape::new(
    namespace.make_shape("GetMessageInput".parse().unwrap()),
    ShapeKind::Structure(input),
);

// ----------------------------------------------------------------------------------------
let mut get_message = Operation::default();
get_message.set_input_shape(&input);
get_message.set_output_shape(&output);
get_message.add_error_shape(&error);
let mut get_message = TopLevelShape::new(
    namespace.make_shape("GetMessage".parse().unwrap()),
    ShapeKind::Operation(get_message),
);
get_message
    .apply(prelude.make_shape("readonly".parse().unwrap()))
    .unwrap();

// ----------------------------------------------------------------------------------------
let mut message = Resource::default();
message.add_identifier(Identifier::new_unchecked("date"), date.id().clone());
message.set_read_operation_shape(&get_message);
let message = TopLevelShape::new(
    namespace.make_shape("Message".parse().unwrap()),
    ShapeKind::Resource(message),
);

// ----------------------------------------------------------------------------------------
let mut service = Service::new("2020-06-21");
service.add_resource_shape(&message);
let mut service = TopLevelShape::new(
    namespace.make_shape("MessageOfTheDay".parse().unwrap()),
    ShapeKind::Service(service),
);
service
    .apply_with_value(
        prelude.make_shape("documentation".parse().unwrap()),
        Value::String("Provides a Message of the day.".to_string()).into(),
    )
    .unwrap();

// ----------------------------------------------------------------------------------------
let mut model = Model::new(Version::V10);
model.add_shape(message);
model.add_shape(date);
model.add_shape(get_message);
model.add_shape(input);
model.add_shape(output);
model.add_shape(error);

println!("{:#?}", model);
```
