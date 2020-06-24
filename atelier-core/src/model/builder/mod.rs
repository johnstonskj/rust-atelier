/*!
Builders to construct models in a more fluent style. See the example in the
[library overview](../../index.html#builder-api-example).

*/

use crate::model::{shapes::Shape, Model, Namespace, ShapeID};
use crate::Version;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Builder for a top-level `Model`. This implements `From<T>` to provide the model itself.
///
#[derive(Debug)]
pub struct ModelBuilder {
    model: Model,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl From<&mut ModelBuilder> for Model {
    fn from(builder: &mut ModelBuilder) -> Self {
        builder.model.clone()
    }
}

impl From<ModelBuilder> for Model {
    fn from(builder: ModelBuilder) -> Self {
        builder.model
    }
}

impl ModelBuilder {
    /// Construct a new model builder for the given namespace.
    pub fn new(namespace: &str) -> Self {
        Self {
            model: Model::new(Namespace::from_str(namespace).unwrap()),
        }
    }

    /// Set the version of Smithy this model conforms to.
    pub fn version(&mut self, version: Version) -> &mut Self {
        self.model.set_version(version);
        self
    }

    /// Add a "uses" statement to add an external reference.
    pub fn uses(&mut self, shape: &str) -> &mut Self {
        self.model.add_reference(ShapeID::from_str(shape).unwrap());
        self
    }

    /// Add the given shape to the model.
    pub fn shape(&mut self, shape: Shape) -> &mut Self {
        self.model.add_shape(shape);
        self
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
pub mod shapes;
pub use shapes::{
    ListBuilder, MapBuilder, MemberBuilder, OperationBuilder, ResourceBuilder, ServiceBuilder,
    SetBuilder, ShapeBuilder, SimpleShapeBuilder, StructureBuilder, UnionBuilder,
};

#[doc(hidden)]
pub mod traits;
pub use traits::TraitBuilder;

#[doc(hidden)]
pub mod values;
pub use values::{ArrayBuilder, ObjectBuilder, ValueBuilder};
