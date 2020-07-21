/*!
Builders to construct models in a more fluent style. See the example in the
[library overview](../../index.html#builder-api-example).

*/

use crate::error::ErrorKind;
use crate::model::shapes::{
    AppliedTrait, ListOrSet, Map, MemberShape, Operation, Resource, Service, Shape, ShapeKind,
    StructureOrUnion, TopLevelShape,
};
use crate::model::values::{Value, ValueMap};
use crate::model::{Model, NamespaceID, ShapeID};
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
    default_namespace: NamespaceID,
    smithy_version: Version,
    metadata: ValueMap,
    shapes: Vec<TopLevelShape>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

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
    /// Construct a new model builder using the provided Smithy version and a default namespace.
    pub fn new(smithy_version: Version, default_namespace: &str) -> Self {
        Self {
            default_namespace: NamespaceID::from_str(default_namespace).unwrap(),
            smithy_version,
            metadata: Default::default(),
            shapes: Default::default(),
        }
    }

    /// Set the default namespace to apply to added shapes. Not that this can be result during use
    /// to allow the creation of models that have different namespaced shapes.
    pub fn default_namespace(&mut self, namespace: &str) -> &mut Self {
        self.default_namespace = NamespaceID::from_str(namespace).unwrap();
        self
    }

    /// Add the given shape to the model.
    pub fn shape(&mut self, shape: TopLevelShape) -> &mut Self {
        if shape.id().is_member() {
            panic!("{}", ErrorKind::ShapeIDExpected(shape.id().clone()));
        } else {
            let _ = self.shapes.push(shape);
        }
        self
    }

    /// Create a new shape name using the default namespace
    pub fn shape_name(&self, shape_name: &str) -> ShapeID {
        self.default_namespace
            .make_shape(shape_name.parse().unwrap())
    }

    /// Create and add a new simple shape to this model using the `SimpleShapeBuilder` instance.
    pub fn simple_shape(&mut self, builder: &mut SimpleShapeBuilder) -> &mut Self {
        let shape_name = self
            .default_namespace
            .make_shape(builder.shape_name.clone());
        self.shape(TopLevelShape::with_traits(
            shape_name,
            ShapeKind::Simple(builder.simple_shape.clone()),
            builder.applied_traits.as_ref(),
        ))
    }

    /// Create and add a new list shape to this model using the `ListBuilder` instance.
    pub fn list(&mut self, builder: &mut ListBuilder) -> &mut Self {
        let shape_name = self
            .default_namespace
            .make_shape(builder.shape_name.clone());
        self.shape(TopLevelShape::with_traits(
            shape_name.clone(),
            ShapeKind::List(ListOrSet::from(MemberShape::with_traits(
                shape_name.make_member(builder.member.member_name.clone()),
                builder.member.target.clone(),
                &builder.member.applied_traits,
            ))),
            &builder.applied_traits,
        ))
    }

    /// Create and add a new set shape to this model using the `ListBuilder` instance.
    pub fn set(&mut self, builder: &mut ListBuilder) -> &mut Self {
        let shape_name = self
            .default_namespace
            .make_shape(builder.shape_name.clone());
        self.shape(TopLevelShape::with_traits(
            shape_name.clone(),
            ShapeKind::List(ListOrSet::from(MemberShape::with_traits(
                shape_name.make_member(builder.member.member_name.clone()),
                builder.member.target.clone(),
                &builder.member.applied_traits,
            ))),
            &builder.applied_traits,
        ))
    }

    /// Create and add a new map shape to this model using the `MapBuilder` instance.
    pub fn map(&mut self, builder: &mut MapBuilder) -> &mut Self {
        let shape_name = self
            .default_namespace
            .make_shape(builder.shape_name.clone());
        self.shape(TopLevelShape::with_traits(
            shape_name.clone(),
            ShapeKind::Map(Map::from(
                MemberShape::with_traits(
                    shape_name.make_member(builder.key.member_name.clone()),
                    builder.key.target.clone(),
                    &builder.key.applied_traits,
                ),
                MemberShape::with_traits(
                    shape_name.make_member(builder.value.member_name.clone()),
                    builder.value.target.clone(),
                    &builder.value.applied_traits,
                ),
            )),
            &builder.applied_traits,
        ))
    }

    /// Create and add a new structure shape to this model using the `StructureBuilder` instance.
    pub fn structure(&mut self, builder: &mut StructureBuilder) -> &mut Self {
        let shape_name = self
            .default_namespace
            .make_shape(builder.shape_name.clone());
        let members: Vec<MemberShape> = builder
            .members
            .iter()
            .map(|mb| {
                MemberShape::with_traits(
                    shape_name.make_member(mb.member_name.clone()),
                    mb.target.clone(),
                    &mb.applied_traits,
                )
            })
            .collect();
        let structure = StructureOrUnion::with_members(&members);
        self.shape(TopLevelShape::with_traits(
            shape_name,
            ShapeKind::Structure(structure),
            &builder.applied_traits,
        ))
    }

    /// Create and add a new union shape to this model using the `StructureBuilder` instance.
    pub fn union(&mut self, builder: &mut StructureBuilder) -> &mut Self {
        let shape_name = self
            .default_namespace
            .make_shape(builder.shape_name.clone());
        let members: Vec<MemberShape> = builder
            .members
            .iter()
            .map(|mb| {
                MemberShape::with_traits(
                    shape_name.make_member(mb.member_name.clone()),
                    mb.target.clone(),
                    &mb.applied_traits,
                )
            })
            .collect();
        let structure = StructureOrUnion::with_members(&members);
        self.shape(TopLevelShape::with_traits(
            shape_name,
            ShapeKind::Union(structure),
            &builder.applied_traits,
        ))
    }

    /// Create and add a new service shape to this model using the `ServiceBuilder` instance.
    pub fn service(&mut self, builder: &mut ServiceBuilder) -> &mut Self {
        let shape_name = self
            .default_namespace
            .make_shape(builder.shape_name.clone());
        let mut service = Service::new(&builder.version);
        for shape_id in builder.operations.drain(..) {
            service.add_operation(shape_id);
        }
        for shape_id in builder.resources.drain(..) {
            service.add_resource(shape_id);
        }
        self.shape(TopLevelShape::with_traits(
            shape_name,
            ShapeKind::Service(service),
            &builder.applied_traits,
        ))
    }

    /// Create and add a new operation shape to this model using the `OperationBuilder` instance.
    pub fn operation(&mut self, builder: &mut OperationBuilder) -> &mut Self {
        let shape_name = self
            .default_namespace
            .make_shape(builder.shape_name.clone());
        let mut operation = Operation::default();
        if let Some(shape_id) = &builder.input {
            operation.set_input(shape_id.clone());
        }
        if let Some(shape_id) = &builder.input {
            operation.set_output(shape_id.clone());
        }
        for shape_id in builder.errors.drain(..) {
            operation.add_error(shape_id);
        }
        self.shape(TopLevelShape::with_traits(
            shape_name,
            ShapeKind::Operation(operation),
            &builder.applied_traits,
        ))
    }

    /// Create and add a new resource shape to this model using the `ResourceBuilder` instance.
    pub fn resource(&mut self, builder: &mut ResourceBuilder) -> &mut Self {
        let shape_name = self
            .default_namespace
            .make_shape(builder.shape_name.clone());
        let mut resource = Resource::default();
        if let Some(shape_id) = &builder.create {
            resource.set_create(shape_id.clone());
        }
        if let Some(shape_id) = &builder.put {
            resource.set_put(shape_id.clone());
        }
        if let Some(shape_id) = &builder.read {
            resource.set_read(shape_id.clone());
        }
        if let Some(shape_id) = &builder.update {
            resource.set_update(shape_id.clone());
        }
        if let Some(shape_id) = &builder.delete {
            resource.set_delete(shape_id.clone());
        }
        if let Some(shape_id) = &builder.list {
            resource.set_list(shape_id.clone());
        }
        for shape_id in builder.operations.drain(..) {
            resource.add_operation(shape_id);
        }
        for shape_id in builder.collection_operations.drain(..) {
            resource.add_collection_operation(shape_id);
        }
        for shape_id in builder.resources.drain(..) {
            resource.add_resource(shape_id);
        }
        self.shape(TopLevelShape::with_traits(
            shape_name,
            ShapeKind::Resource(resource),
            &builder.applied_traits,
        ))
    }

    /// Short-cut method, this creates a new `ShapeKind::Unresolved` in the model.
    pub fn uses(&mut self, shape: &str) -> &mut Self {
        let shape = TopLevelShape::new(ShapeID::from_str(shape).unwrap(), ShapeKind::Unresolved);
        self.shape(shape)
    }

    /// Short-cut method, this creates a new `ShapeKind::Unresolved`, with a trait, in the model.
    pub fn apply(&mut self, shape: &str, a_trait: AppliedTrait) -> &mut Self {
        let mut shape =
            TopLevelShape::new(ShapeID::from_str(shape).unwrap(), ShapeKind::Unresolved);
        shape.apply_trait(a_trait);
        self.shape(shape)
    }

    /// Set a metadata value.
    pub fn meta_data(&mut self, key: String, value: Value) -> &mut Self {
        let _ = self.metadata.insert(key, value);
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
    SimpleShapeBuilder, StructureBuilder,
};

#[doc(hidden)]
pub mod traits;
pub use traits::TraitBuilder;

#[doc(hidden)]
pub mod values;
pub use values::{ArrayBuilder, ObjectBuilder, ValueBuilder};
