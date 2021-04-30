/*!
Builders to construct models in a more fluent style. See the example in the
[library overview](../index.html#the-model-builder-api-example).

Typically the model is constructed by calling the shape methods (`simple_shape`, `list`, `map`,
`structure`, `service`, etc.) providing a corresponding builder instance that is cached in the
model builder. When the Model is constructed all of these builders are executed to construct the
corresponding shapes in the semantic model. This allows for name resolution to be done once all
members are added to the model.

Note that the builder API does not do any model consistency checking, other than 1) checking the
syntax of strings used to construct `Namespace`, `Identifier`, and `ShapeID` values, 2) ensuring
that all unqualified names can be resolved to absolute shape identifiers as required by the semantic
model. In these cases the model builder does not currently return `Result` values, but will panic.

For more information, see [the Rust Atelier book](https://rust-atelier.dev/using/builder_api.html).
*/

use crate::error::ErrorKind;
use crate::model::shapes::{
    ListOrSet, Map, MemberShape, Operation, Resource, Service, ShapeKind, StructureOrUnion,
    TopLevelShape,
};
use crate::model::values::{Value, ValueMap};
use crate::model::{Identifier, Model, NamespaceID, ShapeID};
use crate::prelude::PRELUDE_NAMESPACE;
use crate::Version;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Builder for a top-level `Model`. This implements `From<T>` to provide the model itself.
///
#[derive(Clone, Debug)]
pub struct ModelBuilder {
    make_references: bool,
    default_namespace: NamespaceID,
    prelude_namespace: NamespaceID,
    smithy_version: Version,
    metadata: ValueMap,
    shape_names: HashSet<Identifier>,
    shapes: Vec<TopLevelShapeBuilder>,
}

#[derive(Clone, Debug)]
enum TopLevelShapeBuilder {
    SimpleShape(SimpleShapeBuilder),
    List(ListBuilder),
    Set(ListBuilder),
    Map(MapBuilder),
    Structure(StructureBuilder),
    Union(StructureBuilder),
    Service(ServiceBuilder),
    Operation(OperationBuilder),
    Resource(ResourceBuilder),
    Reference(ReferenceBuilder),
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl From<ModelBuilder> for Model {
    fn from(builder: ModelBuilder) -> Self {
        let mut builder = builder;
        Self::from(&mut builder)
    }
}

impl From<&mut ModelBuilder> for Model {
    fn from(builder: &mut ModelBuilder) -> Self {
        let mut model = Model::new(builder.smithy_version);
        for (k, v) in builder.metadata.drain() {
            let _ = model.add_metadata(k, v);
        }
        for shape in &builder.shapes {
            let _ = model.add_shape(builder.make_shape(&shape));
        }
        model
    }
}

impl ModelBuilder {
    /// Construct a new model builder using the provided Smithy version and a default namespace.
    pub fn new(smithy_version: Version, default_namespace: &str) -> Self {
        Self {
            make_references: false,
            default_namespace: default_namespace.parse().unwrap(),
            prelude_namespace: PRELUDE_NAMESPACE.parse().unwrap(),
            smithy_version,
            metadata: Default::default(),
            shape_names: Default::default(),
            shapes: Default::default(),
        }
    }

    /// Create a new shape name using the default namespace
    pub fn shape_name(&self, shape_name: &str) -> ShapeID {
        self.default_namespace
            .make_shape(shape_name.parse().unwrap())
    }

    // --------------------------------------------------------------------------------------------

    fn push_shape_name(&mut self, id: &str) {
        if ShapeID::is_valid(id) {
            let id: ShapeID = id.parse().unwrap();
            if id.namespace() == &self.default_namespace {
                let _ = self.shape_names.insert(id.shape_name().clone());
            }
        } else if Identifier::is_valid(id) {
            let _ = self.shape_names.insert(id.parse().unwrap());
        } else {
            panic!()
        }
    }

    /// Create and add a new simple shape to this model using the `SimpleShapeBuilder` instance.
    pub fn simple_shape(&mut self, builder: SimpleShapeBuilder) -> &mut Self {
        self.push_shape_name(&builder.shape_name);
        self.shapes.push(TopLevelShapeBuilder::SimpleShape(builder));
        self
    }

    /// Create and add a new list shape to this model using the `ListBuilder` instance.
    pub fn list(&mut self, builder: ListBuilder) -> &mut Self {
        self.push_shape_name(&builder.shape_name);
        self.shapes.push(TopLevelShapeBuilder::List(builder));
        self
    }

