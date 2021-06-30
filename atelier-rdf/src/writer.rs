/*!
Provides an [`model_to_rdf`](fn.model_to_rdf.html) function and [`RdfWriter`](struct.RdfWriter.html)
type that will write a model into RDF.

# Example - model_to_rdf

The following simply constructs an in-memory RDF Graph from a provided model.

```rust
use atelier_core::model::Model;
use atelier_rdf::writer::model_to_rdf;
use rdftk_core::simple::graph_factory;
# use atelier_core::Version;
# fn make_model() -> Model { Model::new(Version::default()) }

let model = make_model();
let rdf_graph = model_to_rdf(&model, None, graph_factory()).unwrap();
```

# Example - RdfWriter

This implementation of ModelWriter will output the provided model in it's RDF form, using the
Turtle serialization format. If you wish to use other serialization formats it is best to call
`model_to_rdf` and use one of the graph writer implementation in the
[rdfktk_io](https://github.com/johnstonskj/rust-rdftk/tree/master/rdftk_io) crate.

```rust
use atelier_core::model::Model;
use atelier_core::io::ModelWriter;
use atelier_rdf::writer::RdfWriter;
use std::io::stdout;
# use atelier_core::Version;
# fn make_model() -> Model { Model::new(Version::default()) }

let model = make_model();
let mut writer = RdfWriter::default();
writer.write(&mut stdout(), &model).unwrap();
```

The example above uses the `default` constructor, this will assign a blank node as the identity
of the model. Alternatively, you may pass in an IRI which will be used as the identity instead.

```rust
use atelier_rdf::writer::RdfWriter;
use rdftk_iri::{IRI, IRIRef};
use std::str::FromStr;

let mut writer = RdfWriter::new(
    IRIRef::from(IRI::from_str("https://example.org/example/smithy").unwrap())
);
```
*/

