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

use crate::builder::id::ShapeName;
use crate::error::{Error, ErrorKind};
use crate::model::shapes::{
    ListOrSet, Map, MemberShape, Operation, Resource, Service, ShapeKind, StructureOrUnion,
    TopLevelShape,
};
use crate::model::values::{Value, ValueMap};
use crate::model::{Identifier, Model, NamespaceID, ShapeID};
use crate::prelude::{
    defined_prelude_shapes, defined_prelude_traits, prelude_namespace_id, PRELUDE_NAMESPACE,
};
use crate::syntax::{MEMBER_KEY, MEMBER_MEMBER, MEMBER_VALUE};
use crate::Version;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::iter::FromIterator;
use std::str::FromStr;
pub use values::{ArrayBuilder, ObjectBuilder, ValueBuilder};

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
    shapes: HashMap<ShapeName, TopLevelShapeBuilder>,
}

#[allow(clippy::large_enum_variant)]
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

impl TryFrom<ModelBuilder> for Model {
    type Error = Error;

    fn try_from(builder: ModelBuilder) -> Result<Self, Self::Error> {
        let mut builder = builder;
        Self::try_from(&mut builder)
    }
}

impl TryFrom<&mut ModelBuilder> for Model {
    type Error = Error;

    fn try_from(builder: &mut ModelBuilder) -> Result<Self, Self::Error> {
        let mut model = Model::new(builder.smithy_version);
        for (k, v) in builder.metadata.iter() {
            let _ = model.add_metadata(k.clone(), v.clone());
        }
        let mut unresolved: HashSet<ShapeID> = Default::default();
        for shape in builder.shapes.values() {
            let _ = model.add_shape(builder.make_shape(&shape, &mut unresolved)?);
        }
        for shape_id in unresolved {
            let _ = model.add_shape(TopLevelShape::new(shape_id, ShapeKind::Unresolved));
        }
        Ok(model)
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
            shapes: Default::default(),
        }
    }

    /// Create a new shape name using the default namespace
    pub fn shape_name(&self, shape_name: &str) -> ShapeID {
        self.default_namespace
            .make_shape(shape_name.parse().unwrap())
    }

    // --------------------------------------------------------------------------------------------

    /// Create and add a new simple shape to this model using the `SimpleShapeBuilder` instance.
    pub fn simple_shape(&mut self, builder: SimpleShapeBuilder) -> &mut Self {
        self.insert(builder.shape_name.clone(), builder.into())
    }

    /// Create and add a new list shape to this model using the `ListBuilder` instance.
    pub fn list(&mut self, builder: ListBuilder) -> &mut Self {
        self.insert(
            builder.shape_name.clone(),
            TopLevelShapeBuilder::List(builder),
        )
    }

    /// Create and add a new set shape to this model using the `ListBuilder` instance.
    pub fn set(&mut self, builder: ListBuilder) -> &mut Self {
        self.insert(
            builder.shape_name.clone(),
            TopLevelShapeBuilder::Set(builder),
        )
    }

    /// Create and add a new map shape to this model using the `MapBuilder` instance.
    pub fn map(&mut self, builder: MapBuilder) -> &mut Self {
        self.insert(builder.shape_name.clone(), builder.into())
    }

    /// Create and add a new structure shape to this model using the `StructureBuilder` instance.
    pub fn structure(&mut self, builder: StructureBuilder) -> &mut Self {
        self.insert(
            builder.shape_name.clone(),
            TopLevelShapeBuilder::Structure(builder),
        )
    }

    /// Create and add a new union shape to this model using the `StructureBuilder` instance.
    pub fn union(&mut self, builder: StructureBuilder) -> &mut Self {
        self.insert(
            builder.shape_name.clone(),
            TopLevelShapeBuilder::Union(builder),
        )
    }

    /// Create and add a new service shape to this model using the `ServiceBuilder` instance.
    pub fn service(&mut self, builder: ServiceBuilder) -> &mut Self {
        self.insert(builder.shape_name.clone(), builder.into())
    }

    /// Create and add a new operation shape to this model using the `OperationBuilder` instance.
    pub fn operation(&mut self, builder: OperationBuilder) -> &mut Self {
        self.insert(builder.shape_name.clone(), builder.into())
    }

    /// Create and add a new resource shape to this model using the `ResourceBuilder` instance.
    pub fn resource(&mut self, builder: ResourceBuilder) -> &mut Self {
        self.insert(builder.shape_name.clone(), builder.into())
    }

    /// Short-cut method, this creates a new `ShapeKind::Unresolved` in the model.
    pub fn uses(&mut self, shape: &str) -> &mut Self {
        self.reference(ReferenceBuilder::new(shape))
    }

    /// Applies a trait to the shape named `shape`. IF the shape is not present in the model
    /// a new reference builder is created and the trait applied to it. This is similar to the
    /// way the `apply` statement works in the Smithy IDL.
    pub fn apply(&mut self, shape: &str, a_trait: TraitBuilder) -> &mut Self {
        let shape_name = ShapeName::from_str(shape).unwrap();
        self.apply_to(&shape_name, a_trait)
    }

    /// Create and add a new resource shape to this model using the `ResourceBuilder` instance.
    pub fn reference(&mut self, builder: ReferenceBuilder) -> &mut Self {
        let shape_id = &builder.shape_id;
        self.insert(shape_id.clone(), builder.into())
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

    fn insert(&mut self, id: ShapeName, shape: TopLevelShapeBuilder) -> &mut Self {
        let previous = self.shapes.insert(id, shape);
        if previous.is_some() {
            info!("Interestingly the shape seems to have been added more than once?",);
        }
        self
    }

    fn reference_names(&self) -> impl Iterator<Item = &ShapeName> {
        self.shapes
            .iter()
            .filter(|(_, builder)| matches!(builder, TopLevelShapeBuilder::Reference(_)))
            .map(|(id, _)| id)
    }

    ///
    /// From [Relative shape ID resolution](https://awslabs.github.io/smithy/1.0/spec/core/idl.html#relative-shape-id-resolution)
    ///
    /// Relative shape IDs are resolved using the following process:
    ///
    /// 1. If a `use_statement` has imported a shape with the same name, the shape ID resolves to
    ///    the imported shape ID.
    /// 2. If a shape is defined in the same namespace as the shape with the same name, the
    ///    namespace of the shape resolves to the *current namespace*.
    /// 3. If a shape is defined in the prelude with the same name, the namespace resolves to
    ///    `smithy.api`.
    /// 4. If a relative shape ID does not satisfy one of the above cases, the shape ID is invalid,
    ///    and the namespace is inherited from the *current namespace*.
    ///
    /// TODO: what about `Service::renames`?
    ///
    fn resolve_shape_name(
        &self,
        shape_name: &ShapeName,
        is_trait: bool,
    ) -> Result<(ShapeID, bool), Error> {
        info!(
            "resolve_shape_name(shape_name: {:?}, is_trait: {})",
            &shape_name, is_trait
        );
        match shape_name {
            ShapeName::Qualified(qualified) => {
                trace!("Qualified ShapeID exists, use as is");
                // If this is a ShapeID already, then just use it as-is.
                if !qualified.is_member() {
                    Ok((qualified.clone(), false))
                } else {
                    error!("expected a qualified, non-member, ShapeID");
                    Err(ErrorKind::ShapeIDExpected(qualified.clone()).into())
                }
            }
            ShapeName::Local(local) => {
                trace!("Local Identifier: proceeding with resolution...");
                let references: Vec<&ShapeName> = self
                    .reference_names()
                    .filter(|shape_name| shape_name.shape_name() == local)
                    .collect();
                debug!(
                    "Found {} references to check, references => {:#?}",
                    references.len(),
                    &references
                );
                match references.len() {
                    1 => {
                        // 1. a `use_statement` has imported a shape with the same name
                        trace!("SUCCESS: a use statement imports this shape explicitly");
                        Ok((
                            references.first().unwrap().as_qualified().unwrap().clone(),
                            false,
                        ))
                    }
                    0 => {
                        if self.shapes.contains_key(shape_name) {
                            // 2. a shape is defined in the same namespace as the shape with the same name
                            trace!(
                                "SUCCESS: shape found in same namespace as shape with same name"
                            );
                            Ok((self.default_namespace.make_shape(local.clone()), false))
                        } else if (!is_trait
                            && defined_prelude_shapes().contains(&*local.to_string()))
                            || (is_trait && defined_prelude_traits().contains(&*local.to_string()))
                        {
                            // 3. a shape is defined in the prelude with the same name
                            trace!("SUCCESS: shape found in prelude with the same name");
                            Ok((prelude_namespace_id().make_shape(local.clone()), false))
                        } else {
                            // 4. the shape ID is invalid
                            // => the namespace is inherited from the *current namespace*.
                            trace!("SUCCESS-ish: shape not found, so assume it's local");
                            let qualified = self.default_namespace.make_shape(local.clone());
                            Ok((qualified, true))
                        }
                    }
                    _ => {
                        error!("shape resolution failed because more than one reference matches");
                        Err(ErrorKind::AmbiguousShape(local.to_string()).into())
                    }
                }
            }
        }
    }

    fn apply_to(&mut self, shape_name: &ShapeName, a_trait: TraitBuilder) -> &mut Self {
        match shape_name {
            ShapeName::Qualified(shape_id) => {
                if let Some(member_name) = shape_id.member_name() {
                    let parent_shape = ShapeName::from(shape_id.shape_only());
                    if let Some(shape) = self.shapes.get_mut(&parent_shape) {
                        apply_to_member(shape, member_name, a_trait);
                    } else {
                        panic!("No shape named {} for member {}", parent_shape, member_name);
                    }
                } else if let Some(shape) = self.shapes.get_mut(&shape_name) {
                    apply_to_shape(shape, a_trait);
                } else {
                    let mut builder = ReferenceBuilder::from(shape_id.clone());
                    let _ = builder.apply_trait(a_trait);
                    let _ = self.reference(builder);
                }
                self
            }
            ShapeName::Local(local) => {
                // This is disallowed in the Smithy IDL which requires apply statements to take
                // qualified names.
                let (shape_id, new_reference) = self
                    .resolve_shape_name(&local.clone().into(), false)
                    .unwrap();
                if new_reference {
                    let _ = self.reference(ReferenceBuilder::from(shape_id.clone()));
                }
                self.apply_to(&shape_id.into(), a_trait)
            }
        }
    }

    fn make_shape(
        &self,
        builder: &TopLevelShapeBuilder,
        references: &mut HashSet<ShapeID>,
    ) -> Result<TopLevelShape, Error> {
        Ok(match builder {
            TopLevelShapeBuilder::SimpleShape(builder) => {
                self.make_simple_shape(builder, references)?
            }
            TopLevelShapeBuilder::List(builder) => self.make_list(builder, references)?,
            TopLevelShapeBuilder::Set(builder) => self.make_set(builder, references)?,
            TopLevelShapeBuilder::Map(builder) => self.make_map(builder, references)?,
            TopLevelShapeBuilder::Structure(builder) => self.make_structure(builder, references)?,
            TopLevelShapeBuilder::Union(builder) => self.make_union(builder, references)?,
            TopLevelShapeBuilder::Service(builder) => self.make_service(builder, references)?,
            TopLevelShapeBuilder::Operation(builder) => self.make_operation(builder, references)?,
            TopLevelShapeBuilder::Resource(builder) => self.make_resource(builder, references)?,
            TopLevelShapeBuilder::Reference(builder) => self.make_reference(builder, references)?,
        })
    }

    fn make_simple_shape(
        &self,
        builder: &SimpleShapeBuilder,
        references: &mut HashSet<ShapeID>,
    ) -> Result<TopLevelShape, Error> {
        let (shape_id, new_reference) = self.resolve_shape_name(&builder.shape_name, false)?;
        if new_reference {
            let _ = references.insert(shape_id.clone());
        }
        Ok(TopLevelShape::with_traits(
            shape_id,
            ShapeKind::Simple(builder.simple_shape.clone()),
            self.make_traits(&builder.applied_traits, references)?,
        ))
    }

    fn make_list(
        &self,
        builder: &ListBuilder,
        references: &mut HashSet<ShapeID>,
    ) -> Result<TopLevelShape, Error> {
        let (shape_id, new_reference) = self.resolve_shape_name(&builder.shape_name, false)?;
        if new_reference {
            let _ = references.insert(shape_id.clone());
        }
        let (target_id, new_reference) = self.resolve_shape_name(&builder.member.target, false)?;
        if new_reference {
            let _ = references.insert(target_id.clone());
        }
        Ok(TopLevelShape::with_traits(
            shape_id.clone(),
            ShapeKind::List(ListOrSet::from(MemberShape::with_traits(
                shape_id.make_member(builder.member.member_name.clone()),
                target_id,
                self.make_traits(&builder.member.applied_traits, references)?,
            ))),
            self.make_traits(&builder.applied_traits, references)?,
        ))
    }

    fn make_set(
        &self,
        builder: &ListBuilder,
        references: &mut HashSet<ShapeID>,
    ) -> Result<TopLevelShape, Error> {
        let (shape_id, new_reference) = self.resolve_shape_name(&builder.shape_name, false)?;
        if new_reference {
            let _ = references.insert(shape_id.clone());
        }
        let (target_id, new_reference) = self.resolve_shape_name(&builder.member.target, false)?;
        if new_reference {
            let _ = references.insert(target_id.clone());
        }
        Ok(TopLevelShape::with_traits(
            shape_id.clone(),
            ShapeKind::List(ListOrSet::from(MemberShape::with_traits(
                shape_id.make_member(builder.member.member_name.clone()),
                target_id,
                self.make_traits(&builder.member.applied_traits, references)?,
            ))),
            self.make_traits(&builder.applied_traits, references)?,
        ))
    }

    fn make_map(
        &self,
        builder: &MapBuilder,
        references: &mut HashSet<ShapeID>,
    ) -> Result<TopLevelShape, Error> {
        let (shape_id, new_reference) = self.resolve_shape_name(&builder.shape_name, false)?;
        if new_reference {
            let _ = references.insert(shape_id.clone());
        }
        let (key_target_id, new_reference) = self.resolve_shape_name(&builder.key.target, false)?;
        if new_reference {
            let _ = references.insert(key_target_id.clone());
        }
        let (value_target_id, new_reference) =
            self.resolve_shape_name(&builder.value.target, false)?;
        if new_reference {
            let _ = references.insert(value_target_id.clone());
        }
        Ok(TopLevelShape::with_traits(
            shape_id.clone(),
            ShapeKind::Map(Map::from(
                MemberShape::with_traits(
                    shape_id.make_member(builder.key.member_name.clone()),
                    key_target_id,
                    self.make_traits(&builder.key.applied_traits, references)?,
                ),
                MemberShape::with_traits(
                    shape_id.make_member(builder.value.member_name.clone()),
                    value_target_id,
                    self.make_traits(&builder.value.applied_traits, references)?,
                ),
            )),
            self.make_traits(&builder.applied_traits, references)?,
        ))
    }

    fn make_structure_inner(
        &self,
        shape_name: &ShapeID,
        builder: &StructureBuilder,
        references: &mut HashSet<ShapeID>,
    ) -> Result<StructureOrUnion, Error> {
        let members: Result<Vec<MemberShape>, Error> = builder
            .members
            .iter()
            .map(|mb| {
                let (member_target_id, new_reference) =
                    self.resolve_shape_name(&mb.target, false)?;
                if new_reference {
                    let _ = references.insert(member_target_id.clone());
                }
                Ok(MemberShape::with_traits(
                    shape_name.make_member(mb.member_name.clone()),
                    member_target_id,
                    self.make_traits(&mb.applied_traits, references)?,
                ))
            })
            .collect();
        members.map(|members| StructureOrUnion::with_members(&members))
    }

    fn make_structure(
        &self,
        builder: &StructureBuilder,
        references: &mut HashSet<ShapeID>,
    ) -> Result<TopLevelShape, Error> {
        let (shape_id, new_reference) = self.resolve_shape_name(&builder.shape_name, false)?;
        if new_reference {
            let _ = references.insert(shape_id.clone());
        }
        Ok(TopLevelShape::with_traits(
            shape_id.clone(),
            ShapeKind::Structure(self.make_structure_inner(&shape_id, builder, references)?),
            self.make_traits(&builder.applied_traits, references)?,
        ))
    }

    fn make_union(
        &self,
        builder: &StructureBuilder,
        references: &mut HashSet<ShapeID>,
    ) -> Result<TopLevelShape, Error> {
        let (shape_id, new_reference) = self.resolve_shape_name(&builder.shape_name, false)?;
        if new_reference {
            let _ = references.insert(shape_id.clone());
        }
        Ok(TopLevelShape::with_traits(
            shape_id.clone(),
            ShapeKind::Union(self.make_structure_inner(&shape_id, builder, references)?),
            self.make_traits(&builder.applied_traits, references)?,
        ))
    }

    fn make_service(
        &self,
        builder: &ServiceBuilder,
        references: &mut HashSet<ShapeID>,
    ) -> Result<TopLevelShape, Error> {
        let (shape_id, new_reference) = self.resolve_shape_name(&builder.shape_name, false)?;
        if new_reference {
            let _ = references.insert(shape_id.clone());
        }
        let mut service = Service::new(&builder.version);
        for member_shape_id in &builder.operations {
            let (shape_id, new_reference) = self.resolve_shape_name(&member_shape_id, false)?;
            if new_reference {
                let _ = references.insert(shape_id.clone());
            }
            service.add_operation(shape_id);
        }
        for member_shape_id in &builder.resources {
            let (shape_id, new_reference) = self.resolve_shape_name(&member_shape_id, false)?;
            if new_reference {
                let _ = references.insert(shape_id.clone());
            }
            service.add_resource(shape_id);
        }
        for (shape_id, local_name) in &builder.rename_shapes {
            let _ = service.insert_rename_shape(shape_id.clone(), local_name.clone());
        }
        Ok(TopLevelShape::with_traits(
            shape_id,
            ShapeKind::Service(service),
            self.make_traits(&builder.applied_traits, references)?,
        ))
    }

    fn make_operation(
        &self,
        builder: &OperationBuilder,
        references: &mut HashSet<ShapeID>,
    ) -> Result<TopLevelShape, Error> {
        let (shape_id, new_reference) = self.resolve_shape_name(&builder.shape_name, false)?;
        if new_reference {
            let _ = references.insert(shape_id.clone());
        }
        let mut operation = Operation::default();
        if let Some(shape_id) = &builder.input {
            let (shape_id, new_reference) = self.resolve_shape_name(&shape_id, false)?;
            if new_reference {
                let _ = references.insert(shape_id.clone());
            }
            operation.set_input(shape_id);
        }
        if let Some(shape_id) = &builder.output {
            let (shape_id, new_reference) = self.resolve_shape_name(&shape_id, false)?;
            if new_reference {
                let _ = references.insert(shape_id.clone());
            }
            operation.set_output(shape_id);
        }
        for shape_id in &builder.errors {
            let (shape_id, new_reference) = self.resolve_shape_name(&shape_id, false)?;
            if new_reference {
                let _ = references.insert(shape_id.clone());
            }
            operation.add_error(shape_id);
        }
        Ok(TopLevelShape::with_traits(
            shape_id,
            ShapeKind::Operation(operation),
            self.make_traits(&builder.applied_traits, references)?,
        ))
    }

    fn make_resource(
        &self,
        builder: &ResourceBuilder,
        references: &mut HashSet<ShapeID>,
    ) -> Result<TopLevelShape, Error> {
        let (shape_id, new_reference) = self.resolve_shape_name(&builder.shape_name, false)?;
        if new_reference {
            let _ = references.insert(shape_id.clone());
        }
        let mut resource = Resource::default();
        for (name, shape_ref) in &builder.identifiers {
            let (shape_id, new_reference) = self.resolve_shape_name(shape_ref, false)?;
            if new_reference {
                let _ = references.insert(shape_id.clone());
            }
            let _ = resource.add_identifier(name.clone(), shape_id);
        }
        if let Some(shape_id) = &builder.create {
            let (shape_id, new_reference) = self.resolve_shape_name(&shape_id, false)?;
            if new_reference {
                let _ = references.insert(shape_id.clone());
            }
            resource.set_create(shape_id);
        }
        if let Some(shape_id) = &builder.put {
            let (shape_id, new_reference) = self.resolve_shape_name(&shape_id, false)?;
            if new_reference {
                let _ = references.insert(shape_id.clone());
            }
            resource.set_put(shape_id);
        }
        if let Some(shape_id) = &builder.read {
            let (shape_id, new_reference) = self.resolve_shape_name(&shape_id, false)?;
            if new_reference {
                let _ = references.insert(shape_id.clone());
            }
            resource.set_read(shape_id);
        }
        if let Some(shape_id) = &builder.update {
            let (shape_id, new_reference) = self.resolve_shape_name(&shape_id, false)?;
            if new_reference {
                let _ = references.insert(shape_id.clone());
            }
            resource.set_update(shape_id);
        }
        if let Some(shape_id) = &builder.delete {
            let (shape_id, new_reference) = self.resolve_shape_name(&shape_id, false)?;
            if new_reference {
                let _ = references.insert(shape_id.clone());
            }
            resource.set_delete(shape_id);
        }
        if let Some(shape_id) = &builder.list {
            let (shape_id, new_reference) = self.resolve_shape_name(&shape_id, false)?;
            if new_reference {
                let _ = references.insert(shape_id.clone());
            }
            resource.set_list(shape_id);
        }
        for shape_id in &builder.operations {
            let (shape_id, new_reference) = self.resolve_shape_name(&shape_id, false)?;
            if new_reference {
                let _ = references.insert(shape_id.clone());
            }
            resource.add_operation(shape_id);
        }
        for shape_id in &builder.collection_operations {
            let (shape_id, new_reference) = self.resolve_shape_name(&shape_id, false)?;
            if new_reference {
                let _ = references.insert(shape_id.clone());
            }
            resource.add_collection_operation(shape_id);
        }
        for shape_id in &builder.resources {
            let (shape_id, new_reference) = self.resolve_shape_name(&shape_id, false)?;
            if new_reference {
                let _ = references.insert(shape_id.clone());
            }
            resource.add_resource(shape_id);
        }
        Ok(TopLevelShape::with_traits(
            shape_id,
            ShapeKind::Resource(resource),
            self.make_traits(&builder.applied_traits, references)?,
        ))
    }

    fn make_reference(
        &self,
        builder: &ReferenceBuilder,
        references: &mut HashSet<ShapeID>,
    ) -> Result<TopLevelShape, Error> {
        Ok(TopLevelShape::with_traits(
            builder.shape_id.as_qualified().unwrap().clone(),
            ShapeKind::Unresolved,
            self.make_traits(&builder.applied_traits, references)?,
        ))
    }

    fn make_traits(
        &self,
        builders: &[TraitBuilder],
        references: &mut HashSet<ShapeID>,
    ) -> Result<HashMap<ShapeID, Option<Value>>, Error> {
        let pairs: Result<Vec<(ShapeID, Option<Value>)>, Error> = builders
            .iter()
            .cloned()
            .map(|builder| {
                let (shape_id, new_reference) = self.resolve_shape_name(&builder.shape_id, true)?;
                if new_reference {
                    let _ = references.insert(shape_id.clone());
                }
                Ok((shape_id, builder.value))
            })
            .collect();
        match pairs {
            Ok(pairs) => Ok(HashMap::from_iter(pairs)),
            Err(err) => Err(err),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn apply_to_member(
    shape: &mut TopLevelShapeBuilder,
    member_name: &Identifier,
    a_trait: TraitBuilder,
) {
    let ok = match shape {
        TopLevelShapeBuilder::List(shape) => {
            if member_name.to_string() == MEMBER_MEMBER {
                let _ = shape.member.apply_trait(a_trait);
                true
            } else {
                false
            }
        }
        TopLevelShapeBuilder::Set(shape) => {
            if member_name.to_string() == MEMBER_MEMBER {
                let _ = shape.member.apply_trait(a_trait);
                true
            } else {
                false
            }
        }
        TopLevelShapeBuilder::Map(shape) => {
            if member_name.to_string() == MEMBER_KEY {
                let _ = shape.key.apply_trait(a_trait);
                true
            } else if member_name.to_string() == MEMBER_VALUE {
                let _ = shape.value.apply_trait(a_trait);
                true
            } else {
                false
            }
        }
        TopLevelShapeBuilder::Structure(shape) => {
            if let Some(member) = shape
                .members
                .iter_mut()
                .find(|m| &m.member_name == member_name)
            {
                let _ = member.apply_trait(a_trait);
                true
            } else {
                false
            }
        }
        TopLevelShapeBuilder::Union(shape) => {
            if let Some(member) = shape
                .members
                .iter_mut()
                .find(|m| &m.member_name == member_name)
            {
                let _ = member.apply_trait(a_trait);
                true
            } else {
                false
            }
        }
        _ => false,
    };
    if !ok {
        panic!("Shape does not have a traitable member {}", member_name);
    }
}

fn apply_to_shape(shape: &mut TopLevelShapeBuilder, a_trait: TraitBuilder) {
    match shape {
        TopLevelShapeBuilder::SimpleShape(shape) => {
            let _ = shape.apply_trait(a_trait);
        }
        TopLevelShapeBuilder::List(shape) => {
            let _ = shape.apply_trait(a_trait);
        }
        TopLevelShapeBuilder::Set(shape) => {
            let _ = shape.apply_trait(a_trait);
        }
        TopLevelShapeBuilder::Map(shape) => {
            let _ = shape.apply_trait(a_trait);
        }
        TopLevelShapeBuilder::Structure(shape) => {
            let _ = shape.apply_trait(a_trait);
        }
        TopLevelShapeBuilder::Union(shape) => {
            let _ = shape.apply_trait(a_trait);
        }
        TopLevelShapeBuilder::Service(shape) => {
            let _ = shape.apply_trait(a_trait);
        }
        TopLevelShapeBuilder::Operation(shape) => {
            let _ = shape.apply_trait(a_trait);
        }
        TopLevelShapeBuilder::Resource(shape) => {
            let _ = shape.apply_trait(a_trait);
        }
        TopLevelShapeBuilder::Reference(shape) => {
            let _ = shape.apply_trait(a_trait);
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
pub mod id;

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
