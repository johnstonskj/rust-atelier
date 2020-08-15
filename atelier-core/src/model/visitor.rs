/*!
This module provides a trait and public function to help implement model visitors.

For more information, see [the Rust Atelier book](https://rust-atelier.dev/using/visitor.html).

*/

use crate::model::shapes::{
    AppliedTrait, ListOrSet, Map, Operation, Resource, Service, Shape, ShapeKind, Simple,
    StructureOrUnion,
};
use crate::model::values::Value;
use crate::model::{Model, ShapeID};

// ------------------------------------------------------------------------------------------------
// Macros
// ------------------------------------------------------------------------------------------------

macro_rules! visit_fn {
    (mut $fn_name:ident, $shape_type:ty, $doc:expr) => {
        #[doc = $doc]
        #[allow(unused_variables)]
        fn $fn_name(
            &mut self,
            id: &ShapeID,
            traits: &[AppliedTrait],
            shape: &$shape_type,
        ) -> Result<(), Self::Error> {
            Ok(())
        }
    };
    (mut $fn_name:ident, $doc:expr) => {
        #[doc = $doc]
        #[allow(unused_variables)]
        fn $fn_name(
            &mut self,
            id: &ShapeID,
            traits: &[AppliedTrait],
        ) -> Result<(), Self::Error> {
            Ok(())
        }
    };
    ($fn_name:ident, $shape_type:ty, $doc:expr) => {
        #[doc = $doc]
        #[allow(unused_variables)]
        fn $fn_name(
            &self,
            id: &ShapeID,
            traits: &[AppliedTrait],
            shape: &$shape_type,
        ) -> Result<(), Self::Error> {
            Ok(())
        }
    };
    ($fn_name:ident, $doc:expr) => {
        #[doc = $doc]
        #[allow(unused_variables)]
        fn $fn_name(
            &self,
            id: &ShapeID,
            traits: &[AppliedTrait],
        ) -> Result<(), Self::Error> {
            Ok(())
        }
    };
}
// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A trait implemented by tools that wish to visit parts of the model and may choose to ignore
/// some In this way a simple filter to read structures for example can be applied.
///
/// Each method in the trait will return `Ok` by default so a particular implementation can choose
/// which methods to override.
///
pub trait ModelVisitor {
    /// The error which will be returned by this model.
    type Error;

    /// Called once for each key in the model's metadata.
    #[allow(unused_variables)]
    fn metadata(&self, key: &str, value: &Value) -> Result<(), Self::Error> {
        Ok(())
    }

    visit_fn! { simple_shape, Simple, "Called for each `ShapeKind::Simple` in this model's **shapes** collection." }
    visit_fn! { list, ListOrSet, "Called for each `ShapeKind::List` in this model's **shapes** collection." }
    visit_fn! { set, ListOrSet, "Called for each `ShapeKind::Set` in this model's **shapes** collection." }
    visit_fn! { map, Map, "Called for each `ShapeKind::Map` in this model's **shapes** collection." }
    visit_fn! { structure, StructureOrUnion, "Called for each `ShapeKind::Structure` in this model's **shapes** collection." }
    visit_fn! { union, StructureOrUnion, "Called for each `ShapeKind::Union` in this model's **shapes** collection." }
    visit_fn! { service, Service, "Called for each `ShapeKind::Service` in this model's **shapes** collection." }
    visit_fn! { operation, Operation, "Called for each `ShapeKind::Operation` in this model's **shapes** collection." }
    visit_fn! { resource, Resource, "Called for each `ShapeKind::Resource` in this model's **shapes** collection." }
    visit_fn! { unresolved_id, "Called for each `ShapeKind::Unresolved` shape identifier in this model's **shapes** collection." }
}

///
/// Identical to [ModelVisitor](trait.ModelVisitor.html), but `self` is a mutable reference.
///
pub trait MutableModelVisitor {
    /// The error which will be returned by this model.
    type Error;

    /// Called once for each key in the model's metadata.
    #[allow(unused_variables)]
    fn metadata(&self, key: &str, value: &Value) -> Result<(), Self::Error> {
        Ok(())
    }