    /// Create and add a new set shape to this model using the `ListBuilder` instance.
    pub fn set(&mut self, builder: ListBuilder) -> &mut Self {
        self.push_shape_name(&builder.shape_name);
        self.shapes.push(TopLevelShapeBuilder::Set(builder));
        self
    }

    /// Create and add a new map shape to this model using the `MapBuilder` instance.
    pub fn map(&mut self, builder: MapBuilder) -> &mut Self {
        self.push_shape_name(&builder.shape_name);
        self.shapes.push(TopLevelShapeBuilder::Map(builder));
        self
    }

    /// Create and add a new structure shape to this model using the `StructureBuilder` instance.
    pub fn structure(&mut self, builder: StructureBuilder) -> &mut Self {
        self.push_shape_name(&builder.shape_name);
        self.shapes.push(TopLevelShapeBuilder::Structure(builder));
        self
    }

    /// Create and add a new union shape to this model using the `StructureBuilder` instance.
    pub fn union(&mut self, builder: StructureBuilder) -> &mut Self {
        self.push_shape_name(&builder.shape_name);
        self.shapes.push(TopLevelShapeBuilder::Union(builder));
        self
    }

    /// Create and add a new service shape to this model using the `ServiceBuilder` instance.
    pub fn service(&mut self, builder: ServiceBuilder) -> &mut Self {
        self.push_shape_name(&builder.shape_name);
        self.shapes.push(TopLevelShapeBuilder::Service(builder));
        self
    }

    /// Create and add a new operation shape to this model using the `OperationBuilder` instance.
    pub fn operation(&mut self, builder: OperationBuilder) -> &mut Self {
        self.push_shape_name(&builder.shape_name);
        self.shapes.push(TopLevelShapeBuilder::Operation(builder));
        self
    }

    /// Create and add a new resource shape to this model using the `ResourceBuilder` instance.
    pub fn resource(&mut self, builder: ResourceBuilder) -> &mut Self {
        self.push_shape_name(&builder.shape_name);
        self.shapes.push(TopLevelShapeBuilder::Resource(builder));
        self
    }

    /// Short-cut method, this creates a new `ShapeKind::Unresolved` in the model.
    pub fn uses(&mut self, shape: &str) -> &mut Self {
        self.reference(ReferenceBuilder::new(shape))
    }

    /// Short-cut method, this creates a new `ShapeKind::Unresolved`, with a trait, in the model.
    pub fn apply(&mut self, shape: &str, a_trait: TraitBuilder) -> &mut Self {
        let mut builder = ReferenceBuilder::new(shape);
        let _ = builder.apply_trait(a_trait);
        self.reference(builder)
    }

    /// Create and add a new resource shape to this model using the `ResourceBuilder` instance.
    pub fn reference(&mut self, builder: ReferenceBuilder) -> &mut Self {
        self.shapes.push(TopLevelShapeBuilder::Reference(builder));
        self
    }

    /// Set a metadata value.
    pub fn meta_data(&mut self, key: String, value: Value) -> &mut Self {
        let _ = self.metadata.insert(key, value);
        self
    }

    /// Set a metadata value.
    pub fn meta_data_from(&mut self, value_map: ValueMap) -> &mut Self {
        for (key, value) in value_map {
            let _ = self.metadata.insert(key, value);
        }
        self
    }

    // --------------------------------------------------------------------------------------------

    fn resolve_shape_name(&self, name: &str) -> ShapeID {
        if ShapeID::is_valid(name) {
            let name: ShapeID = name.parse().unwrap();
            if !name.is_member() {
                name
            } else {
                panic!("{}", ErrorKind::ShapeIDExpected(name))
            }
        } else if Identifier::is_valid(name) {
            let shape_name: Identifier = name.parse().unwrap();
            if self.shape_names.contains(&shape_name) {
                self.default_namespace.make_shape(shape_name)
            } else if prelude::prelude_model_shape_ids(&self.smithy_version)
                .contains(&ShapeID::new_unchecked(PRELUDE_NAMESPACE, name, None))
            {
                self.prelude_namespace.make_shape(shape_name)
            } else {
                panic!("{:?}", ErrorKind::UnknownShape(name.to_string()))
            }
        } else {
            panic!("{:?}", ErrorKind::InvalidShapeID(name.to_string()))
        }
    }

