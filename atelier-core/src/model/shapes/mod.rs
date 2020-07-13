/*!
Model structures common across all shape types.

The concept of a _shape_ in Smithy is abstract, the ABNF contains productions `shape_statements`
and `shape_body` but they are not concrete. Shapes are then classified as _simple_, _aggregate_,
and _service_. The model here introduces `Shape` as a common concrete structure which contains an
enumeration, `ShapeBody`, to represent each of the productions referenced by `shape_body`.

*/

use crate::model::{values::Value, Identifier, ShapeID};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This structure represents a shape within the model. The shape-specific data is within the
/// `ShapeBody` enumeration.
///
#[derive(Clone, Debug)]
pub struct Shape {
    id: ShapeID,
    traits: Vec<AppliedTrait>,
    body: ShapeKind,
}

///
/// This enumeration represents the set of shape types supported by Smithy.
///
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug)]
pub enum ShapeKind {
    /// An shape holding atomic, or primitive values.
    Simple(Simple),
    /// An ordered list of shapes.
    List(ListOrSet),
    /// An unordered set of shapes.
    Set(ListOrSet),
    /// A map of names to shapes.
    Map(Map),
    /// A structure consisting of pairs of shape ids; the name of the member and it's type.
    Structure(StructureOrUnion),
    /// A structure consisting of pairs of shape ids; the name of the member and it's type.
    Union(StructureOrUnion),
    /// A shape representing some deployed software service.
    Service(Service),
    /// A shape representing some resource managed by a software service, or a sub-resource of
    /// another resource.
    Operation(Operation),
    /// A shape representing an operation on a software service or resource.
    Resource(Resource),
    /// Represents a member shape, part of an aggregate or service shape. The `ShapeID` is the target
    /// type for this member.
    Member(Member),
    /// This represents a forward reference that has not yet been resolved to a defined shape. Any
    /// model that contains unresolved reference is considered to be `incomplete` and will result in
    /// validation errors.
    Unresolved,
}

///
/// A Trait applied to a shape or member including any value associated with the trait for this
/// instance.
///
#[derive(Clone, Debug, PartialEq)]
pub struct AppliedTrait {
    id: ShapeID,
    value: Option<Value>,
}

///
/// Members are the values within aggregate types.
///
#[derive(Clone, Debug, PartialEq)]
pub struct Member {
    name: Identifier,
    target: ShapeID,
}

// ------------------------------------------------------------------------------------------------
// Macros
// ------------------------------------------------------------------------------------------------

