/*!
Model structures common across all shape types.

The concept of a _shape_ in Smithy is abstract, the ABNF contains productions `shape_statements`
and `shape_body` but they are not concrete. Shapes are then classified as _simple_, _aggregate_,
and _service_. The model here introduces `Shape` as a common concrete structure which contains an
enumeration, `ShapeBody`, to represent each of the productions referenced by `shape_body`.

*/

use crate::model::{values::Value, ShapeID};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A common trait shared by `TopLevelShape` and `MemberShape`.
///
pub trait Shape {
    /// The absolute ShapeID of this shape.
    fn id(&self) -> &ShapeID;

    /// Set the absolute ShapeID of this shape.
    fn set_id(&mut self, id: ShapeID);

    /// Returns `true` if the model element has any applied traits, else `false`.
    fn has_traits(&self) -> bool;

    /// Returns `true` if the model element has any applied traits with the associated id, else `false`.
    fn has_trait(&self, id: &ShapeID) -> bool;

    /// Return an iterator over all traits applied to this model element
    fn traits(&self) -> &Vec<AppliedTrait>;

    /// Apply a trait to this model element.
    fn apply_trait(&mut self, a_trait: AppliedTrait);

    /// Add all these elements to this member's collection.
    fn append_traits(&mut self, traits: &[AppliedTrait]);

    /// Add all the traits to this model element.
    fn remove_trait(&mut self, id: &ShapeID);

    /// Is this instance a member (or top-level) shape?
    fn is_member(&self) -> bool;
}

///
/// This structure represents a top-level shape within a model. The shape-specific data is within the
/// `ShapeKind` enumeration. Aggregate shapes may have members of type `MemberShape`, but a model only
/// directly contains top-level shapes.
///
#[derive(Clone, Debug)]
pub struct TopLevelShape {
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
    is_as! { is_unresolved, Unresolved }
}

// ------------------------------------------------------------------------------------------------

impl Shape for TopLevelShape {
    fn id(&self) -> &ShapeID {
        &self.id
    }

    fn set_id(&mut self, id: ShapeID) {
        self.id = id
    }

    fn has_traits(&self) -> bool {
        !self.traits.is_empty()
    }

    fn has_trait(&self, id: &ShapeID) -> bool {
        self.traits.iter().any(|t| t.id() == id)
    }

    fn traits(&self) -> &Vec<AppliedTrait> {
        &self.traits
    }

    fn apply_trait(&mut self, a_trait: AppliedTrait) {
        // TODO: apply trait duplicate rules.
        // (https://github.com/johnstonskj/rust-atelier/issues/5)
        self.traits.push(a_trait);
    }

    fn append_traits(&mut self, traits: &[AppliedTrait]) {
        for a_trait in traits {
            self.apply_trait(a_trait.clone());
        }
    }

    fn remove_trait(&mut self, id: &ShapeID) {
        self.traits.retain(|t| t.id() != id);
    }

    fn is_member(&self) -> bool {
        false
    }
}

impl TopLevelShape {
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

    delegate! { is_simple, inner = body }
    delegate! { is_list, inner = body }
    delegate! { is_set, inner = body }
    delegate! { is_map, inner = body }
    delegate! { is_structure, inner = body }
    delegate! { is_union, inner = body }
    delegate! { is_service, inner = body }
    delegate! { is_operation, inner = body }
    delegate! { is_resource, inner = body }
    delegate! { is_unresolved, inner = body }
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
// Modules
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
pub mod simple;
pub use simple::Simple;

#[doc(hidden)]
pub mod aggregate;
pub use aggregate::{ListOrSet, Map, MemberShape, StructureOrUnion};

#[doc(hidden)]
pub mod service;
pub use service::{Operation, Resource, Service};