    fn make_shape(&self, builder: &TopLevelShapeBuilder) -> TopLevelShape {
        match builder {
            TopLevelShapeBuilder::SimpleShape(builder) => self.make_simple_shape(builder),
            TopLevelShapeBuilder::List(builder) => self.make_list(builder),
            TopLevelShapeBuilder::Set(builder) => self.make_set(builder),
            TopLevelShapeBuilder::Map(builder) => self.make_map(builder),
            TopLevelShapeBuilder::Structure(builder) => self.make_structure(builder),
            TopLevelShapeBuilder::Union(builder) => self.make_union(builder),
            TopLevelShapeBuilder::Service(builder) => self.make_service(builder),
            TopLevelShapeBuilder::Operation(builder) => self.make_operation(builder),
            TopLevelShapeBuilder::Resource(builder) => self.make_resource(builder),
            TopLevelShapeBuilder::Reference(builder) => self.make_reference(builder),
        }
    }

    fn make_simple_shape(&self, builder: &SimpleShapeBuilder) -> TopLevelShape {
        let shape_name = self.resolve_shape_name(&builder.shape_name);
        TopLevelShape::with_traits(
            shape_name,
            ShapeKind::Simple(builder.simple_shape.clone()),
            self.make_traits(&builder.applied_traits),
        )
    }

    fn make_list(&self, builder: &ListBuilder) -> TopLevelShape {
        let shape_name = self.resolve_shape_name(&builder.shape_name);
        TopLevelShape::with_traits(
            shape_name.clone(),
            ShapeKind::List(ListOrSet::from(MemberShape::with_traits(
                shape_name.make_member(builder.member.member_name.parse().unwrap()),
                self.resolve_shape_name(&builder.member.target),
                self.make_traits(&builder.member.applied_traits),
            ))),
            self.make_traits(&builder.applied_traits),
        )
    }

    fn make_set(&self, builder: &ListBuilder) -> TopLevelShape {
        let shape_name = self.resolve_shape_name(&builder.shape_name);
        TopLevelShape::with_traits(
            shape_name.clone(),
            ShapeKind::List(ListOrSet::from(MemberShape::with_traits(
                shape_name.make_member(builder.member.member_name.parse().unwrap()),
                self.resolve_shape_name(&builder.member.target),
                self.make_traits(&builder.member.applied_traits),
            ))),
            self.make_traits(&builder.applied_traits),
        )
    }

    fn make_map(&self, builder: &MapBuilder) -> TopLevelShape {
        println!("{:#?}", builder);
        let shape_name = self.resolve_shape_name(&builder.shape_name);
        TopLevelShape::with_traits(
            shape_name.clone(),
            ShapeKind::Map(Map::from(
                MemberShape::with_traits(
                    shape_name.make_member(builder.key.member_name.parse().unwrap()),
                    self.resolve_shape_name(&builder.key.target),
                    self.make_traits(&builder.key.applied_traits),
                ),
                MemberShape::with_traits(
                    shape_name.make_member(builder.value.member_name.parse().unwrap()),
                    self.resolve_shape_name(&builder.value.target),
                    self.make_traits(&builder.value.applied_traits),
                ),
            )),
            self.make_traits(&builder.applied_traits),
        )
    }

    fn make_structure(&self, builder: &StructureBuilder) -> TopLevelShape {
        let shape_name = self.resolve_shape_name(&builder.shape_name);
        let members: Vec<MemberShape> = builder
            .members
            .iter()
            .map(|mb| {
                MemberShape::with_traits(
                    shape_name.make_member(mb.member_name.parse().unwrap()),
                    self.resolve_shape_name(&mb.target),
                    self.make_traits(&mb.applied_traits),
                )
            })
            .collect();
        let structure = StructureOrUnion::with_members(&members);
        TopLevelShape::with_traits(
            shape_name,
            ShapeKind::Structure(structure),
            self.make_traits(&builder.applied_traits),
        )
    }

    fn make_union(&self, builder: &StructureBuilder) -> TopLevelShape {
        let shape_name = self.resolve_shape_name(&builder.shape_name);
        let members: Vec<MemberShape> = builder
            .members
            .iter()
            .map(|mb| {
                MemberShape::with_traits(
                    shape_name.make_member(mb.member_name.parse().unwrap()),
                    self.resolve_shape_name(&mb.target),
                    self.make_traits(&mb.applied_traits),
                )
            })
            .collect();
        let structure = StructureOrUnion::with_members(&members);
        TopLevelShape::with_traits(
            shape_name,
            ShapeKind::Union(structure),
            self.make_traits(&builder.applied_traits),
        )
    }

