/*!
Model structures for shapes.

*/

use crate::model::{Annotated, Identifier, Named, ShapeID};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Shape {
    id: Identifier,
    traits: Vec<Trait>,
    inner: ShapeInner,
}

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug)]
pub enum ShapeInner {
    SimpleType(SimpleType),
    List(ListOrSet),
    Set(ListOrSet),
    Map(Map),
    Structure(StructureOrUnion),
    Union(StructureOrUnion),
    Service(Service),
    Operation(Operation),
    Resource(Resource),
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Named<Identifier> for Shape {
    fn id(&self) -> &Identifier {
        &self.id
    }
}

impl Annotated for Shape {
    fn has_traits(&self) -> bool {
        !self.traits.is_empty()
    }

    fn has_trait(&self, id: &ShapeID) -> bool {
        self.traits.iter().any(|t| t.id() == id)
    }

    fn traits(&self) -> &Vec<Trait> {
        &self.traits
    }

    fn add_trait(&mut self, a_trait: Trait) {
        self.traits.push(a_trait);
    }

    fn remove_trait(&mut self, id: &ShapeID) {
        self.traits.retain(|t| t.id() != id);
    }
}

impl Shape {
    pub fn new(id: Identifier, inner: ShapeInner) -> Self {
        Self {
            id,
            traits: Default::default(),
            inner,
        }
    }

    pub fn inner(&self) -> &ShapeInner {
        &self.inner
    }

    pub(crate) fn inner_mut(&mut self) -> &mut ShapeInner {
        &mut self.inner
    }

    pub fn set_inner(&mut self, inner: ShapeInner) {
        self.inner = inner
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
pub mod members;
pub use members::{Member, Trait, Valued};

#[doc(hidden)]
pub mod services;
pub use services::{Operation, Resource, Service};

#[doc(hidden)]
pub mod types;
pub use types::{ListOrSet, Map, SimpleType, StructureOrUnion};