macro_rules! proxy_is {
    ($is_fn:ident) => {
        /// Determines the kind of this shape.
        pub fn $is_fn(&self) -> bool {
            self.body.$is_fn()
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl From<Simple> for ShapeKind {
    fn from(body: Simple) -> Self {
        Self::Simple(body)
    }
}

impl From<Service> for ShapeKind {
    fn from(body: Service) -> Self {
        Self::Service(body)
    }
}

impl From<Operation> for ShapeKind {
    fn from(body: Operation) -> Self {
        Self::Operation(body)
    }
}

impl From<Resource> for ShapeKind {
    fn from(body: Resource) -> Self {
        Self::Resource(body)
    }
}

impl From<Member> for ShapeKind {
    fn from(body: Member) -> Self {
        Self::Member(body)
    }
}

impl ShapeKind {
    is_as! { is_simple, Simple, as_simple, Simple }
    is_as! { is_list, List, as_list, ListOrSet }
    is_as! { is_set, Set, as_set, ListOrSet }
    is_as! { is_map, Map, as_map, Map}
    is_as! { is_structure, Structure, as_structure, StructureOrUnion}
    is_as! { is_union, Union, as_union, StructureOrUnion}
    is_as! { is_service, Service, as_service, Service }
    is_as! { is_operation, Operation, as_operation, Operation }
    is_as! { is_resource, Resource, as_resource, Resource }
    is_as! { is_member, Member, as_member, Member }
    is_as! { is_unresolved, Unresolved }
}

// ------------------------------------------------------------------------------------------------

impl Shape {
    ///
    /// Construct a new shape with the given identifier (shape name) and shape-specific data.
    ///
    pub fn new(id: ShapeID, body: ShapeKind) -> Self {
        Self {
            id,
            traits: Default::default(),
            body,
        }
    }

    ///
    /// Construct a new shape with the given identifier (shape name) and shape-specific data.
    ///
    pub fn with_traits(id: ShapeID, body: ShapeKind, traits: &[AppliedTrait]) -> Self {
        Self {
            id,
            traits: traits.to_vec(),
            body,
        }
    }

    // --------------------------------------------------------------------------------------------

    ///
    /// Construct a new shape with a `ShapeBody::List` body.
    ///
    pub fn new_simple(&self, shape_name: Identifier, a_type: Simple) -> Self {
        Self::new(self.id.to_shape(shape_name), ShapeKind::Simple(a_type))
    }

    ///
    /// Construct a new shape with a `ShapeBody::List` body.
    ///
    pub fn list(&self, shape_name: Identifier, member: ShapeID) -> Self {
        Self::new(
            self.id.to_shape(shape_name),
            ShapeKind::List(ListOrSet::new_list(member)),
        )
    }

    ///
    /// Construct a new shape with a `ShapeBody::Set` body.
    ///
    pub fn set(&self, shape_name: Identifier, member: ShapeID) -> Self {
        Self::new(
            self.id.to_shape(shape_name),
            ShapeKind::Set(ListOrSet::new_set(member)),
        )
    }

    ///
    /// Construct a new shape with a `ShapeBody::Set` body.
    ///
    pub fn map(&self, shape_name: Identifier, key: ShapeID, value: ShapeID) -> Self {
        Self::new(
            self.id.to_shape(shape_name),
            ShapeKind::Map(Map::new(key, value)),
        )
    }

    ///
    /// Construct a new shape with a `ShapeBody::Structure` body.
    ///
    /// Note: that all members must have a body variant `ShapeBody::Member`, otherwise this method
    /// will panic.
    ///
    pub fn structure(&self, shape_name: Identifier, members: &[Shape]) -> Self {
        assert!(members.iter().all(|shape| shape.is_member()));
        Self::new(
            self.id.to_shape(shape_name),
            ShapeKind::Structure(StructureOrUnion::with_members(members)),
        )
    }

    ///
    /// Construct a new shape with a `ShapeBody::Structure` body.
    ///
    pub fn union(&self, shape_name: Identifier, members: &[Shape]) -> Self {
        assert!(members.iter().all(|shape| shape.is_member()));
        Self::new(
            self.id.to_shape(shape_name),
            ShapeKind::Union(StructureOrUnion::with_members(members)),
        )
    }

    ///
    /// Construct a new shape with a `ShapeBody::Member` body.
    ///
    pub fn member(&self, member_name: Identifier, refers_to: ShapeID) -> Self {
        Self::new(
            self.id.to_member(member_name.clone()),
            ShapeKind::Member(Member::new(member_name, refers_to)),
        )
    }

    // --------------------------------------------------------------------------------------------

    /// The absolute ShapeID of this shape.
    pub fn id(&self) -> &ShapeID {
        &self.id
    }

    /// Set the absolute ShapeID of this shape.
    pub fn set_id(&mut self, id: ShapeID) {
        assert!(id.is_absolute());
        self.id = id
    }

    // --------------------------------------------------------------------------------------------

    /// Returns `true` if the model element has any applied traits, else `false`.
    pub fn has_traits(&self) -> bool {
        !self.traits.is_empty()
    }

    /// Returns `true` if the model element has any applied traits with the associated id, else `false`.
    pub fn has_trait(&self, id: &ShapeID) -> bool {
        self.traits.iter().any(|t| t.id() == id)
    }

    /// Return an iterator over all traits applied to this model element
    pub fn traits(&self) -> &Vec<AppliedTrait> {
        &self.traits
    }

    /// Apply a trait to this model element.
    pub fn apply_trait(&mut self, a_trait: AppliedTrait) {
        // TODO: apply trait duplicate rules.
        self.traits.push(a_trait);
    }

    /// Add all the traits to this model element.
    pub fn remove_trait(&mut self, id: &ShapeID) {
        self.traits.retain(|t| t.id() != id);
    }

    // --------------------------------------------------------------------------------------------

    ///
    /// Return a reference to the shape-specific data within the shape.
    ///
    pub fn body(&self) -> &ShapeKind {
        &self.body
    }

    ///
    /// Return a mutable reference to the shape-specific data within the shape.
    ///
    pub fn body_mut(&mut self) -> &mut ShapeKind {
        &mut self.body
    }

    ///
    /// Set the shape-specific data for this shape.
    ///
    pub fn set_body(&mut self, body: ShapeKind) {
        self.body = body
    }

    // --------------------------------------------------------------------------------------------

    proxy_is! { is_simple }
    proxy_is! { is_list }
    proxy_is! { is_set }
    proxy_is! { is_map }
    proxy_is! { is_structure }
    proxy_is! { is_union }
    proxy_is! { is_service }
    proxy_is! { is_operation }
    proxy_is! { is_resource }
    proxy_is! { is_member }
    proxy_is! { is_unresolved }
}

// ------------------------------------------------------------------------------------------------

impl AppliedTrait {
    /// Construct a new trait with the given identity but no value.
    pub fn new(id: ShapeID) -> Self {
        Self { id, value: None }
    }

    /// Construct a new trait with the given identity and a value.
    pub fn with_value(id: ShapeID, value: Value) -> Self {
        Self {
            id,
            value: Some(value),
        }
    }

    /// Returns the identifier of the shape that this trait refers to.
    pub fn id(&self) -> &ShapeID {
        &self.id
    }

    /// Returns `true` if this applied trait has an associated value.
    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    /// Return a reference to the current value, if set.
    pub fn value(&self) -> &Option<Value> {
        &self.value
    }

    /// Return a mutable reference to the current value, if set.
    pub fn value_mut(&mut self) -> &mut Option<Value> {
        &mut self.value
    }

    /// Set the current value.
    pub fn set_value(&mut self, value: Value) {
        self.value = Some(value);
    }

    /// Set the current value to None.
    pub fn unset_value(&mut self) {
        self.value = None;
    }
}

// ------------------------------------------------------------------------------------------------

impl Member {
    /// Construct a new Member shape with the given name and target shape (type).
    pub fn new(name: Identifier, target: ShapeID) -> Self {
        Self { name, target }
    }

    /// The name of this member.
    pub fn name(&self) -> &Identifier {
        &self.name
    }

    required_member! { target, ShapeID, set_target }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
pub mod simple;
pub use simple::Simple;

#[doc(hidden)]
pub mod aggregate;
pub use aggregate::{ListOrSet, Map, StructureOrUnion};

#[doc(hidden)]
pub mod service;
pub use service::{Operation, Resource, Service};