use crate::urn::shape_to_iri;
use crate::vocabulary;
use atelier_core::error::{Error as ModelError, ErrorKind, Result as ModelResult};
use atelier_core::io::ModelWriter;
use atelier_core::model::shapes::{
    AppliedTraits, HasTraits, ListOrSet, Map, MemberShape, Operation, Resource, Service, Simple,
    StructureOrUnion,
};
use atelier_core::model::values::{Number, Value};
use atelier_core::model::visitor::{walk_model, ModelVisitor};
use atelier_core::model::{HasIdentity, Model, ShapeID};
use rdftk_core::model::graph::{Graph, GraphFactoryRef, GraphRef};
use rdftk_core::model::statement::SubjectNodeRef;
use rdftk_core::simple::graph_factory;
use rdftk_io::turtle::writer::TurtleWriter;
use rdftk_io::turtle::NAME;
use rdftk_io::GraphWriter;
use rdftk_iri::{IRIRef, IRI};
use rdftk_names::rdf;
use std::cell::RefMut;
use std::io::Write;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Simple implementation of the `ModelWriter` trait that writes the RDF representation of a model.
///
/// Currently the RDF writer takes only one parameter which will be used as the subject IRI for the
/// model. If not specified the writer will use a generated blank node instead.
///
///```rust
/// use atelier_rdf::RdfWriter;
/// use atelier_rdf::urn::shape_to_iri;
/// use atelier_core::model::ShapeID;
/// use std::str::FromStr;
///
/// let writer = RdfWriter::default();
///
/// let shape_id = ShapeID::from_str("org.example#ExampleShape").unwrap();
/// let writer = RdfWriter::new(shape_to_iri(&shape_id));
/// ```
///
#[derive(Debug)]
pub struct RdfWriter {
    model_iri: Option<IRIRef>,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

struct RdfModelVisitor {
    model_subject: SubjectNodeRef,
    graph: GraphRef,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Convert a Smithy semantic model into a canonical RDF graph representation. This function is
/// used by the `RdfWriter` implementation.
///
pub fn model_to_rdf(
    model: &Model,
    model_iri: Option<IRIRef>,
    factory: GraphFactoryRef,
) -> ModelResult<GraphRef> {
    let graph = factory.graph();
    let model_subject = match model_iri {
        None => graph.borrow().statement_factory().blank_subject(),
        Some(iri) => graph.borrow().statement_factory().named_subject(iri),
    };
    let mappings = factory.mapping_factory().common();
    {
        let mut mappings = mappings.borrow_mut();
        let _ = mappings.insert(
            vocabulary::default_prefix(),
            vocabulary::namespace_iri().clone(),
        );
        let _ = mappings.insert(
            "api",
            IRIRef::from(IRI::from_str("urn:smithy:smithy.api:").unwrap()),
        );
    }
    {
        let mut graph = graph.borrow_mut();
        let _ = graph.set_prefix_mappings(mappings);
        let statement_factory = graph.statement_factory();
        let literal_factory = graph.literal_factory();

        graph.insert(
            statement_factory
                .statement(
                    model_subject.clone(),
                    vocabulary::smithy_version().clone(),
                    statement_factory.literal_object(
                        literal_factory.literal(&model.smithy_version().to_string()),
                    ),
                )
                .unwrap(),
        );

        graph.insert(
            statement_factory
                .statement(
                    model_subject.clone(),
                    rdf::a_type().clone(),
                    statement_factory.named_object(vocabulary::model().clone()),
                )
                .unwrap(),
        );
    }
    let visitor = RdfModelVisitor {
        model_subject,
        graph,
    };
    walk_model(model, &visitor)?;

    Ok(visitor.graph)
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for RdfWriter {
    fn default() -> Self {
        Self { model_iri: None }
    }
}

impl ModelWriter for RdfWriter {
    fn write(&mut self, w: &mut impl Write, model: &Model) -> atelier_core::error::Result<()> {
        let graph_factory = graph_factory();
        let rdf_graph = model_to_rdf(model, self.model_iri.clone(), graph_factory)?;

        let writer = TurtleWriter::default();

        let result = writer.write(w, &rdf_graph);

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(ErrorKind::Serialization(NAME.to_string()).into()),
        }
    }
}

impl RdfWriter {
    ///
    /// Construct a new writer with the provided IRI identifying the model.
    ///
    pub fn new(model_iri: IRIRef) -> Self {
        Self {
            model_iri: Some(model_iri),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl ModelVisitor for RdfModelVisitor {
    type Error = ModelError;

    fn metadata(&self, _key: &str, _value: &Value) -> Result<(), Self::Error> {
        Ok(())
    }

    fn simple_shape(
        &self,
        id: &ShapeID,
        traits: &AppliedTraits,
        shape: &Simple,
    ) -> Result<(), Self::Error> {
        let subject: IRIRef = shape_to_iri(id);
        let mut graph: RefMut<'_, dyn Graph> = self.graph.borrow_mut();
        add_shape(&mut graph, self.model_subject.clone(), subject.clone());
        let statement_factory = graph.statement_factory();
        graph.insert(
            statement_factory
                .statement(
                    statement_factory.named_subject(subject.clone()),
                    rdf::a_type().clone(),
                    statement_factory.named_object(
                        match shape {
                            Simple::Blob => vocabulary::blob_shape(),
                            Simple::Boolean => vocabulary::boolean_shape(),
                            Simple::Document => vocabulary::document_shape(),
                            Simple::String => vocabulary::string_shape(),
                            Simple::Byte => vocabulary::byte_shape(),
                            Simple::Short => vocabulary::short_shape(),
                            Simple::Integer => vocabulary::integer_shape(),
                            Simple::Long => vocabulary::long_shape(),
                            Simple::Float => vocabulary::float_shape(),
                            Simple::Double => vocabulary::double_shape(),
                            Simple::BigInteger => vocabulary::big_integer_shape(),
                            Simple::BigDecimal => vocabulary::big_decimal_shape(),
                            Simple::Timestamp => vocabulary::timestamp_shape(),
                        }
                        .clone(),
                    ),
                )
                .unwrap(),
        );
        from_traits(&mut graph, statement_factory.named_subject(subject), traits)
    }

    fn list(
        &self,
        id: &ShapeID,
        traits: &AppliedTraits,
        shape: &ListOrSet,
    ) -> Result<(), Self::Error> {
        let subject: IRIRef = shape_to_iri(id);
        let mut graph: RefMut<'_, dyn Graph> = self.graph.borrow_mut();
        add_shape(&mut graph, self.model_subject.clone(), subject.clone());
        let statement_factory = graph.statement_factory();
        graph.insert(
            statement_factory
                .statement(
                    statement_factory.named_subject(subject.clone()),
                    rdf::a_type().clone(),
                    statement_factory.named_object(vocabulary::list_shape().clone()),
                )
                .unwrap(),
        );
        from_member(&mut graph, subject.clone(), shape.member())?;
        from_traits(&mut graph, statement_factory.named_subject(subject), traits)
    }

    fn set(
        &self,
        id: &ShapeID,
        traits: &AppliedTraits,
        shape: &ListOrSet,
    ) -> Result<(), Self::Error> {
        let subject: IRIRef = shape_to_iri(id);
        let mut graph: RefMut<'_, dyn Graph> = self.graph.borrow_mut();
        add_shape(&mut graph, self.model_subject.clone(), subject.clone());
        let statement_factory = graph.statement_factory();
        graph.insert(
            statement_factory
                .statement(
                    statement_factory.named_subject(subject.clone()),
                    rdf::a_type().clone(),
                    statement_factory.named_object(vocabulary::set_shape().clone()),
                )
                .unwrap(),
        );
        from_member(&mut graph, subject.clone(), shape.member())?;
        from_traits(&mut graph, statement_factory.named_subject(subject), traits)
    }

    fn map(&self, id: &ShapeID, traits: &AppliedTraits, shape: &Map) -> Result<(), Self::Error> {
        let subject: IRIRef = shape_to_iri(id);
        let mut graph: RefMut<'_, dyn Graph> = self.graph.borrow_mut();
        add_shape(&mut graph, self.model_subject.clone(), subject.clone());
        let statement_factory = graph.statement_factory();
        graph.insert(
            statement_factory
                .statement(
                    statement_factory.named_subject(subject.clone()),
                    rdf::a_type().clone(),
                    statement_factory.named_object(vocabulary::map_shape().clone()),
                )
                .unwrap(),
        );
        from_member(&mut graph, subject.clone(), shape.key())?;
        from_member(&mut graph, subject.clone(), shape.value())?;
        from_traits(&mut graph, statement_factory.named_subject(subject), traits)
    }

    fn structure(
        &self,
        id: &ShapeID,
        traits: &AppliedTraits,
        shape: &StructureOrUnion,
    ) -> Result<(), Self::Error> {
        let subject: IRIRef = shape_to_iri(id);
        let mut graph: RefMut<'_, dyn Graph> = self.graph.borrow_mut();
        add_shape(&mut graph, self.model_subject.clone(), subject.clone());
        let statement_factory = graph.statement_factory();
        graph.insert(
            statement_factory
                .statement(
                    statement_factory.named_subject(subject.clone()),
                    rdf::a_type().clone(),
                    statement_factory.named_object(vocabulary::structure_shape().clone()),
                )
                .unwrap(),
        );
        for member in shape.members() {
            from_member(&mut graph, subject.clone(), member)?;
        }
        from_traits(&mut graph, statement_factory.named_subject(subject), traits)
    }

    fn union(
        &self,
        id: &ShapeID,
        traits: &AppliedTraits,
        shape: &StructureOrUnion,
    ) -> Result<(), Self::Error> {
        let subject: IRIRef = shape_to_iri(id);
        let mut graph: RefMut<'_, dyn Graph> = self.graph.borrow_mut();
        add_shape(&mut graph, self.model_subject.clone(), subject.clone());
        let statement_factory = graph.statement_factory();
        graph.insert(
            statement_factory
                .statement(
                    statement_factory.named_subject(subject.clone()),
                    rdf::a_type().clone(),
                    statement_factory.named_object(vocabulary::union_shape().clone()),
                )
                .unwrap(),
        );
        for member in shape.members() {
            from_member(&mut graph, subject.clone(), member)?;
        }
        from_traits(&mut graph, statement_factory.named_subject(subject), traits)
    }

    fn service(
        &self,
        id: &ShapeID,
        traits: &AppliedTraits,
        shape: &Service,
    ) -> Result<(), Self::Error> {
        let subject: IRIRef = shape_to_iri(id);
        let mut graph: RefMut<'_, dyn Graph> = self.graph.borrow_mut();
        add_shape(&mut graph, self.model_subject.clone(), subject.clone());
        let statement_factory = graph.statement_factory();
        let literal_factory = graph.literal_factory();
        graph.insert(
            statement_factory
                .statement(
                    statement_factory.named_subject(subject.clone()),
                    rdf::a_type().clone(),
                    statement_factory.named_object(vocabulary::service_shape().clone()),
                )
                .unwrap(),
        );
        graph.insert(
            statement_factory
                .statement(
                    statement_factory.named_subject(subject.clone()),
                    vocabulary::version().clone(),
                    statement_factory.literal_object(literal_factory.literal(shape.version())),
                )
                .unwrap(),
        );
        for operation in shape.operations() {
            graph.insert(
                statement_factory
                    .statement(
                        statement_factory.named_subject(subject.clone()),
                        vocabulary::operation().clone(),
                        statement_factory.named_object(shape_to_iri(operation).clone()),
                    )
                    .unwrap(),
            );
        }
        for resource in shape.resources() {
            graph.insert(
                statement_factory
                    .statement(
                        statement_factory.named_subject(subject.clone()),
                        vocabulary::resource().clone(),
                        statement_factory.named_object(shape_to_iri(resource).clone()),
                    )
                    .unwrap(),
            );
        }
        from_traits(&mut graph, statement_factory.named_subject(subject), traits)
    }

    fn operation(
        &self,
        id: &ShapeID,
        traits: &AppliedTraits,
        shape: &Operation,
    ) -> Result<(), Self::Error> {
        let subject: IRIRef = shape_to_iri(id);
        let mut graph: RefMut<'_, dyn Graph> = self.graph.borrow_mut();
        add_shape(&mut graph, self.model_subject.clone(), subject.clone());
        let statement_factory = graph.statement_factory();
        graph.insert(
            statement_factory
                .statement(
                    statement_factory.named_subject(subject.clone()),
                    rdf::a_type().clone(),
                    statement_factory.named_object(vocabulary::operation_shape().clone()),
                )
                .unwrap(),
        );
        if let Some(input) = shape.input() {
            graph.insert(
                statement_factory
                    .statement(
                        statement_factory.named_subject(subject.clone()),
                        vocabulary::input().clone(),
                        statement_factory.named_object(shape_to_iri(input)),
                    )
                    .unwrap(),
            );
        }
        if let Some(output) = shape.output() {
            graph.insert(
                statement_factory
                    .statement(
                        statement_factory.named_subject(subject.clone()),
                        vocabulary::output().clone(),
                        statement_factory.named_object(shape_to_iri(output)),
                    )
                    .unwrap(),
            );
        }
        for error in shape.errors() {
            graph.insert(
                statement_factory
                    .statement(
                        statement_factory.named_subject(subject.clone()),
                        vocabulary::error().clone(),
                        statement_factory.named_object(shape_to_iri(error).clone()),
                    )
                    .unwrap(),
            );
        }
        from_traits(&mut graph, statement_factory.named_subject(subject), traits)
    }

    fn resource(
        &self,
        id: &ShapeID,
        traits: &AppliedTraits,
        shape: &Resource,
    ) -> Result<(), Self::Error> {
        let subject: IRIRef = shape_to_iri(id);
        let mut graph: RefMut<'_, dyn Graph> = self.graph.borrow_mut();
        add_shape(&mut graph, self.model_subject.clone(), subject.clone());
        let statement_factory = graph.statement_factory();
        let literal_factory = graph.literal_factory();
        graph.insert(
            statement_factory
                .statement(
                    statement_factory.named_subject(subject.clone()),
                    rdf::a_type().clone(),
                    statement_factory.named_object(vocabulary::resource_shape().clone()),
                )
                .unwrap(),
        );
        if shape.has_identifiers() {
            let identifier_bag = graph.statement_factory().blank_subject();
            graph.insert(
                statement_factory
                    .statement(
                        statement_factory.named_subject(subject.clone()),
                        vocabulary::identifiers().clone(),
                        statement_factory.subject_as_object(identifier_bag.clone()),
                    )
                    .unwrap(),
            );
            graph.insert(
                statement_factory
                    .statement(
                        identifier_bag.clone(),
                        rdf::a_type().clone(),
                        statement_factory.named_object(rdf::bag().clone()),
                    )
                    .unwrap(),
            );
            for (idx, (name, target)) in shape.identifiers().enumerate() {
                let member = IRIRef::new(
                    IRI::from_str(&format!("{}_{}", rdf::namespace_iri(), idx + 1)).unwrap(),
                );
                let name_target_pair = graph.statement_factory().blank_subject();
                graph.insert(
                    statement_factory
                        .statement(
                            identifier_bag.clone(),
                            member,
                            statement_factory.subject_as_object(name_target_pair.clone()),
                        )
                        .unwrap(),
                );
                graph.insert(
                    statement_factory
                        .statement(
                            name_target_pair.clone(),
                            vocabulary::key().clone(),
                            statement_factory
                                .literal_object(literal_factory.literal(&name.to_string())),
                        )
                        .unwrap(),
                );
                graph.insert(
                    statement_factory
                        .statement(
                            name_target_pair.clone(),
                            vocabulary::value().clone(),
                            statement_factory.named_object(shape_to_iri(target).clone()),
                        )
                        .unwrap(),
                );
            }
        }
        if let Some(create) = shape.create() {
            graph.insert(
                statement_factory
                    .statement(
                        statement_factory.named_subject(subject.clone()),
                        vocabulary::create().clone(),
                        statement_factory.named_object(shape_to_iri(create)),
                    )
                    .unwrap(),
            );
        }
        if let Some(put) = shape.put() {
            graph.insert(
                statement_factory
                    .statement(
                        statement_factory.named_subject(subject.clone()),
                        vocabulary::put().clone(),
                        statement_factory.named_object(shape_to_iri(put)),
                    )
                    .unwrap(),
            );
        }
        if let Some(update) = shape.update() {
            graph.insert(
                statement_factory
                    .statement(
                        statement_factory.named_subject(subject.clone()),
                        vocabulary::update().clone(),
                        statement_factory.named_object(shape_to_iri(update)),
                    )
                    .unwrap(),
            );
        }
        if let Some(delete) = shape.delete() {
            graph.insert(
                statement_factory
                    .statement(
                        statement_factory.named_subject(subject.clone()),
                        vocabulary::delete().clone(),
                        statement_factory.named_object(shape_to_iri(delete)),
                    )
                    .unwrap(),
            );
        }
        if let Some(read) = shape.read() {
            graph.insert(
                statement_factory
                    .statement(
                        statement_factory.named_subject(subject.clone()),
                        vocabulary::read().clone(),
                        statement_factory.named_object(shape_to_iri(read)),
                    )
                    .unwrap(),
            );
        }
        if let Some(list) = shape.list() {
            graph.insert(
                statement_factory
                    .statement(
                        statement_factory.named_subject(subject.clone()),
                        vocabulary::list().clone(),
                        statement_factory.named_object(shape_to_iri(list)),
                    )
                    .unwrap(),
            );
        }
        for operation in shape.operations() {
            graph.insert(
                statement_factory
                    .statement(
                        statement_factory.named_subject(subject.clone()),
                        vocabulary::operation().clone(),
                        statement_factory.named_object(shape_to_iri(operation).clone()),
                    )
                    .unwrap(),
            );
        }
        for operation in shape.collection_operations() {
            graph.insert(
                statement_factory
                    .statement(
                        statement_factory.named_subject(subject.clone()),
                        vocabulary::collection_operation().clone(),
                        statement_factory.named_object(shape_to_iri(operation).clone()),
                    )
                    .unwrap(),
            );
        }
        for resource in shape.resources() {
            graph.insert(
                statement_factory
                    .statement(
                        statement_factory.named_subject(subject.clone()),
                        vocabulary::resource().clone(),
                        statement_factory.named_object(shape_to_iri(resource).clone()),
                    )
                    .unwrap(),
            );
        }
        from_traits(&mut graph, statement_factory.named_subject(subject), traits)
    }

    fn unresolved_id(&self, _id: &ShapeID, _traits: &AppliedTraits) -> Result<(), Self::Error> {
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn add_shape(graph: &mut RefMut<'_, dyn Graph>, the_model: SubjectNodeRef, subject: IRIRef) {
    let statement_factory = graph.statement_factory();
    graph.insert(
        statement_factory
            .statement(
                the_model,
                vocabulary::shape().clone(),
                statement_factory.named_object(subject),
            )
            .unwrap(),
    );
}

fn from_member(
    graph: &mut RefMut<'_, dyn Graph>,
    subject: IRIRef,
    member: &MemberShape,
) -> Result<(), ModelError> {
    let statement_factory = graph.statement_factory();
    let literal_factory = graph.literal_factory();
    let trait_node = graph.statement_factory().blank_subject();
    graph.insert(
        statement_factory
            .statement(
                statement_factory.named_subject(subject),
                vocabulary::member().clone(),
                statement_factory.subject_as_object(trait_node.clone()),
            )
            .unwrap(),
    );
    graph.insert(
        statement_factory
            .statement(
                trait_node.clone(),
                vocabulary::name().clone(),
                statement_factory.literal_object(literal_factory.literal(&member.id().to_string())),
            )
            .unwrap(),
    );
    graph.insert(
        statement_factory
            .statement(
                trait_node.clone(),
                rdf::a_type().clone(),
                statement_factory.named_object(shape_to_iri(member.target())),
            )
            .unwrap(),
    );
    from_traits(graph, trait_node, member.traits())
}

fn from_traits(
    graph: &mut RefMut<'_, dyn Graph>,
    parent: SubjectNodeRef,
    traits: &AppliedTraits,
) -> Result<(), ModelError> {
    let statement_factory = graph.statement_factory();
    for (id, value) in traits {
        let trait_node = graph.statement_factory().blank_subject();
        graph.insert(
            statement_factory
                .statement(
                    parent.clone(),
                    vocabulary::apply().clone(),
                    statement_factory.subject_as_object(trait_node.clone()),
                )
                .unwrap(),
        );
        graph.insert(
            statement_factory
                .statement(
                    trait_node.clone(),
                    vocabulary::trait_shape().clone(),
                    statement_factory.named_object(shape_to_iri(id)),
                )
                .unwrap(),
        );
        if let Some(value) = value {
            from_value(
                graph,
                trait_node.clone(),
                vocabulary::value().clone(),
                value,
            )?;
        }
    }
    Ok(())
}

fn from_value(
    graph: &mut RefMut<'_, dyn Graph>,
    subject: SubjectNodeRef,
    predicate: IRIRef,
    value: &Value,
) -> Result<(), ModelError> {
    let statement_factory = graph.statement_factory();
    let literal_factory = graph.literal_factory();
    match value {
        Value::String(v) => {
            graph.insert(
                statement_factory
                    .statement(
                        subject,
                        predicate,
                        statement_factory.literal_object(literal_factory.literal(v)),
                    )
                    .unwrap(),
            );
        }
        Value::Number(v) => match v {
            Number::Integer(v) => {
                graph.insert(
                    statement_factory
                        .statement(
                            subject,
                            predicate,
                            statement_factory.literal_object(literal_factory.long(*v)),
                        )
                        .unwrap(),
                );
            }
            Number::Float(v) => {
                graph.insert(
                    statement_factory
                        .statement(
                            subject,
                            predicate,
                            statement_factory.literal_object(literal_factory.double(*v)),
                        )
                        .unwrap(),
                );
            }
        },
        Value::Boolean(v) => {
            graph.insert(
                statement_factory
                    .statement(
                        subject,
                        predicate,
                        statement_factory.literal_object(literal_factory.boolean(*v)),
                    )
                    .unwrap(),
            );
        }
        Value::Array(v) => {
            let the_value = graph.statement_factory().blank_subject();
            graph.insert(
                statement_factory
                    .statement(
                        subject,
                        predicate,
                        statement_factory.subject_as_object(the_value.clone()),
                    )
                    .unwrap(),
            );
            graph.insert(
                statement_factory
                    .statement(
                        the_value.clone(),
                        rdf::a_type().clone(),
                        statement_factory.named_object(rdf::list().clone()),
                    )
                    .unwrap(),
            );
            for value in v {
                from_value(graph, the_value.clone(), rdf::li().clone(), value)?;
            }
        }
        Value::Object(v) => {
            let the_value = graph.statement_factory().blank_subject();
            graph.insert(
                statement_factory
                    .statement(
                        subject,
                        predicate,
                        statement_factory.subject_as_object(the_value.clone()),
                    )
                    .unwrap(),
            );
            graph.insert(
                statement_factory
                    .statement(
                        the_value.clone(),
                        rdf::a_type().clone(),
                        statement_factory.named_object(rdf::bag().clone()),
                    )
                    .unwrap(),
            );
            for (k, v) in v {
                let kv_pair = graph.statement_factory().blank_subject();
                graph.insert(
                    statement_factory
                        .statement(
                            the_value.clone(),
                            rdf::li().clone(),
                            statement_factory.subject_as_object(kv_pair.clone()),
                        )
                        .unwrap(),
                );
                graph.insert(
                    statement_factory
                        .statement(
                            kv_pair.clone(),
                            vocabulary::key().clone(),
                            statement_factory.literal_object(literal_factory.literal(k)),
                        )
                        .unwrap(),
                );
                from_value(graph, kv_pair, vocabulary::value().clone(), v)?;
            }
        }
        Value::None => {
            graph.insert(
                statement_factory
                    .statement(
                        subject,
                        predicate,
                        statement_factory.named_object(rdf::nil().clone()),
                    )
                    .unwrap(),
            );
        }
    }
    Ok(())
}
