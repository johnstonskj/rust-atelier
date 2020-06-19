/*!
Builders to construct models in a more fluent style.

*/

use crate::model::shapes::Shape;
use crate::model::{Model, Namespace, ShapeID};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

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
pub use shapes::Builder;

#[doc(hidden)]
pub mod traits;
use crate::Version;
pub use traits::TraitBuilder;
