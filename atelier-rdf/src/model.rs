/*!
Implements the mapping between Smithy and RDF.

```text
```
*/

use crate::vocabulary;
use crate::SmithyUrn;
use atelier_core::error::{Error as ModelError, Result as ModelResult};
use atelier_core::model::shapes::{
    AppliedTrait, ListOrSet, Map, Operation, Resource, Service, Simple, StructureOrUnion,
};
use atelier_core::model::values::Value;
use atelier_core::model::visitor::{walk_model, ModelVisitor};
use atelier_core::model::{Model, ShapeID};
use rdftk_core::{ObjectNode, Statement, SubjectNode};
use rdftk_graph::Graph;
use rdftk_iri::IRI;
use rdftk_memgraph::MemGraph;
use rdftk_names::rdf;
use std::cell::RefCell;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

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
        let mut graph = self.graph.borrow_mut();
        let subject: IRI = SmithyUrn::from(id).into();
        graph.insert(Statement::new(
            SubjectNode::named(subject.clone()),
            rdf::a_type(),
            ObjectNode::named(match shape {
                Simple::Blob => vocabulary::blob(),
                Simple::Boolean => vocabulary::boolean(),
                Simple::Document => vocabulary::document(),
                Simple::String => vocabulary::string(),
                Simple::Byte => vocabulary::byte(),
                Simple::Short => vocabulary::short(),
                Simple::Integer => vocabulary::integer(),
                Simple::Long => vocabulary::long(),
                Simple::Float => vocabulary::float(),
                Simple::Double => vocabulary::double(),
                Simple::BigInteger => vocabulary::big_integer(),
                Simple::BigDecimal => vocabulary::big_decimal(),
                Simple::Timestamp => vocabulary::timestamp(),
            }),
        ));
        self.traits(subject, traits)
    }

    fn list(
        &self,
        id: &ShapeID,
        traits: &[AppliedTrait],
        shape: &ListOrSet,
    ) -> Result<(), Self::Error> {
        let mut graph = self.graph.borrow_mut();
        let subject: IRI = SmithyUrn::from(id).into();
        self.add_shape(subject.clone())?;
        graph.insert(Statement::new(
            SubjectNode::named(subject.clone()),
            rdf::a_type(),
            ObjectNode::named(vocabulary::list_type()),
        ));
        graph.insert(Statement::new(
            SubjectNode::named(subject.clone()),
            vocabulary::member(),
            ObjectNode::named(SmithyUrn::from(shape.member().target()).into()),
        ));
        self.traits(subject, traits)
    }

    fn set(
        &self,
        id: &ShapeID,
        traits: &[AppliedTrait],
        shape: &ListOrSet,
    ) -> Result<(), Self::Error> {
        let mut graph = self.graph.borrow_mut();
        let subject: IRI = SmithyUrn::from(id).into();
        self.add_shape(subject.clone())?;
        graph.insert(Statement::new(
            SubjectNode::named(subject.clone()),
            rdf::a_type(),
            ObjectNode::named(vocabulary::set()),
        ));
        graph.insert(Statement::new(
            SubjectNode::named(subject.clone()),
            vocabulary::member(),
            ObjectNode::named(SmithyUrn::from(shape.member().target()).into()),
        ));
        self.traits(subject, traits)
    }

    fn map(&self, id: &ShapeID, traits: &[AppliedTrait], shape: &Map) -> Result<(), Self::Error> {
        let mut graph = self.graph.borrow_mut();
        let subject: IRI = SmithyUrn::from(id).into();
        self.add_shape(subject.clone())?;
        graph.insert(Statement::new(
            SubjectNode::named(subject.clone()),
            rdf::a_type(),
            ObjectNode::named(vocabulary::map()),
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
        self.traits(subject, traits)
    }

    fn structure(
        &self,
        _id: &ShapeID,
        _traits: &[AppliedTrait],
        _shape: &StructureOrUnion,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn union(
        &self,
        _id: &ShapeID,
        _traits: &[AppliedTrait],
        _shape: &StructureOrUnion,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn service(
        &self,
        _id: &ShapeID,
        _traits: &[AppliedTrait],
        _shape: &Service,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn operation(
        &self,
        _id: &ShapeID,
        _traits: &[AppliedTrait],
        _shape: &Operation,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn resource(
        &self,
        _id: &ShapeID,
        _traits: &[AppliedTrait],
        _shape: &Resource,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn unresolved_id(&self, _id: &ShapeID, _traits: &[AppliedTrait]) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl RdfModelVisitor {
    fn add_shape(&self, subject: IRI) -> Result<(), ModelError> {
        let mut graph = self.graph.borrow_mut();
        graph.insert(Statement::new(
            self.shape_bag.clone(),
            rdf::li(),
            ObjectNode::from(subject),
        ));
        Ok(())
    }

    fn traits(&self, subject: IRI, traits: &[AppliedTrait]) -> Result<(), ModelError> {
        let mut graph = self.graph.borrow_mut();
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
            let subject = SmithyUrn::from(a_trait.id());
            graph.insert(Statement::new(
                trait_bag.clone(),
                rdf::li(),
                ObjectNode::named(subject.into()),
            ));
            // todo: values!!
        }
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