    visit_fn! { mut simple_shape, Simple, "Called for each `ShapeKind::Simple` in this model's **shapes** collection." }
    visit_fn! { mut list, ListOrSet, "Called for each `ShapeKind::List` in this model's **shapes** collection." }
    visit_fn! { mut set, ListOrSet, "Called for each `ShapeKind::Set` in this model's **shapes** collection." }
    visit_fn! { mut map, Map, "Called for each `ShapeKind::Map` in this model's **shapes** collection." }
    visit_fn! { mut structure, StructureOrUnion, "Called for each `ShapeKind::Structure` in this model's **shapes** collection." }
    visit_fn! { mut union, StructureOrUnion, "Called for each `ShapeKind::Union` in this model's **shapes** collection." }
    visit_fn! { mut service, Service, "Called for each `ShapeKind::Service` in this model's **shapes** collection." }
    visit_fn! { mut operation, Operation, "Called for each `ShapeKind::Operation` in this model's **shapes** collection." }
    visit_fn! { mut resource, Resource, "Called for each `ShapeKind::Resource` in this model's **shapes** collection." }
    visit_fn! { mut unresolved_id, "Called for each `ShapeKind::Unresolved` shape identifier in this model's **shapes** collection." }
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Walk the provided model calling out to the visitor as necessary. This is a useful tool for use
/// cases where you do not need to cross-validate model elements but can process the model shape by
/// shape independently.
///
pub fn walk_model<V>(model: &Model, visitor: &V) -> Result<(), V::Error>
where
    V: ModelVisitor,
{
    for (key, value) in model.metadata() {
        visitor.metadata(key, value)?;
    }

    for shape in model.shapes() {
        match &shape.body() {
            ShapeKind::Simple(body) => visitor.simple_shape(shape.id(), &shape.traits(), &body)?,
            ShapeKind::List(body) => visitor.list(shape.id(), &shape.traits(), &body)?,
            ShapeKind::Set(body) => visitor.set(shape.id(), &shape.traits(), &body)?,
            ShapeKind::Map(body) => visitor.map(shape.id(), &shape.traits(), &body)?,
            ShapeKind::Structure(body) => visitor.structure(shape.id(), &shape.traits(), &body)?,
            ShapeKind::Union(body) => visitor.union(shape.id(), &shape.traits(), &body)?,
            ShapeKind::Service(body) => visitor.service(shape.id(), &shape.traits(), &body)?,
            ShapeKind::Operation(body) => visitor.operation(shape.id(), &shape.traits(), &body)?,
            ShapeKind::Resource(body) => visitor.resource(shape.id(), &shape.traits(), &body)?,
            ShapeKind::Unresolved => visitor.unresolved_id(shape.id(), &shape.traits())?,
        }
    }

    Ok(())
}

///
/// Identical to [walk_model](fn.walk_model.html), but `visitor` is a mutable reference.
///
pub fn walk_model_mut<V>(model: &Model, visitor: &mut V) -> Result<(), V::Error>
where
    V: MutableModelVisitor,
{
    for (key, value) in model.metadata() {
        visitor.metadata(key, value)?;
    }

    for shape in model.shapes() {
        match &shape.body() {
            ShapeKind::Simple(body) => visitor.simple_shape(shape.id(), &shape.traits(), &body)?,
            ShapeKind::List(body) => visitor.list(shape.id(), &shape.traits(), &body)?,
            ShapeKind::Set(body) => visitor.set(shape.id(), &shape.traits(), &body)?,
            ShapeKind::Map(body) => visitor.map(shape.id(), &shape.traits(), &body)?,
            ShapeKind::Structure(body) => visitor.structure(shape.id(), &shape.traits(), &body)?,
            ShapeKind::Union(body) => visitor.union(shape.id(), &shape.traits(), &body)?,
            ShapeKind::Service(body) => visitor.service(shape.id(), &shape.traits(), &body)?,
            ShapeKind::Operation(body) => visitor.operation(shape.id(), &shape.traits(), &body)?,
            ShapeKind::Resource(body) => visitor.resource(shape.id(), &shape.traits(), &body)?,
            ShapeKind::Unresolved => visitor.unresolved_id(shape.id(), &shape.traits())?,
        }
    }

    Ok(())
}
