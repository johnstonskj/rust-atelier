/*!
Implements the mapping between the Smithy semantic model and an RDF graph. The functions
`model_to_rdf` and `rdf_to_model` perform the mapping itself.

# Mapping

This provides a brief description of the Model to RDF mapping; the qualified names in the examples
below use the prefix "smithy" which is defined in [`vocabulary::PREFIX`](../vocabulary/constant.PREFIX.html)
and which maps to the namespace IRI in [`vocabulary::NAMESPACE`](../vocabulary/constant.NAMESPACE.html).

These values are set in the examples below in [Turtle](https://www.w3.org/TR/turtle/) syntax as a
common preamble:

```turtle
@prefix smithy: <https://awslabs.github.io/smithy/vocab/1.0#> .
@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
```

Note the inclusion of the `xsd` namespace for literals.

## Model

1. Each model MUST have a subject, either a provided IRI or a blank node will be created.
1. This subject MUST have an `rdf:type` of `smithy:Model`.
1. This subject MAY have a relationship, typed as `smithy:shapes` to a node with `rdf:type` of
   `rdf:Bag`. This relationship may be omitted if the model contains no shapes.

```turtle
_:subject a smithy:Model ;
            smithy:shapes _:shapes .

_:shapes a rdf:Bag .
```

## Shape

1. Each shape MUST be present as a member of the `smithy:shapes` bag introduced above.
1. The identifier is the URN form of the shapes **shape ID**.
1. The shape MUST include an `rdf:type` statement that denotes it's Smithy type.
1. Additional requirements are type specific and introduced below.

```turtle
_:shapes rdf:li <urn:smithy:example.motd:Shape> .

<urn:smithy:example.motd:Shape> a smithy:String .
```

1. Simple shapes;
   1. no additional rules.
1. List and Set shapes;
   1. An additional statement for the shape MUST be present with the predicate `smithy:member`
      and the object being the URN of the target shape.
   1. This member MAY have traits (see below).
1. Map shapes;
   1. An additional statement for the shape MUST be present with the predicate `smithy:key`
      and the object being the URN of the target shape.
   1. An additional statement for the shape MUST be present with the predicate `smithy:value`
      and the object being the URN of the target shape.
   1. These members MAY have traits (see below).
1. Structure and Union shapes;
   1. Each member of the shape becomes a statement with the shape ID as predicate and the object
      being a URN for the target shape.
   1. These members MAY have traits (see below).
1. Service shapes;
   1. An additional statement for the shape MUST be present with the predicate `smithy:version` and
      the object being a literal, non-empty, string.
   1. Each member of the shape becomes a statement with the corresponding predicate `smithy:*`
      and the object being the URN of the target shape.
   1. For the multi-valued members `operations`, and `resources`, the statement SHALL be repeated
      once for each value.
1. Operation shapes;
   1. Each member of the shape becomes a statement with the corresponding predicate `smithy:*`
      and the object being the URN of the target shape.
   1. For the multi-valued member `errors` the statement SHALL be repeated once for each value.
1. Resource Shapes;
   1. The resource subject MAY have a relationship, typed as `smithy:identifiers` to a node with
      `rdf:type` of `rdf:Bag`. This relationship may be omitted if the model contains no identifier
      pairs.
      1. Each identifier pair consists of a blank node in the bag with two statements;
         1. one with the predicate `smithy:key` and the object being a literal string for the identifier name,
         1. one with the predicate `smithy:value` and the object being the URN of the target shape.
   1. Each member of the shape becomes a statement with the corresponding predicate `smithy:*`
      and the object being the URN of the target shape.
   1. For the multi-valued members `operations`, `collectionOperations`, and `resources`, the
      statement SHALL be repeated once for each value.

## Traits

Any shape, either a top-level shape, or a member, may have traits applied, these are represented as
follows:

1. This shape MAY have a relationship, typed as `smithy:traits` to a node with `rdf:type` of
   `rdf:Bag`. This relationship may be omitted if the shape has no applied traits.
1. Each applied trait is represented as a blank node, with predicate `rdf:li` in the trait bag.
1. This new node MUST include a statement with the predicate `smithy:trait` and object being the
   URN of the trait shape.
1. The new node MAY include a statement with the predicate `smithy:value` and object being the
   value applied with this shape (see production below).

```turtle
<urn:smithy:example.motd:Shape> a smithy:String ;
            smithy:traits _:shape_traits .

_:shape_traits a rdf:Bag ;
            rdf:li _:a_trait .

_:a_trait smithy:trait <urn:smithy:smithy.api:required> .
```

## Values

Values are attached to a node with the predicate `smithy:value` and the value represented as follows:

1. string values MUST be represented as unqualified string literals,
1. boolean values MUST be represented as string literals with the type `xsd:boolean`,
1. numeric values MUST be represented as string literals with either the type `xsd:signedLong` or
   `xsd:double`.
1. null values MUST be represented as `rdf:nil`,
1. array values MUST be represented as a new blank node,
   1. this node MUST have a statement with `rdf:type` of `rdf:List`,
   1. each element in the array occurs in this list with the predicate `rdf:li` and object being
      the value represented using these same production rules,
1. object values MUST be represented as a new blank node,
   1. this node MUST have a statement with `rdf:type` of `rdf:Bag`,
   1. each element in the object occurs in this list with the predicate `rdf:li` and object being
      a new node blank node,
   1. this node MUST have a statement with `smithy:key` and the object being a string literal
      for the identifier name,
   1. this node MUST have a statement with `smithy:value` and the object being the URN of the
      target shape.

```turtle
_:a_trait smithy:trait <urn:smithy:smithy.api:documentation> ;
            smithy:value "Here is some documentation".
```

*/

