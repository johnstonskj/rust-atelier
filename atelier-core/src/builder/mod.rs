/*!
Builders to construct models in a more fluent style. See the example in the
[library overview](../../index.html#builder-api-example).

*/

use crate::error::{ErrorKind, Result as ModelResult};
use crate::model::shapes::{AppliedTrait, Shape, ShapeKind};
use crate::model::values::{Value, ValueMap};
use crate::model::{Identifier, Model, NamespaceID, ShapeID};
use crate::Version;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PartialShapeID {
    namespace: Option<NamespaceID>,
    shape_name: Option<Identifier>,
    member_name: Option<Identifier>,
}

///
/// Builder for a top-level `Model`. This implements `From<T>` to provide the model itself.
///
#[derive(Debug)]
pub struct ModelBuilder {
    default_namespace: Option<NamespaceID>,
    smithy_version: Version,
    metadata: ValueMap,
    shapes: Vec<Shape>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl PartialShapeID {
    pub fn shape(shape_name: &str) -> Self {
        Self {
            namespace: None,
            shape_name: Some(shape_name.parse().unwrap()),
            member_name: None,
        }
    }

    pub fn member(member_name: &str) -> Self {
        Self {
            namespace: None,
            shape_name: None,
            member_name: Some(member_name.parse().unwrap()),
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn set_namespace(&mut self, namespace: &str) -> &mut Self {
        self.namespace = Some(namespace.parse().unwrap());
        self
    }

    pub fn set_shape_name(&mut self, shape_name: &str) -> &mut Self {
        self.shape_name = Some(shape_name.parse().unwrap());
        self
    }

    pub fn set_member_name(&mut self, member_name: &str) -> &mut Self {
        self.member_name = Some(member_name.parse().unwrap());
        self
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_namespace(&self) -> bool {
        self.namespace.is_some()
    }

    pub fn has_shape_name(&self) -> bool {
        self.shape_name.is_some()
    }

    pub fn has_member_name(&self) -> bool {
        self.member_name.is_some()
    }

    pub fn is_absolute(&self) -> bool {
        self.has_namespace() && self.has_shape_name()
    }

    // --------------------------------------------------------------------------------------------

    pub fn to_shape_id(&self) -> ModelResult<ShapeID> {
        if self.namespace.is_none() && self.shape_name.is_none() {
            Err(ErrorKind::InvalidShapeID(format!(
                "{:?}#{:?}${:?}",
                self.namespace, self.shape_name, self.member_name
            ))
            .into())
        } else {
            Ok(ShapeID::new(
                self.namespace.unwrap(),
                self.shape_name.unwrap(),
                self.member_name.clone(),
            ))
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for ModelBuilder {
    fn default() -> Self {
        Self::new(Version::current())
    }
}

impl From<&mut ModelBuilder> for Model {
    fn from(builder: &mut ModelBuilder) -> Self {
        let mut model = Model::new(builder.smithy_version.clone());
        for (k, v) in builder.metadata.drain() {
            let _ = model.add_metadata(k, v);
        }
        for shape in builder.shapes.drain(..) {
            let _ = model.add_shape(shape);
        }
        model
    }
}

impl ModelBuilder {
    /// Construct a new model builder using the provided Smithy version.
    pub fn new(smithy_version: Version) -> Self {
        Self {
            default_namespace: None,
            smithy_version,
            metadata: Default::default(),
            shapes: Default::default(),
        }
    }

    /// Construct a new model builder using the provided Smithy version and a default namespace.
    pub fn with_namespace(smithy_version: Version, default_namespace: &str) -> Self {
        Self {
            default_namespace: Some(NamespaceID::from_str(default_namespace).unwrap()),
            smithy_version,
            metadata: Default::default(),
            shapes: Default::default(),
        }
    }

    /// Set the default namespace to apply to added shapes. Not that this can be result during use
    /// to allow the creation of models that have different namespaced shapes.
    pub fn default_namespace(&mut self, namespace: &str) -> &mut Self {
        self.default_namespace = Some(NamespaceID::from_str(namespace).unwrap());
        self
    }

    /// Add the given shape to the model.
    pub fn shape(&mut self, shape: Shape) -> &mut Self {
        if shape.id().is_member() {
            panic!("{}", ErrorKind::ShapeIDExpected(shape.id().clone()));
        } else {
            let _ = self.shapes.push(shape);
        }
        self
    }

    pub fn simple_shape(&mut self, builder: &mut SimpleShapeBuilder) -> &mut Self {
        let id = self.resolve(&builder.id);
        self.shape(Shape::with_traits(
            id,
            ShapeKind::Simple(builder.simple_shape.clone()),
            builder.applied_traits.as_ref(),
        ));
        self
    }

    pub fn list(&mut self, builder: &mut ListBuilder) -> &mut Self {}

    pub fn set(&mut self, builder: &mut SetBuilder) -> &mut Self {
        self
    }

    pub fn map(&mut self, builder: &mut MapBuilder) -> &mut Self {
        self
    }

    pub fn structure(&mut self, builder: &mut StructureBuilder) -> &mut Self {
        self
    }

    pub fn union(&mut self, builder: &mut UnionBuilder) -> &mut Self {
        self
    }

    pub fn service(&mut self, builder: &mut ServiceBuilder) -> &mut Self {
        self
    }

    pub fn operation(&mut self, builder: &mut OperationBuilder) -> &mut Self {
        self
    }

    pub fn resource(&mut self, builder: &mut ResourceBuilder) -> &mut Self {
        self
    }

    /// Short-cut method, this creates a new `ShapeKind::Unresolved` in the model.
    pub fn uses(&mut self, shape: &str) -> &mut Self {
        let shape = Shape::new(ShapeID::from_str(shape).unwrap(), ShapeKind::Unresolved);
        self.shape(shape)
    }

    /// Short-cut method, this creates a new `ShapeKind::Unresolved`, with a trait, in the model.
    pub fn apply(&mut self, shape: &str, a_trait: AppliedTrait) -> &mut Self {
        let mut shape = Shape::new(ShapeID::from_str(shape).unwrap(), ShapeKind::Unresolved);
        shape.apply_trait(a_trait);
        self.shape(shape)
    }

    /// Set a metadata value.
    pub fn meta_data(&mut self, key: String, value: Value) -> &mut Self {
        let _ = self.metadata.insert(key, value);
        self
    }

    fn resolve(&self, partial: &PartialShapeID) -> ShapeID {}
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
