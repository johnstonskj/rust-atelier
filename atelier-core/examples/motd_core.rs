use atelier_core::model::shapes::{
    AppliedTrait, MemberShape, Operation, Resource, Service, Shape, ShapeKind, Simple,
    StructureOrUnion, TopLevelShape,
};
use atelier_core::model::values::Value;
use atelier_core::model::{Model, NamespaceID};
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
    let mut pattern_trait = AppliedTrait::new(prelude.make_shape("pattern".parse().unwrap()));
    pattern_trait.set_value(Value::String(r"^\d\d\d\d\-\d\d-\d\d$".to_string()));
    date.apply_trait(pattern_trait);

    // ----------------------------------------------------------------------------------------
    let shape_name = namespace.make_shape("BadDateValue".parse().unwrap());
    let mut body = StructureOrUnion::new();
    body.add_member(
        shape_name.make_member("errorMessage".parse().unwrap()),
        prelude.make_shape("String".parse().unwrap()),
    );
    let mut error = TopLevelShape::new(shape_name, ShapeKind::Structure(body));
    let error_trait = AppliedTrait::with_value(
        prelude.make_shape("error".parse().unwrap()),
        "client".to_string().into(),
    );
    error.apply_trait(error_trait);

    // ----------------------------------------------------------------------------------------
    let shape_name = namespace.make_shape("GetMessageOutput".parse().unwrap());
    let mut output = StructureOrUnion::new();
    let mut message = MemberShape::new(
        shape_name.make_member("message".parse().unwrap()),
        prelude.make_shape("String".parse().unwrap()),
    );
    let required = AppliedTrait::new(prelude.make_shape("required".parse().unwrap()));
    message.apply_trait(required);
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
    let required = AppliedTrait::new(prelude.make_shape("readonly".parse().unwrap()));
    get_message.apply_trait(required);

    // ----------------------------------------------------------------------------------------
    let mut message = Resource::default();
    message.add_identifier("date".to_string(), Value::String(date.id().to_string()));
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
    let documentation = AppliedTrait::with_value(
        prelude.make_shape("documentation".parse().unwrap()),
        Value::String("Provides a Message of the day.".to_string()),
    );
    service.apply_trait(documentation);

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