use crate::urn::shape_to_iri;
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
use rdftk_core::graph::MutableGraph;
use rdftk_core::{DataType, Graph, Literal, ObjectNode, Statement, SubjectNode};
use rdftk_iri::IRIRef;
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
pub fn model_to_rdf(model: &Model, model_iri: Option<IRIRef>) -> ModelResult<Box<dyn Graph>> {
    let model_iri = match model_iri {
        None => SubjectNode::blank(),
        Some(iri) => SubjectNode::named(iri),
    };
    let mut graph = MemGraph::default();

    graph.insert(Statement::new(
        model_iri.clone(),
        rdf::a_type().clone(),
        ObjectNode::named(vocabulary::model().clone()),
    ));

    let shape_bag = SubjectNode::blank();
    graph.insert(Statement::new(
        model_iri,
        vocabulary::shapes().clone(),
        ObjectNode::from(shape_bag.clone()),
    ));
    graph.insert(Statement::new(
        shape_bag.clone(),
        rdf::a_type().clone(),
        ObjectNode::named(rdf::bag().clone()),
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
pub fn rdf_to_model(_rdf_graph: &impl Graph, _model_iri: Option<IRIRef>) -> ModelResult<Model> {
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
        let subject: IRIRef = shape_to_iri(id);
        let mut graph = self.graph.borrow_mut();
        add_shape(&mut graph, self.shape_bag.clone(), subject.clone())?;
        graph.insert(Statement::new(
            subject.clone().into(),
            rdf::a_type().clone(),
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
            .into(),
        ));
        from_traits(&mut graph, subject, traits)
    }

    fn list(
        &self,
        id: &ShapeID,
        traits: &[AppliedTrait],
        shape: &ListOrSet,
    ) -> Result<(), Self::Error> {
        let subject: IRIRef = shape_to_iri(id);
        let mut graph = self.graph.borrow_mut();
        add_shape(&mut graph, self.shape_bag.clone(), subject.clone())?;
        graph.insert(Statement::new(
            subject.clone().into(),
            rdf::a_type().clone(),
            vocabulary::list_shape().into(),
        ));
        graph.insert(Statement::new(
            subject.clone().into(),
            vocabulary::member().clone(),
            shape_to_iri(shape.member().target()).into(),
        ));
        from_traits(&mut graph, subject, traits)
    }

    fn set(
        &self,
        id: &ShapeID,
        traits: &[AppliedTrait],
        shape: &ListOrSet,
    ) -> Result<(), Self::Error> {
        let subject: IRIRef = shape_to_iri(id).into();
        let mut graph = self.graph.borrow_mut();
        add_shape(&mut graph, self.shape_bag.clone(), subject.clone())?;
        graph.insert(Statement::new(
            subject.clone().into(),
            rdf::a_type().clone(),
            vocabulary::set_shape().into(),
        ));
        graph.insert(Statement::new(
            subject.clone().into(),
            vocabulary::member().clone(),
            shape_to_iri(shape.member().target()).into(),
        ));
        from_traits(&mut graph, subject, traits)
    }

    fn map(&self, id: &ShapeID, traits: &[AppliedTrait], shape: &Map) -> Result<(), Self::Error> {
        let subject: IRIRef = shape_to_iri(id);
        let mut graph = self.graph.borrow_mut();
        add_shape(&mut graph, self.shape_bag.clone(), subject.clone())?;
        graph.insert(Statement::new(
            subject.clone().into(),
            rdf::a_type().clone(),
            vocabulary::map_shape().into(),
        ));
        graph.insert(Statement::new(
            subject.clone().into(),
            vocabulary::key().clone(),
            shape_to_iri(shape.key().target()).into(),
        ));
        graph.insert(Statement::new(
            subject.clone().into(),
            vocabulary::key().clone(),
            shape_to_iri(shape.value().target()).into(),
        ));
        from_traits(&mut graph, subject, traits)
    }

    fn structure(
        &self,
        id: &ShapeID,
        traits: &[AppliedTrait],
        shape: &StructureOrUnion,
    ) -> Result<(), Self::Error> {
        let subject: IRIRef = shape_to_iri(id);
        let mut graph = self.graph.borrow_mut();
        add_shape(&mut graph, self.shape_bag.clone(), subject.clone())?;
        graph.insert(Statement::new(
            subject.clone().into(),
            rdf::a_type().clone(),
            vocabulary::structure_shape().into(),
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
        let subject: IRIRef = shape_to_iri(id);
        let mut graph = self.graph.borrow_mut();
        add_shape(&mut graph, self.shape_bag.clone(), subject.clone())?;
        graph.insert(Statement::new(
            subject.clone().into(),
            rdf::a_type().clone(),
            vocabulary::union_shape().into(),
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
        let subject: IRIRef = shape_to_iri(id);
        let mut graph = self.graph.borrow_mut();
        add_shape(&mut graph, self.shape_bag.clone(), subject.clone())?;
        graph.insert(Statement::new(
            subject.clone().into(),
            rdf::a_type().clone(),
            vocabulary::service_shape().into(),
        ));
        graph.insert(Statement::new(
            subject.clone().into(),
            vocabulary::version().clone(),
            Literal::new(shape.version()).into(),
        ));
        for operation in shape.operations() {
            graph.insert(Statement::new(
                subject.clone().into(),
                vocabulary::operation().clone(),
                shape_to_iri(operation).into(),
            ));
        }
        for resource in shape.resources() {
            graph.insert(Statement::new(
                subject.clone().into(),
                vocabulary::resource().clone(),
                shape_to_iri(resource).into(),
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
        let subject: IRIRef = shape_to_iri(id).into();
        let mut graph = self.graph.borrow_mut();
        add_shape(&mut graph, self.shape_bag.clone(), subject.clone())?;
        graph.insert(Statement::new(
            subject.clone().into(),
            rdf::a_type().clone(),
            vocabulary::operation_shape().into(),
        ));
        if let Some(input) = shape.input() {
            graph.insert(Statement::new(
                subject.clone().into(),
                vocabulary::input().clone(),
                shape_to_iri(input).into(),
            ));
        }
        if let Some(output) = shape.output() {
            graph.insert(Statement::new(
                subject.clone().into(),
                vocabulary::output().clone(),
                shape_to_iri(output).into(),
            ));
        }
        for error in shape.errors() {
            graph.insert(Statement::new(
                subject.clone().into(),
                vocabulary::error().clone(),
                shape_to_iri(error).into(),
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
        let subject: IRIRef = shape_to_iri(id).into();
        let mut graph = self.graph.borrow_mut();
        add_shape(&mut graph, self.shape_bag.clone(), subject.clone())?;
        graph.insert(Statement::new(
            subject.clone().into(),
            rdf::a_type().clone(),
            vocabulary::resource_shape().into(),
        ));
        if shape.has_identifiers() {
            let identifier_bag = SubjectNode::blank();
            graph.insert(Statement::new(
                subject.clone().into(),
                vocabulary::identifiers().clone(),
                identifier_bag.clone().into(),
            ));
            graph.insert(Statement::new(
                identifier_bag.clone().into(),
                rdf::a_type().clone(),
                rdf::bag().into(),
            ));
            for (name, target) in shape.identifiers() {
                let name_target_pair = SubjectNode::blank();
                graph.insert(Statement::new(
                    identifier_bag.clone(),
                    rdf::li().clone(),
                    name_target_pair.clone().into(),
                ));
                graph.insert(Statement::new(
                    name_target_pair.clone(),
                    vocabulary::key().clone(),
                    Literal::new(name).into(),
                ));
                from_value(
                    &mut graph,
                    name_target_pair.clone(),
                    vocabulary::value().clone(),
                    target,
                )?;
            }
        }
        if let Some(create) = shape.create() {
            graph.insert(Statement::new(
                subject.clone().clone().into(),
                vocabulary::create().clone(),
                shape_to_iri(create).into(),
            ));
        }
        if let Some(put) = shape.put() {
            graph.insert(Statement::new(
                subject.clone().clone().into(),
                vocabulary::put().clone(),
                shape_to_iri(put).into(),
            ));
        }
        if let Some(update) = shape.update() {
            graph.insert(Statement::new(
                subject.clone().clone().into(),
                vocabulary::update().clone(),
                shape_to_iri(update).into(),
            ));
        }
        if let Some(delete) = shape.delete() {
            graph.insert(Statement::new(
                subject.clone().clone().into(),
                vocabulary::delete().clone(),
                shape_to_iri(delete).into(),
            ));
        }
        if let Some(read) = shape.read() {
            graph.insert(Statement::new(
                subject.clone().clone().into(),
                vocabulary::read().clone(),
                shape_to_iri(read).into(),
            ));
        }
        if let Some(list) = shape.list() {
            graph.insert(Statement::new(
                subject.clone().clone().into(),
                vocabulary::list().clone(),
                shape_to_iri(list).into(),
            ));
        }
        for operation in shape.operations() {
            graph.insert(Statement::new(
                subject.clone().clone().into(),
                vocabulary::operation().clone(),
                shape_to_iri(operation).into(),
            ));
        }
        for operation in shape.collection_operations() {
            graph.insert(Statement::new(
                subject.clone().clone().into(),
                vocabulary::collection_operation().clone(),
                shape_to_iri(operation).into(),
            ));
        }
        for resource in shape.resources() {
            graph.insert(Statement::new(
                subject.clone().into(),
                vocabulary::resource().clone(),
                shape_to_iri(resource).into(),
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
    subject: IRIRef,
) -> Result<(), ModelError> {
    graph.insert(Statement::new(shape_bag, rdf::li().clone(), subject.into()));
    Ok(())
}

fn from_member(
    graph: &mut RefMut<'_, MemGraph>,
    subject: IRIRef,
    member: &MemberShape,
) -> Result<(), ModelError> {
    graph.insert(Statement::new(
        subject.clone().into(),
        shape_to_iri(member.id()).into(),
        shape_to_iri(member.target()).into(),
    ));
    from_traits(graph, subject, member.traits())
}

fn from_traits(
    graph: &mut RefMut<'_, MemGraph>,
    subject: IRIRef,
    traits: &[AppliedTrait],
) -> Result<(), ModelError> {
    if !traits.is_empty() {
        let trait_bag = SubjectNode::blank();
        graph.insert(Statement::new(
            subject.into(),
            vocabulary::traits().clone(),
            trait_bag.clone().into(),
        ));
        graph.insert(Statement::new(
            trait_bag.clone(),
            rdf::a_type().clone(),
            rdf::bag().into(),
        ));

        for a_trait in traits {
            let the_trait = SubjectNode::blank();
            graph.insert(Statement::new(
                trait_bag.clone(),
                rdf::li().clone(),
                the_trait.clone().into(),
            ));
            graph.insert(Statement::new(
                the_trait.clone(),
                vocabulary::trait_name().clone(),
                shape_to_iri(a_trait.id()).into(),
            ));
            if let Some(value) = a_trait.value() {
                from_value(graph, the_trait.clone(), vocabulary::value().clone(), value)?;
            }
        }
    }
    Ok(())
}

fn from_value(
    graph: &mut RefMut<'_, MemGraph>,
    subject: SubjectNode,
    predicate: IRIRef,
    value: &Value,
) -> Result<(), ModelError> {
    match value {
        Value::String(v) => {
            graph.insert(Statement::new(subject, predicate, Literal::new(v).into()));
        }
        Value::Number(v) => match v {
            Number::Integer(v) => {
                graph.insert(Statement::new(
                    subject,
                    predicate,
                    Literal::with_type(&v.to_string(), DataType::UnsignedLong).into(),
                ));
            }
            Number::Float(v) => {
                graph.insert(Statement::new(
                    subject,
                    predicate,
                    Literal::with_type(&v.to_string(), DataType::Double).into(),
                ));
            }
        },
        Value::Boolean(v) => {
            graph.insert(Statement::new(
                subject,
                predicate,
                Literal::with_type(&v.to_string(), DataType::Boolean).into(),
            ));
        }
        Value::Array(v) => {
            let the_value = SubjectNode::blank();
            graph.insert(Statement::new(subject, predicate, the_value.clone().into()));
            graph.insert(Statement::new(
                the_value.clone(),
                rdf::a_type().clone(),
                rdf::list().into(),
            ));
            for value in v {
                from_value(graph, the_value.clone(), rdf::li().clone(), value)?;
            }
        }
        Value::Object(v) => {
            let the_value = SubjectNode::blank();
            graph.insert(Statement::new(subject, predicate, the_value.clone().into()));
            graph.insert(Statement::new(
                the_value.clone(),
                rdf::a_type().clone(),
                rdf::bag().into(),
            ));
            for (k, v) in v {
                let kv_pair = SubjectNode::blank();
                graph.insert(Statement::new(
                    the_value.clone(),
                    rdf::li().clone(),
                    kv_pair.clone().into(),
                ));
                graph.insert(Statement::new(
                    kv_pair.clone(),
                    vocabulary::key().clone(),
                    Literal::new(k).into(),
                ));
                from_value(graph, kv_pair, vocabulary::value().clone(), v)?;
            }
        }
        Value::None => {
            graph.insert(Statement::new(subject, predicate, rdf::nil().into()));
        }
    }
    Ok(())
}
