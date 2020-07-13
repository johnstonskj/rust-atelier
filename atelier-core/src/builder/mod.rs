/*!
Builders to construct models in a more fluent style. See the example in the
[library overview](../../index.html#builder-api-example).

*/

use crate::model::{shapes::Shape, Model, Namespace};
use crate::Version;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Builder for a top-level `Model`. This implements `From<T>` to provide the model itself.
///
#[derive(Debug)]
pub struct ModelBuilder {
    default_namespace: Option<Namespace>,
    model: Model,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for ModelBuilder {
    fn default() -> Self {
        Self::new(Version::current())
    }
}

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
    /// Construct a new model builder using the provided Smithy version.
    pub fn new(version: Version) -> Self {
        Self {
            default_namespace: None,
            model: Model::new(version),
        }
    }

    /// Construct a new model builder using the provided Smithy version and namespace.
    pub fn with_namespace(version: Version, default_namespace: Namespace) -> Self {
        Self {
            default_namespace: Some(default_namespace),
            model: Model::new(version),
        }
    }

    /// Add the given shape to the model.
    pub fn shape(&mut self, shape: Shape) -> &mut Self {
        let _ = self.model.add_a_shape(shape);
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
    SetBuilder, SimpleShapeBuilder, StructureBuilder, UnionBuilder,
};

#[doc(hidden)]
pub mod traits;
pub use traits::TraitBuilder;

#[doc(hidden)]
pub mod values;
pub use values::{ArrayBuilder, ObjectBuilder, ValueBuilder};
