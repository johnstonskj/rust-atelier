/*!
Implements the mapping between the Smithy semantic model and an RDF graph. The functions
`model_to_rdf` and `rdf_to_model` perform the mapping itself.

# Mapping

This provides a brief description of the Model to RDF mapping.

## Model

## Shape

## Traits

## Values

*/

use crate::urn::SmithyUrn;
use crate::vocabulary;
use atelier_core::error::{Error as ModelError, Result as ModelResult};
use atelier_core::model::shapes::{
    AppliedTrait, ListOrSet, Map, MemberShape, Operation, Resource, Service, Shape, Simple,
    StructureOrUnion,
};
use atelier_core::model::values::Number;
use atelier_core::model::values::Value;
use atelier_core::model::visitor::{walk_model, ModelVisitor};
use atelier_core::model::{Model, ShapeID};
use rdftk_core::{DataType, Literal, ObjectNode, Statement, SubjectNode};
use rdftk_graph::Graph;
use rdftk_iri::IRI;
use rdftk_memgraph::MemGraph;
use rdftk_names::rdf;
use std::cell::{RefCell, RefMut};

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

struct RdfModelVisitor {
    shape_bag: SubjectNode,
    graph: RefCell<MemGraph>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Convert a Smithy semantic model into a canonical RDF graph representation.
///
/// See [module documentation](index.html) for an explanation of the mapping itself.
///
pub fn model_to_rdf(model: &Model, model_iri: Option<&IRI>) -> ModelResult<Box<dyn Graph>> {
    let model_iri = match model_iri {
        None => SubjectNode::blank(),
        Some(iri) => SubjectNode::named(iri.clone()),
    };
    let mut graph = MemGraph::default();

    graph.insert(Statement::new(
        model_iri.clone(),
        rdf::a_type(),
        ObjectNode::named(vocabulary::model()),
    ));

    let shape_bag = SubjectNode::blank();
    graph.insert(Statement::new(
        model_iri,
        vocabulary::shapes(),
        ObjectNode::from(shape_bag.clone()),
    ));
    graph.insert(Statement::new(
        shape_bag.clone(),
        rdf::a_type(),
        ObjectNode::named(rdf::bag()),
    ));

    let visitor = RdfModelVisitor {
        shape_bag,
        graph: RefCell::new(graph),
    };
    walk_model(model, &visitor)?;

    Ok(Box::new(visitor.graph.into_inner()))
}

///
/// Convert an RDF graph into a Smithy semantic model.
///
/// See [module documentation](index.html) for an explanation of the mapping itself.
///
pub fn rdf_to_model(_rdf_graph: &impl Graph, _model_iri: Option<&IRI>) -> ModelResult<Model> {
    unimplemented!()
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl ModelVisitor for RdfModelVisitor {
    type Error = ModelError;

    fn metadata(&self, _key: &str, _value: &Value) -> Result<(), Self::Error> {
        Ok(())
    }

    fn simple_shape(
        &self,
        id: &ShapeID,
        traits: &[AppliedTrait],
        shape: &Simple,
    ) -> Result<(), Self::Error> {
        let subject: IRI = SmithyUrn::from(id).into();
        let mut graph = self.graph.borrow_mut();
        add_shape(&mut graph, self.shape_bag.clone(), subject.clone())?;
        graph.insert(Statement::new(
            SubjectNode::named(subject.clone()),
            rdf::a_type(),
            ObjectNode::named(match shape {
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
            }),
        ));
        from_traits(&mut graph, subject, traits)
    }

    fn list(
        &self,
        id: &ShapeID,
        traits: &[AppliedTrait],
        shape: &ListOrSet,
    ) -> Result<(), Self::Error> {
        let subject: IRI = SmithyUrn::from(id).into();
        let mut graph = self.graph.borrow_mut();
        add_shape(&mut graph, self.shape_bag.clone(), subject.clone())?;
        graph.insert(Statement::new(
            SubjectNode::named(subject.clone()),
            rdf::a_type(),
            ObjectNode::named(vocabulary::list_shape()),
        ));
        graph.insert(Statement::new(
            SubjectNode::named(subject.clone()),
            vocabulary::member(),
            ObjectNode::named(SmithyUrn::from(shape.member().target()).into()),
        ));
        from_traits(&mut graph, subject, traits)
    }

    fn set(
        &self,
        id: &ShapeID,
        traits: &[AppliedTrait],
        shape: &ListOrSet,
    ) -> Result<(), Self::Error> {
        let subject: IRI = SmithyUrn::from(id).into();
        let mut graph = self.graph.borrow_mut();
        add_shape(&mut graph, self.shape_bag.clone(), subject.clone())?;
        graph.insert(Statement::new(
            SubjectNode::named(subject.clone()),
            rdf::a_type(),
            ObjectNode::named(vocabulary::set_shape()),
        ));
        graph.insert(Statement::new(
            SubjectNode::named(subject.clone()),
            vocabulary::member(),
            ObjectNode::named(SmithyUrn::from(shape.member().target()).into()),
        ));
        from_traits(&mut graph, subject, traits)
    }

    fn map(&self, id: &ShapeID, traits: &[AppliedTrait], shape: &Map) -> Result<(), Self::Error> {
        let subject: IRI = SmithyUrn::from(id).into();
        let mut graph = self.graph.borrow_mut();
        add_shape(&mut graph, self.shape_bag.clone(), subject.clone())?;
        graph.insert(Statement::new(
            SubjectNode::named(subject.clone()),
            rdf::a_type(),
            ObjectNode::named(vocabulary::map_shape()),
        ));
        graph.insert(Statement::new(
            SubjectNode::named(subject.clone()),
            vocabulary::key(),
            ObjectNode::named(SmithyUrn::from(shape.key().target()).into()),
        ));
        graph.insert(Statement::new(
            SubjectNode::named(subject.clone()),
            vocabulary::key(),
            ObjectNode::named(SmithyUrn::from(shape.value().target()).into()),
        ));
        from_traits(&mut graph, subject, traits)
    }

    fn structure(
        &self,
        id: &ShapeID,
        traits: &[AppliedTrait],
        shape: &StructureOrUnion,
    ) -> Result<(), Self::Error> {
        let subject: IRI = SmithyUrn::from(id).into();
        let mut graph = self.graph.borrow_mut();
        add_shape(&mut graph, self.shape_bag.clone(), subject.clone())?;
        graph.insert(Statement::new(
            SubjectNode::named(subject.clone()),
            rdf::a_type(),
            ObjectNode::named(vocabulary::structure_shape()),
        ));
        for member in shape.members() {
            from_member(&mut graph, subject.clone(), member)?;
        }
        from_traits(&mut graph, subject, traits)
    }

    fn union(
        &self,
        id: &ShapeID,
        traits: &[AppliedTrait],
        shape: &StructureOrUnion,
    ) -> Result<(), Self::Error> {
        let subject: IRI = SmithyUrn::from(id).into();
        let mut graph = self.graph.borrow_mut();
        add_shape(&mut graph, self.shape_bag.clone(), subject.clone())?;
        graph.insert(Statement::new(
            SubjectNode::named(subject.clone()),
            rdf::a_type(),
            ObjectNode::named(vocabulary::union_shape()),
        ));
        for member in shape.members() {
            from_member(&mut graph, subject.clone(), member)?;
        }
        from_traits(&mut graph, subject, traits)
    }

    fn service(
        &self,
        id: &ShapeID,
        traits: &[AppliedTrait],
        shape: &Service,
    ) -> Result<(), Self::Error> {
        let subject: IRI = SmithyUrn::from(id).into();
        let mut graph = self.graph.borrow_mut();
        add_shape(&mut graph, self.shape_bag.clone(), subject.clone())?;
        graph.insert(Statement::new(
            SubjectNode::named(subject.clone()),
            rdf::a_type(),
            ObjectNode::named(vocabulary::service_shape()),
        ));
        graph.insert(Statement::new(
            SubjectNode::named(subject.clone()),
            vocabulary::version(),
            ObjectNode::from(Literal::new(shape.version())),
        ));
        for operation in shape.operations() {
            graph.insert(Statement::new(
                SubjectNode::named(subject.clone()),
                vocabulary::operation(),
                ObjectNode::named(SmithyUrn::from(operation).into()),
            ));
        }
        for resource in shape.resources() {
            graph.insert(Statement::new(
                SubjectNode::named(subject.clone()),
                vocabulary::resource(),
                ObjectNode::named(SmithyUrn::from(resource).into()),
            ));
        }
        from_traits(&mut graph, subject, traits)
    }

    fn operation(
        &self,
        id: &ShapeID,
        traits: &[AppliedTrait],
        shape: &Operation,
    ) -> Result<(), Self::Error> {
        let subject: IRI = SmithyUrn::from(id).into();
        let mut graph = self.graph.borrow_mut();
        add_shape(&mut graph, self.shape_bag.clone(), subject.clone())?;
        graph.insert(Statement::new(
            SubjectNode::named(subject.clone()),
            rdf::a_type(),
            ObjectNode::named(vocabulary::operation_shape()),
        ));
        if let Some(input) = shape.input() {
            graph.insert(Statement::new(
                SubjectNode::named(subject.clone()),
                vocabulary::input(),
                ObjectNode::named(SmithyUrn::from(input).into()),
            ));
        }
        if let Some(output) = shape.output() {
            graph.insert(Statement::new(
                SubjectNode::named(subject.clone()),
                vocabulary::output(),
                ObjectNode::named(SmithyUrn::from(output).into()),
            ));
        }
        for error in shape.errors() {
            graph.insert(Statement::new(
                SubjectNode::named(subject.clone()),
                vocabulary::error(),
                ObjectNode::named(SmithyUrn::from(error).into()),
            ));
        }
        from_traits(&mut graph, subject, traits)
    }

    fn resource(
        &self,
        id: &ShapeID,
        traits: &[AppliedTrait],
        shape: &Resource,
    ) -> Result<(), Self::Error> {
        let subject: IRI = SmithyUrn::from(id).into();
        let mut graph = self.graph.borrow_mut();
        add_shape(&mut graph, self.shape_bag.clone(), subject.clone())?;
        graph.insert(Statement::new(
            SubjectNode::named(subject.clone()),
            rdf::a_type(),
            ObjectNode::named(vocabulary::resource_shape()),
        ));
        if let Some(create) = shape.create() {
            graph.insert(Statement::new(
                SubjectNode::named(subject.clone()),
                vocabulary::create(),
                ObjectNode::named(SmithyUrn::from(create).into()),
            ));
        }
        if let Some(put) = shape.put() {
            graph.insert(Statement::new(
                SubjectNode::named(subject.clone()),
                vocabulary::put(),
                ObjectNode::named(SmithyUrn::from(put).into()),
            ));
        }
        if let Some(update) = shape.update() {
            graph.insert(Statement::new(
                SubjectNode::named(subject.clone()),
                vocabulary::update(),
                ObjectNode::named(SmithyUrn::from(update).into()),
            ));
        }
        if let Some(delete) = shape.delete() {
            graph.insert(Statement::new(
                SubjectNode::named(subject.clone()),
                vocabulary::delete(),
                ObjectNode::named(SmithyUrn::from(delete).into()),
            ));
        }
        if let Some(read) = shape.read() {
            graph.insert(Statement::new(
                SubjectNode::named(subject.clone()),
                vocabulary::read(),
                ObjectNode::named(SmithyUrn::from(read).into()),
            ));
        }
        if let Some(list) = shape.list() {
            graph.insert(Statement::new(
                SubjectNode::named(subject.clone()),
                vocabulary::list(),
                ObjectNode::named(SmithyUrn::from(list).into()),
            ));
        }
        for operation in shape.operations() {
            graph.insert(Statement::new(
                SubjectNode::named(subject.clone()),
                vocabulary::operation(),
                ObjectNode::named(SmithyUrn::from(operation).into()),
            ));
        }
        for operation in shape.collection_operations() {
            graph.insert(Statement::new(
                SubjectNode::named(subject.clone()),
                vocabulary::collection_operation(),
                ObjectNode::named(SmithyUrn::from(operation).into()),
            ));
        }
        for resource in shape.resources() {
            graph.insert(Statement::new(
                SubjectNode::named(subject.clone()),
                vocabulary::resource(),
                ObjectNode::named(SmithyUrn::from(resource).into()),
            ));
        }
        from_traits(&mut graph, subject, traits)
    }

    fn unresolved_id(&self, _id: &ShapeID, _traits: &[AppliedTrait]) -> Result<(), Self::Error> {
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn add_shape(
    graph: &mut RefMut<'_, MemGraph>,
    shape_bag: SubjectNode,
    subject: IRI,
) -> Result<(), ModelError> {
    graph.insert(Statement::new(
        shape_bag,
        rdf::li(),
        ObjectNode::from(subject),
    ));
    Ok(())
}

fn from_member(
    graph: &mut RefMut<'_, MemGraph>,
    subject: IRI,
    member: &MemberShape,
) -> Result<(), ModelError> {
    graph.insert(Statement::new(
        SubjectNode::named(subject.clone()),
        SmithyUrn::from(member.id().clone()).into(),
        ObjectNode::named(SmithyUrn::from(member.target().clone()).into()),
    ));
    from_traits(graph, subject, member.traits())
}

fn from_traits(
    graph: &mut RefMut<'_, MemGraph>,
    subject: IRI,
    traits: &[AppliedTrait],
) -> Result<(), ModelError> {
    if !traits.is_empty() {
        let trait_bag = SubjectNode::blank();
        graph.insert(Statement::new(
            SubjectNode::named(subject),
            vocabulary::traits(),
            ObjectNode::from(trait_bag.clone()),
        ));
        graph.insert(Statement::new(
            trait_bag.clone(),
            rdf::a_type(),
            ObjectNode::named(rdf::bag()),
        ));

        for a_trait in traits {
            let the_trait = SubjectNode::blank();
            graph.insert(Statement::new(
                trait_bag.clone(),
                rdf::li(),
                ObjectNode::from(the_trait.clone()),
            ));
            graph.insert(Statement::new(
                the_trait.clone(),
                vocabulary::trait_name(),
                ObjectNode::named(SmithyUrn::from(a_trait.id()).into()),
            ));
            if let Some(value) = a_trait.value() {
                from_value(graph, the_trait.clone(), vocabulary::value(), value)?;
            }
        }
    }
    Ok(())
}

fn from_value(
    graph: &mut RefMut<'_, MemGraph>,
    subject: SubjectNode,
    predicate: IRI,
    value: &Value,
) -> Result<(), ModelError> {
    match value {
        Value::String(v) => {
            graph.insert(Statement::new(
                subject,
                predicate,
                ObjectNode::from(Literal::new(v)),
            ));
        }
        Value::Number(v) => match v {
            Number::Integer(v) => {
                graph.insert(Statement::new(
                    subject,
                    predicate,
                    ObjectNode::from(Literal::with_type(&v.to_string(), DataType::UnsignedLong)),
                ));
            }
            Number::Float(v) => {
                graph.insert(Statement::new(
                    subject,
                    predicate,
                    ObjectNode::from(Literal::with_type(&v.to_string(), DataType::Double)),
                ));
            }
        },
        Value::Boolean(v) => {
            graph.insert(Statement::new(
                subject,
                predicate,
                ObjectNode::from(Literal::with_type(&v.to_string(), DataType::Boolean)),
            ));
        }
        Value::Array(v) => {
            let the_value = SubjectNode::blank();
            graph.insert(Statement::new(
                subject,
                predicate,
                ObjectNode::from(the_value.clone()),
            ));
            graph.insert(Statement::new(
                the_value.clone(),
                rdf::a_type(),
                ObjectNode::from(rdf::list()),
            ));
            for value in v {
                from_value(graph, the_value.clone(), rdf::li(), value)?;
            }
        }
        Value::Object(v) => {
            let the_value = SubjectNode::blank();
            graph.insert(Statement::new(
                subject,
                predicate,
                ObjectNode::from(the_value.clone()),
            ));
            graph.insert(Statement::new(
                the_value.clone(),
                rdf::a_type(),
                ObjectNode::from(rdf::bag()),
            ));
            for (k, v) in v {
                let kv_pair = SubjectNode::blank();
                graph.insert(Statement::new(
                    the_value.clone(),
                    rdf::li(),
                    ObjectNode::from(kv_pair.clone()),
                ));
                graph.insert(Statement::new(
                    kv_pair.clone(),
                    vocabulary::key(),
                    ObjectNode::from(Literal::new(k)),
                ));
                from_value(graph, kv_pair, vocabulary::value(), v)?;
            }
        }
        Value::None => {
            graph.insert(Statement::new(
                subject,
                predicate,
                ObjectNode::named(rdf::nil()),
            ));
        }
    }
    Ok(())
}
