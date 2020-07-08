/*!
One-line description.

More detailed description, with

# Example

*/

use crate::model::shapes::{
    ListOrSet, Map, Operation, Resource, Service, ShapeBody, SimpleType, StructureOrUnion, Trait,
};
use crate::model::values::{Key, NodeValue};
use crate::model::{Annotated, Model, Named, ShapeID};

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
    fn metadata(&self, key: &Key, value: &NodeValue) -> Result<(), Self::Error> {
        Ok(())
    }
    /// Called for each simple shape.
    #[allow(unused_variables)]
    fn simple_shape(
        &self,
        id: &ShapeID,
        traits: &[Trait],
        shape: &SimpleType,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
    /// Called for each list shape.
    #[allow(unused_variables)]
    fn list(&self, id: &ShapeID, traits: &[Trait], shape: &ListOrSet) -> Result<(), Self::Error> {
        Ok(())
    }
    /// Called for each set shape.
    #[allow(unused_variables)]
    fn set(&self, id: &ShapeID, traits: &[Trait], shape: &ListOrSet) -> Result<(), Self::Error> {
        Ok(())
    }
    /// Called for each map shape.
    #[allow(unused_variables)]
    fn map(&self, id: &ShapeID, traits: &[Trait], shape: &Map) -> Result<(), Self::Error> {
        Ok(())
    }
    /// Called for each structure shape.
    #[allow(unused_variables)]
    fn structure(
        &self,
        id: &ShapeID,
        traits: &[Trait],
        shape: &StructureOrUnion,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
    /// Called for each union shape.
    #[allow(unused_variables)]
    fn union(
        &self,
        id: &ShapeID,
        traits: &[Trait],
        shape: &StructureOrUnion,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
    /// Called for each service shape.
    #[allow(unused_variables)]
    fn service(&self, id: &ShapeID, traits: &[Trait], shape: &Service) -> Result<(), Self::Error> {
        Ok(())
    }
    /// Called for each operation shape.
    #[allow(unused_variables)]
    fn operation(
        &self,
        id: &ShapeID,
        traits: &[Trait],
        operation: &Operation,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
    /// Called for each resource shape.
    #[allow(unused_variables)]
    fn resource(
        &self,
        id: &ShapeID,
        traits: &[Trait],
        shape: &Resource,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
    /// Called for each apply statement.
    #[allow(unused_variables)]
    fn apply(&self, id: &ShapeID, traits: &[Trait]) -> Result<(), Self::Error> {
        Ok(())
    }
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
            ShapeBody::SimpleType(body) => {
                visitor.simple_shape(shape.id(), &shape.traits(), body)?
            }
            ShapeBody::List(body) => visitor.list(shape.id(), &shape.traits(), body)?,
            ShapeBody::Set(body) => visitor.set(shape.id(), &shape.traits(), body)?,
            ShapeBody::Map(body) => visitor.map(shape.id(), &shape.traits(), body)?,
            ShapeBody::Structure(body) => visitor.structure(shape.id(), &shape.traits(), body)?,
            ShapeBody::Union(body) => visitor.union(shape.id(), &shape.traits(), body)?,
            ShapeBody::Service(body) => visitor.service(shape.id(), &shape.traits(), body)?,
            ShapeBody::Operation(body) => visitor.operation(shape.id(), &shape.traits(), body)?,
            ShapeBody::Resource(body) => visitor.resource(shape.id(), &shape.traits(), body)?,
            ShapeBody::Apply => visitor.apply(shape.id(), &shape.traits())?,
        }
    }
    Ok(())
}
