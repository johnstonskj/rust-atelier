/*!
Provides an [`model_to_rdf`](fn.model_to_rdf.html) function and [`RdfWriter`](struct.RdfWriter.html)
type that will write a model into RDF.

# Example - model_to_rdf

The following simply constructs an in-memory RDF Graph from a provided model.

```rust
use atelier_core::model::Model;
use atelier_rdf::writer::model_to_rdf;
# use atelier_core::Version;
# fn make_model() -> Model { Model::new(Version::default()) }

let model = make_model();
let rdf_graph = model_to_rdf(&model, None).unwrap();
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
use rdftk_core::graph::mapping::PrefixMappings;
use rdftk_core::graph::MutableGraph;
use rdftk_core::statement::SubjectNodeRef;
use rdftk_core::{DataType, Literal, ObjectNode, Statement, SubjectNode};
use rdftk_io::turtle::writer::TurtleWriter;
use rdftk_io::turtle::NAME;
use rdftk_io::GraphWriter;
use rdftk_iri::{IRIRef, IRI};
use rdftk_memgraph::{Mappings, MemGraph};
use rdftk_names::rdf;
use std::cell::{RefCell, RefMut};
use std::io::Write;
use std::rc::Rc;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Simple implementation of the `ModelWriter` trait that writes the RDF representation of a model.
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
    graph: RefCell<MemGraph>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Convert a Smithy semantic model into a canonical RDF graph representation.
///
pub fn model_to_rdf(model: &Model, model_iri: Option<IRIRef>) -> ModelResult<MemGraph> {
    let model_subject = match model_iri {
        None => SubjectNode::blank_ref(),
        Some(iri) => SubjectNode::named_ref(iri),
    };
    let mut graph = MemGraph::default();
    let mut mappings = Mappings::default();
    let _ = mappings
        .include_xsd()
        .include_rdf()
        .include_rdfs()
        .insert(
            vocabulary::default_prefix(),
            vocabulary::namespace_iri().clone(),
        )
        .insert(
            "api",
            IRIRef::from(IRI::from_str("urn:smithy:smithy.api:").unwrap()),
        );
    let _ = graph.mappings(Rc::from(mappings));

    graph.insert(Statement::new_ref(
        model_subject.clone(),
        vocabulary::smithy_version().clone(),
        ObjectNode::literal_ref(Literal::from(model.smithy_version().to_string())),
    ));

    graph.insert(Statement::new_ref(
        model_subject.clone(),
        rdf::a_type().clone(),
        ObjectNode::named_ref(vocabulary::model().clone()),
    ));

    let visitor = RdfModelVisitor {
        model_subject,
        graph: RefCell::new(graph),
    };
    walk_model(model, &visitor)?;

    Ok(visitor.graph.into_inner())
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
        let rdf_graph = model_to_rdf(model, self.model_iri.clone())?;

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
        let mut graph = self.graph.borrow_mut();
        add_shape(&mut graph, self.model_subject.clone(), subject.clone());
        graph.insert(Statement::new_ref(
            SubjectNode::named_ref(subject.clone()),
            rdf::a_type().clone(),
            ObjectNode::named_ref(
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
        ));
        from_traits(&mut graph, SubjectNode::named_ref(subject), traits)
    }

    fn list(
        &self,
        id: &ShapeID,
        traits: &AppliedTraits,
        shape: &ListOrSet,
    ) -> Result<(), Self::Error> {
        let subject: IRIRef = shape_to_iri(id);
        let mut graph = self.graph.borrow_mut();
        add_shape(&mut graph, self.model_subject.clone(), subject.clone());
        graph.insert(Statement::new_ref(
            SubjectNode::named_ref(subject.clone()),
            rdf::a_type().clone(),
            ObjectNode::named_ref(vocabulary::list_shape().clone()),
        ));
        from_member(&mut graph, subject.clone(), shape.member())?;
        from_traits(&mut graph, SubjectNode::named_ref(subject), traits)
    }

    fn set(
        &self,
        id: &ShapeID,
        traits: &AppliedTraits,
        shape: &ListOrSet,
    ) -> Result<(), Self::Error> {
        let subject: IRIRef = shape_to_iri(id);
        let mut graph = self.graph.borrow_mut();
        add_shape(&mut graph, self.model_subject.clone(), subject.clone());
        graph.insert(Statement::new_ref(
            SubjectNode::named_ref(subject.clone()),
            rdf::a_type().clone(),
            ObjectNode::named_ref(vocabulary::set_shape().clone()),
        ));
        from_member(&mut graph, subject.clone(), shape.member())?;
        from_traits(&mut graph, SubjectNode::named_ref(subject), traits)
    }

    fn map(&self, id: &ShapeID, traits: &AppliedTraits, shape: &Map) -> Result<(), Self::Error> {
        let subject: IRIRef = shape_to_iri(id);
        let mut graph = self.graph.borrow_mut();
        add_shape(&mut graph, self.model_subject.clone(), subject.clone());
        graph.insert(Statement::new_ref(
            SubjectNode::named_ref(subject.clone()),
            rdf::a_type().clone(),
            ObjectNode::named_ref(vocabulary::map_shape().clone()),
        ));
        from_member(&mut graph, subject.clone(), shape.key())?;
        from_member(&mut graph, subject.clone(), shape.value())?;
        from_traits(&mut graph, SubjectNode::named_ref(subject), traits)
    }

    fn structure(
        &self,
        id: &ShapeID,
        traits: &AppliedTraits,
        shape: &StructureOrUnion,
    ) -> Result<(), Self::Error> {
        let subject: IRIRef = shape_to_iri(id);
        let mut graph = self.graph.borrow_mut();
        add_shape(&mut graph, self.model_subject.clone(), subject.clone());
        graph.insert(Statement::new_ref(
            SubjectNode::named_ref(subject.clone()),
            rdf::a_type().clone(),
            ObjectNode::named_ref(vocabulary::structure_shape().clone()),
        ));
        for member in shape.members() {
            from_member(&mut graph, subject.clone(), member)?;
        }
        from_traits(&mut graph, SubjectNode::named_ref(subject), traits)
    }

    fn union(
        &self,
        id: &ShapeID,
        traits: &AppliedTraits,
        shape: &StructureOrUnion,
    ) -> Result<(), Self::Error> {
        let subject: IRIRef = shape_to_iri(id);
        let mut graph = self.graph.borrow_mut();
        add_shape(&mut graph, self.model_subject.clone(), subject.clone());
        graph.insert(Statement::new_ref(
            SubjectNode::named_ref(subject.clone()),
            rdf::a_type().clone(),
            ObjectNode::named_ref(vocabulary::union_shape().clone()),
        ));
        for member in shape.members() {
            from_member(&mut graph, subject.clone(), member)?;
        }
        from_traits(&mut graph, SubjectNode::named_ref(subject), traits)
    }

    fn service(
        &self,
        id: &ShapeID,
        traits: &AppliedTraits,
        shape: &Service,
    ) -> Result<(), Self::Error> {
        let subject: IRIRef = shape_to_iri(id);
        let mut graph = self.graph.borrow_mut();
        add_shape(&mut graph, self.model_subject.clone(), subject.clone());
        graph.insert(Statement::new_ref(
            SubjectNode::named_ref(subject.clone()),
            rdf::a_type().clone(),
            ObjectNode::named_ref(vocabulary::service_shape().clone()),
        ));
        graph.insert(Statement::new_ref(
            SubjectNode::named_ref(subject.clone()),
            vocabulary::version().clone(),
            Literal::new(shape.version()).into(),
        ));
        for operation in shape.operations() {
            graph.insert(Statement::new_ref(
                SubjectNode::named_ref(subject.clone()),
                vocabulary::operation().clone(),
                ObjectNode::named_ref(shape_to_iri(operation).clone()),
            ));
        }
        for resource in shape.resources() {
            graph.insert(Statement::new_ref(
                SubjectNode::named_ref(subject.clone()),
                vocabulary::resource().clone(),
                ObjectNode::named_ref(shape_to_iri(resource).clone()),
            ));
        }
        from_traits(&mut graph, SubjectNode::named_ref(subject), traits)
    }

    fn operation(
        &self,
        id: &ShapeID,
        traits: &AppliedTraits,
        shape: &Operation,
    ) -> Result<(), Self::Error> {
        let subject: IRIRef = shape_to_iri(id);
        let mut graph = self.graph.borrow_mut();
        add_shape(&mut graph, self.model_subject.clone(), subject.clone());
        graph.insert(Statement::new_ref(
            SubjectNode::named_ref(subject.clone()),
            rdf::a_type().clone(),
            ObjectNode::named_ref(vocabulary::operation_shape().clone()),
        ));
        if let Some(input) = shape.input() {
            graph.insert(Statement::new_ref(
                SubjectNode::named_ref(subject.clone()),
                vocabulary::input().clone(),
                ObjectNode::named_ref(shape_to_iri(input)),
            ));
        }
        if let Some(output) = shape.output() {
            graph.insert(Statement::new_ref(
                SubjectNode::named_ref(subject.clone()),
                vocabulary::output().clone(),
                ObjectNode::named_ref(shape_to_iri(output)),
            ));
        }
        for error in shape.errors() {
            graph.insert(Statement::new_ref(
                SubjectNode::named_ref(subject.clone()),
                vocabulary::error().clone(),
                ObjectNode::named_ref(shape_to_iri(error).clone()),
            ));
        }
        from_traits(&mut graph, SubjectNode::named_ref(subject), traits)
    }

    fn resource(
        &self,
        id: &ShapeID,
        traits: &AppliedTraits,
        shape: &Resource,
    ) -> Result<(), Self::Error> {
        let subject: IRIRef = shape_to_iri(id);
        let mut graph = self.graph.borrow_mut();
        add_shape(&mut graph, self.model_subject.clone(), subject.clone());
        graph.insert(Statement::new_ref(
            SubjectNode::named_ref(subject.clone()),
            rdf::a_type().clone(),
            ObjectNode::named_ref(vocabulary::resource_shape().clone()),
        ));
        if shape.has_identifiers() {
            let identifier_bag = SubjectNode::blank_ref();
            graph.insert(Statement::new_ref(
                SubjectNode::named_ref(subject.clone()),
                vocabulary::identifiers().clone(),
                identifier_bag.as_object(),
            ));
            graph.insert(Statement::new_ref(
                identifier_bag.clone(),
                rdf::a_type().clone(),
                ObjectNode::named_ref(rdf::bag().clone()),
            ));
            for (idx, (name, target)) in shape.identifiers().enumerate() {
                let member = IRIRef::new(
                    IRI::from_str(&format!("{}_{}", rdf::namespace_iri(), idx + 1)).unwrap(),
                );
                let name_target_pair = SubjectNode::blank_ref();
                graph.insert(Statement::new_ref(
                    identifier_bag.clone(),
                    member,
                    name_target_pair.as_object(),
                ));
                graph.insert(Statement::new_ref(
                    name_target_pair.clone(),
                    vocabulary::key().clone(),
                    Literal::new(&name.to_string()).into(),
                ));
                graph.insert(Statement::new_ref(
                    name_target_pair.clone(),
                    vocabulary::value().clone(),
                    ObjectNode::named_ref(shape_to_iri(target).clone()),
                ));
            }
        }
        if let Some(create) = shape.create() {
            graph.insert(Statement::new_ref(
                SubjectNode::named_ref(subject.clone()),
                vocabulary::create().clone(),
                ObjectNode::named_ref(shape_to_iri(create)),
            ));
        }
        if let Some(put) = shape.put() {
            graph.insert(Statement::new_ref(
                SubjectNode::named_ref(subject.clone()),
                vocabulary::put().clone(),
                ObjectNode::named_ref(shape_to_iri(put)),
            ));
        }
        if let Some(update) = shape.update() {
            graph.insert(Statement::new_ref(
                SubjectNode::named_ref(subject.clone()),
                vocabulary::update().clone(),
                ObjectNode::named_ref(shape_to_iri(update)),
            ));
        }
        if let Some(delete) = shape.delete() {
            graph.insert(Statement::new_ref(
                SubjectNode::named_ref(subject.clone()),
                vocabulary::delete().clone(),
                ObjectNode::named_ref(shape_to_iri(delete)),
            ));
        }
        if let Some(read) = shape.read() {
            graph.insert(Statement::new_ref(
                SubjectNode::named_ref(subject.clone()),
                vocabulary::read().clone(),
                ObjectNode::named_ref(shape_to_iri(read)),
            ));
        }
        if let Some(list) = shape.list() {
            graph.insert(Statement::new_ref(
                SubjectNode::named_ref(subject.clone()),
                vocabulary::list().clone(),
                ObjectNode::named_ref(shape_to_iri(list)),
            ));
        }
        for operation in shape.operations() {
            graph.insert(Statement::new_ref(
                SubjectNode::named_ref(subject.clone()),
                vocabulary::operation().clone(),
                ObjectNode::named_ref(shape_to_iri(operation).clone()),
            ));
        }
        for operation in shape.collection_operations() {
            graph.insert(Statement::new_ref(
                SubjectNode::named_ref(subject.clone()),
                vocabulary::collection_operation().clone(),
                ObjectNode::named_ref(shape_to_iri(operation).clone()),
            ));
        }
        for resource in shape.resources() {
            graph.insert(Statement::new_ref(
                SubjectNode::named_ref(subject.clone()),
                vocabulary::resource().clone(),
                ObjectNode::named_ref(shape_to_iri(resource).clone()),
            ));
        }
        from_traits(&mut graph, SubjectNode::named_ref(subject), traits)
    }

    fn unresolved_id(&self, _id: &ShapeID, _traits: &AppliedTraits) -> Result<(), Self::Error> {
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn add_shape(graph: &mut RefMut<'_, MemGraph>, the_model: SubjectNodeRef, subject: IRIRef) {
    graph.insert(Statement::new_ref(
        the_model,
        vocabulary::shape().clone(),
        ObjectNode::named_ref(subject),
    ));
}

fn from_member(
    graph: &mut RefMut<'_, MemGraph>,
    subject: IRIRef,
    member: &MemberShape,
) -> Result<(), ModelError> {
    let trait_node = SubjectNode::blank_ref();
    graph.insert(Statement::new_ref(
        SubjectNode::named_ref(subject),
        vocabulary::member().clone(),
        trait_node.as_object(),
    ));
    graph.insert(Statement::new_ref(
        trait_node.clone(),
        vocabulary::name().clone(),
        Literal::from(member.id().member_name().as_ref().unwrap().to_string()).into(),
    ));
    graph.insert(Statement::new_ref(
        trait_node.clone(),
        rdf::a_type().clone(),
        ObjectNode::named_ref(shape_to_iri(member.target())),
    ));
    from_traits(graph, trait_node, member.traits())
}

fn from_traits(
    graph: &mut RefMut<'_, MemGraph>,
    parent: SubjectNodeRef,
    traits: &AppliedTraits,
) -> Result<(), ModelError> {
    for (id, value) in traits {
        let trait_node = SubjectNode::blank_ref();
        graph.insert(Statement::new_ref(
            parent.clone(),
            vocabulary::apply().clone(),
            trait_node.as_object(),
        ));
        graph.insert(Statement::new_ref(
            trait_node.clone(),
            vocabulary::trait_shape().clone(),
            ObjectNode::named_ref(shape_to_iri(id)),
        ));
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
    graph: &mut RefMut<'_, MemGraph>,
    subject: SubjectNodeRef,
    predicate: IRIRef,
    value: &Value,
) -> Result<(), ModelError> {
    match value {
        Value::String(v) => {
            graph.insert(Statement::new_ref(
                subject,
                predicate,
                Literal::new(v).into(),
            ));
        }
        Value::Number(v) => match v {
            Number::Integer(v) => {
                graph.insert(Statement::new_ref(
                    subject,
                    predicate,
                    Literal::with_type(&v.to_string(), DataType::UnsignedLong).into(),
                ));
            }
            Number::Float(v) => {
                graph.insert(Statement::new_ref(
                    subject,
                    predicate,
                    Literal::with_type(&v.to_string(), DataType::Double).into(),
                ));
            }
        },
        Value::Boolean(v) => {
            graph.insert(Statement::new_ref(
                subject,
                predicate,
                Literal::with_type(&v.to_string(), DataType::Boolean).into(),
            ));
        }
        Value::Array(v) => {
            let the_value = SubjectNode::blank_ref();
            graph.insert(Statement::new_ref(
                subject,
                predicate,
                the_value.as_object(),
            ));
            graph.insert(Statement::new_ref(
                the_value.clone(),
                rdf::a_type().clone(),
                ObjectNode::named_ref(rdf::list().clone()),
            ));
            for value in v {
                from_value(graph, the_value.clone(), rdf::li().clone(), value)?;
            }
        }
        Value::Object(v) => {
            let the_value = SubjectNode::blank_ref();
            graph.insert(Statement::new_ref(
                subject,
                predicate,
                the_value.as_object(),
            ));
            graph.insert(Statement::new_ref(
                the_value.clone(),
                rdf::a_type().clone(),
                ObjectNode::named_ref(rdf::bag().clone()),
            ));
            for (k, v) in v {
                let kv_pair = SubjectNode::blank_ref();
                graph.insert(Statement::new_ref(
                    the_value.clone(),
                    rdf::li().clone(),
                    kv_pair.as_object(),
                ));
                graph.insert(Statement::new_ref(
                    kv_pair.clone(),
                    vocabulary::key().clone(),
                    Literal::new(k).into(),
                ));
                from_value(graph, kv_pair, vocabulary::value().clone(), v)?;
            }
        }
        Value::None => {
            graph.insert(Statement::new_ref(
                subject,
                predicate,
                ObjectNode::named_ref(rdf::nil().clone()),
            ));
        }
    }
    Ok(())
}
