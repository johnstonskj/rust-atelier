use atelier_core::model::shapes::{
    HasTraits, MemberShape, Operation, Resource, Service, ShapeKind, Simple, StructureOrUnion,
    TopLevelShape,
};
use atelier_core::model::values::Value;
use atelier_core::model::{HasIdentity, Identifier, Model, NamespaceID};
use atelier_core::prelude::PRELUDE_NAMESPACE;
use atelier_core::Version;

pub fn main() {
    let prelude: NamespaceID = PRELUDE_NAMESPACE.parse().unwrap();
    let namespace: NamespaceID = "example.motd".parse().unwrap();

    // ----------------------------------------------------------------------------------------
    let mut date = TopLevelShape::new(
        namespace.make_shape("Date".parse().unwrap()),
        ShapeKind::Simple(Simple::String),
    );
    date.apply_with_value(
        prelude.make_shape("pattern".parse().unwrap()),
        Value::String(r"^\d\d\d\d\-\d\d-\d\d$".to_string()).into(),
    )
    .unwrap();

    // ----------------------------------------------------------------------------------------
    let shape_name = namespace.make_shape("BadDateValue".parse().unwrap());
    let mut body = StructureOrUnion::new();
    body.add_member(
        shape_name.make_member("errorMessage".parse().unwrap()),
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
        shape_name.make_member("message".parse().unwrap()),
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
        shape_name.make_member("date".parse().unwrap()),
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
    let _ = model.add_shape(message);
    let _ = model.add_shape(date);
    let _ = model.add_shape(get_message);
    let _ = model.add_shape(input);
    let _ = model.add_shape(output);
    let _ = model.add_shape(error);

    println!("{:#?}", model);
}
