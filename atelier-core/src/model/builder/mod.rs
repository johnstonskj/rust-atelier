/*!
Builders to construct models in a more fluent style.

*/

use crate::model::{Model, Namespace, Shape, ShapeID};
use crate::Version;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct ModelBuilder {
    model: Model,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl ModelBuilder {
    pub fn new(namespace: &str) -> Self {
        Self {
            model: Model {
                version: Default::default(),
                namespace: Namespace::from_str(namespace).unwrap(),
                uses: vec![],
                shapes: Default::default(),
                applies: vec![],
                metadata: vec![],
            },
        }
    }

    pub fn version(&mut self, version: Version) -> &mut Self {
        self.model.version = version;
        self
    }

    pub fn uses(&mut self, shape: &str) -> &mut Self {
        self.model.add_usage(ShapeID::from_str(shape).unwrap());
        self
    }

    pub fn shape(&mut self, shape: Shape) -> &mut Self {
        self.model.add_shape(shape);
        self
    }

    pub fn build(&self) -> Model {
        self.model.clone()
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
pub mod shapes;
pub use shapes::{
    Builder, ListBuilder, MapBuilder, MemberBuilder, OperationBuilder, ResourceBuilder,
    ServiceBuilder, SetBuilder, SimpleShapeBuilder, StructureBuilder, UnionBuilder,
};

#[doc(hidden)]
pub mod traits;
pub use traits::TraitBuilder;

#[doc(hidden)]
pub mod values;
pub use values::{ArrayBuilder, ObjectBuilder, ValueBuilder};