    fn make_service(&self, builder: &ServiceBuilder) -> TopLevelShape {
        let shape_name = self.resolve_shape_name(&builder.shape_name);
        let mut service = Service::new(&builder.version);
        for shape_id in &builder.operations {
            service.add_operation(self.resolve_shape_name(shape_id));
        }
        for shape_id in &builder.resources {
            service.add_resource(self.resolve_shape_name(shape_id));
        }
        TopLevelShape::with_traits(
            shape_name,
            ShapeKind::Service(service),
            self.make_traits(&builder.applied_traits),
        )
    }

    fn make_operation(&self, builder: &OperationBuilder) -> TopLevelShape {
        let shape_name = self.resolve_shape_name(&builder.shape_name);
        let mut operation = Operation::default();
        if let Some(shape_id) = &builder.input {
            operation.set_input(self.resolve_shape_name(shape_id));
        }
        if let Some(shape_id) = &builder.input {
            operation.set_output(self.resolve_shape_name(shape_id));
        }
        for shape_id in &builder.errors {
            operation.add_error(self.resolve_shape_name(shape_id));
        }
        TopLevelShape::with_traits(
            shape_name,
            ShapeKind::Operation(operation),
            self.make_traits(&builder.applied_traits),
        )
    }

    fn make_resource(&self, builder: &ResourceBuilder) -> TopLevelShape {
        let shape_name = self.resolve_shape_name(&builder.shape_name);
        let mut resource = Resource::default();
        for (name, shape_ref) in &builder.identifiers {
            let shape = self.resolve_shape_name(&shape_ref);
            let _ = resource.add_identifier(Identifier::from_str(name).unwrap(), shape);
        }
        if let Some(shape_id) = &builder.create {
            resource.set_create(self.resolve_shape_name(shape_id));
        }
        if let Some(shape_id) = &builder.put {
            resource.set_put(self.resolve_shape_name(shape_id));
        }
        if let Some(shape_id) = &builder.read {
            resource.set_read(self.resolve_shape_name(shape_id));
        }
        if let Some(shape_id) = &builder.update {
            resource.set_update(self.resolve_shape_name(shape_id));
        }
        if let Some(shape_id) = &builder.delete {
            resource.set_delete(self.resolve_shape_name(shape_id));
        }
        if let Some(shape_id) = &builder.list {
            resource.set_list(self.resolve_shape_name(shape_id));
        }
        for shape_id in &builder.operations {
            resource.add_operation(self.resolve_shape_name(shape_id));
        }
        for shape_id in &builder.collection_operations {
            resource.add_collection_operation(self.resolve_shape_name(shape_id));
        }
        for shape_id in &builder.resources {
            resource.add_resource(self.resolve_shape_name(shape_id));
        }
        TopLevelShape::with_traits(
            shape_name,
            ShapeKind::Resource(resource),
            self.make_traits(&builder.applied_traits),
        )
    }

    fn make_reference(&self, builder: &ReferenceBuilder) -> TopLevelShape {
        let shape_id: ShapeID = builder.shape_id.parse().unwrap();

        TopLevelShape::with_traits(
            shape_id,
            ShapeKind::Unresolved,
            self.make_traits(&builder.applied_traits),
        )
    }

    fn make_traits(&self, builders: &[TraitBuilder]) -> HashMap<ShapeID, Option<Value>> {
        builders
            .iter()
            .cloned()
            .map(|builder| (self.resolve_shape_name(&builder.shape_id), builder.value))
            .collect()
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod prelude;

pub mod selector;

#[doc(hidden)]
pub mod shapes;
pub use shapes::{
    ListBuilder, MapBuilder, MemberBuilder, OperationBuilder, ReferenceBuilder, ResourceBuilder,
    ServiceBuilder, ShapeTraits, SimpleShapeBuilder, StructureBuilder,
};

#[doc(hidden)]
pub mod traits;
pub use traits::TraitBuilder;

#[doc(hidden)]
pub mod values;
pub use values::{ArrayBuilder, ObjectBuilder, ValueBuilder};
