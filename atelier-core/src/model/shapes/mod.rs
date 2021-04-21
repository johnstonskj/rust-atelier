/*!
Model structures common across all shape types.

The concept of a _shape_ in Smithy is abstract, the ABNF contains productions `shape_statements`
and `shape_body` but they are not concrete. Shapes are then classified as _simple_, _aggregate_,
and _service_. The model here introduces `Shape` as a common concrete structure which contains an
enumeration, `ShapeBody`, to represent each of the productions referenced by `shape_body`.

*/

use crate::model::identity::HasIdentity;
use crate::model::{values::Value, Identifier, ShapeID};
use crate::prelude::{
    PRELUDE_NAMESPACE, TRAIT_BOX, TRAIT_DEPRECATED, TRAIT_DOCUMENTATION, TRAIT_ERROR,
    TRAIT_EXTERNALDOCUMENTATION, TRAIT_IDEMPOTENT, TRAIT_LENGTH, TRAIT_NOREPLACE, TRAIT_PAGINATED,
    TRAIT_PATTERN, TRAIT_PRIVATE, TRAIT_READONLY, TRAIT_REFERENCES, TRAIT_REQUIRED,
    TRAIT_REQUIRESLENGTH, TRAIT_SENSITIVE, TRAIT_SINCE, TRAIT_STREAMING, TRAIT_TAGS, TRAIT_TITLE,
    TRAIT_TRAIT, TRAIT_UNIQUEITEMS, TRAIT_UNSTABLE,
};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This trait is implemented by model elements that may have Smithy traits applied.
///
pub trait HasTraits {
    /// Returns `true` if the model element has any applied traits, else `false`.
    fn has_traits(&self) -> bool;

    /// Returns `true` if the model element has any applied traits with the associated id,
    /// else `false`.
    fn has_trait(&self, id: &ShapeID) -> bool;

    /// Return an iterator over all traits applied to this model element
    fn traits(&self) -> &Vec<AppliedTrait>;

    /// Returns all traits applied to this shape with the provided id.
    fn traits_named(&self, id: &ShapeID) -> Vec<&AppliedTrait>;

    /// Apply a trait to this model element.
    fn apply_trait(&mut self, a_trait: AppliedTrait);

    /// Apply a trait with the provided identifier to this model element.
    fn apply(&mut self, a_trait: ShapeID) {
        self.apply_trait(AppliedTrait::new(a_trait));
    }

    /// Apply a trait with the provided identifier and value to this model element.
    fn apply_with_value(&mut self, a_trait: ShapeID, value: Value) {
        self.apply_trait(AppliedTrait::with_value(a_trait, value));
    }

    /// Add all these elements to this member's collection.
    fn append_traits(&mut self, traits: &[AppliedTrait]);

    /// Add all the traits to this model element.
    fn remove_trait(&mut self, id: &ShapeID);

    // --------------------------------------------------------------------------------------------

    /// Returns `true` if the model element has the prelude trait `boxed` applied.
    fn is_boxed(&self) -> bool {
        self.has_trait(&prelude_name(TRAIT_BOX))
    }

    /// Returns `true` if the model element has the prelude trait `deprecated` applied.
    fn is_deprecated(&self) -> bool {
        self.has_trait(&prelude_name(TRAIT_DEPRECATED))
    }

    /// Returns `true` if the model element has the prelude trait `documentation` applied.
    fn is_documentation(&self) -> bool {
        self.has_trait(&prelude_name(TRAIT_DOCUMENTATION))
    }

    /// Returns `true` if the model element has the prelude trait `error` applied.
    fn is_error(&self) -> bool {
        self.has_trait(&prelude_name(TRAIT_ERROR))
    }

    /// Returns `true` if the model element has the prelude trait `external_documentation` applied.
    fn is_external_documentation(&self) -> bool {
        self.has_trait(&prelude_name(TRAIT_EXTERNALDOCUMENTATION))
    }

    /// Returns `true` if the model element has the prelude trait `idempotent` applied.
    fn is_idempotent(&self) -> bool {
        self.has_trait(&prelude_name(TRAIT_IDEMPOTENT))
    }

    /// Returns `true` if the model element has the prelude trait `length` applied.
    fn has_length(&self) -> bool {
        self.has_trait(&prelude_name(TRAIT_LENGTH))
    }

    /// Returns `true` if the model element has the prelude trait `no_replace` applied.
    fn is_no_replace(&self) -> bool {
        self.has_trait(&prelude_name(TRAIT_NOREPLACE))
    }

    /// Returns `true` if the model element has the prelude trait `paginated` applied.
    fn is_paginated(&self) -> bool {
        self.has_trait(&prelude_name(TRAIT_PAGINATED))
    }

    /// Returns `true` if the model element has the prelude trait `pattern` applied.
    fn has_pattern(&self) -> bool {
        self.has_trait(&prelude_name(TRAIT_PATTERN))
    }

    /// Returns `true` if the model element has the prelude trait `private` applied.
    fn is_private(&self) -> bool {
        self.has_trait(&prelude_name(TRAIT_PRIVATE))
    }

    /// Returns `true` if the model element has the prelude trait `readonly` applied.
    fn is_readonly(&self) -> bool {
        self.has_trait(&prelude_name(TRAIT_READONLY))
    }

    /// Returns `true` if the model element has the prelude trait `references` applied.
    fn is_references(&self) -> bool {
        self.has_trait(&prelude_name(TRAIT_REFERENCES))
    }

    /// Returns `true` if the model element has the prelude trait `required` applied.
    fn is_required(&self) -> bool {
        self.has_trait(&prelude_name(TRAIT_REQUIRED))
    }

    /// Returns `true` if the model element has the prelude trait `requiresLength` applied.
    fn has_required_length(&self) -> bool {
        self.has_trait(&prelude_name(TRAIT_REQUIRESLENGTH))
    }

    /// Returns `true` if the model element has the prelude trait `sensitive` applied.
    fn is_sensitive(&self) -> bool {
        self.has_trait(&prelude_name(TRAIT_SENSITIVE))
    }

    /// Returns `true` if the model element has the prelude trait `streaming` applied.
    fn is_streaming(&self) -> bool {
        self.has_trait(&prelude_name(TRAIT_STREAMING))
    }

    /// Returns `true` if the model element has the prelude trait `since` applied.
    fn is_since(&self) -> bool {
        self.has_trait(&prelude_name(TRAIT_SINCE))
    }

    /// Returns `true` if the model element has the prelude trait `tagged` applied.
    fn is_tagged(&self) -> bool {
        self.has_trait(&prelude_name(TRAIT_TAGS))
    }

    /// Returns `true` if the model element has the prelude trait `title` applied.
    fn is_titled(&self) -> bool {
        self.has_trait(&prelude_name(TRAIT_TITLE))
    }

    /// Returns `true` if the model element has the prelude trait `trait` applied.
    fn is_trait(&self) -> bool {
        self.has_trait(&prelude_name(TRAIT_TRAIT))
    }

    /// Returns `true` if the model element has the prelude trait `uniqueItems` applied.
    fn has_unique_items(&self) -> bool {
        self.has_trait(&prelude_name(TRAIT_UNIQUEITEMS))
    }

    /// Returns `true` if the model element has the prelude trait `unstable` applied.
    fn is_unstable(&self) -> bool {
        self.has_trait(&prelude_name(TRAIT_UNSTABLE))
    }
}

///
/// A common trait shared by `TopLevelShape` and `MemberShape`.
///
pub trait Shape: HasIdentity + HasTraits {
    /// Returns `true` if this shape is defined in the prelude.
    fn is_prelude_shape(&self) -> bool {
        self.id().namespace().to_string() == *PRELUDE_NAMESPACE
    }

    /// Is this instance a member (or top-level) shape?
    fn is_member(&self) -> bool;
}

///
/// This structure represents a top-level shape within a model. The shape-specific data is within
/// the `ShapeKind` enumeration. Aggregate shapes may have members of type `MemberShape`, but a
/// model only directly contains top-level shapes.
///
#[derive(Clone, Debug, PartialEq)]
pub struct TopLevelShape {
    id: ShapeID,
    traits: Vec<AppliedTrait>,
    body: ShapeKind,
}

///
/// This enumeration represents the set of shape types supported by Smithy.
///
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, PartialEq)]
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
// Macros
// ------------------------------------------------------------------------------------------------

macro_rules! has_traits_impl {
    ($struct_name:ident . $field_name:ident) => {
        impl HasTraits for $struct_name {
            fn has_traits(&self) -> bool {
                !self.$field_name.is_empty()
            }

            fn has_trait(&self, id: &ShapeID) -> bool {
                self.$field_name.iter().any(|t| t.id() == id)
            }

            fn traits(&self) -> &Vec<AppliedTrait> {
                &self.$field_name
            }

            fn traits_named(&self, id: &ShapeID) -> Vec<&AppliedTrait> {
                self.$field_name.iter().filter(|t| t.id() == id).collect()
            }

            fn apply_trait(&mut self, a_trait: AppliedTrait) {
                // TODO: apply trait duplicate rules.
                // (https://github.com/johnstonskj/rust-atelier/issues/5)
                self.$field_name.push(a_trait);
            }

            fn append_traits(&mut self, traits: &[AppliedTrait]) {
                for a_trait in traits {
                    self.apply_trait(a_trait.clone());
                }
            }

            fn remove_trait(&mut self, id: &ShapeID) {
                self.$field_name.retain(|t| t.id() != id);
            }
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

impl ShapeKind {
    is_as! { simple, Simple, Simple }
    is_as! { list, List, ListOrSet }
    is_as! { set, Set, ListOrSet }
    is_as! { map, Map, Map}
    is_as! { structure, Structure, StructureOrUnion}
    is_as! { union, Union, StructureOrUnion}
    is_as! { service, Service, Service }
    is_as! { operation, Operation, Operation }
    is_as! { resource, Resource, Resource }
    is_only! { unresolved, Unresolved }
}

// ------------------------------------------------------------------------------------------------

impl HasIdentity for TopLevelShape {
    fn id(&self) -> &ShapeID {
        &self.id
    }

    fn set_id(&mut self, id: ShapeID) {
        self.id = id
    }
}

has_traits_impl! { TopLevelShape . traits }

impl Shape for TopLevelShape {
    fn is_member(&self) -> bool {
        false
    }
}

lazy_static! {
    static ref MEMBER_MEMBER: Identifier = Identifier::from_str("member").unwrap();
    static ref MEMBER_KEY: Identifier = Identifier::from_str("key").unwrap();
    static ref MEMBER_VALUE: Identifier = Identifier::from_str("value").unwrap();
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

    // --------------------------------------------------------------------------------------------

    ///
    /// Does this shape support members?
    ///
    pub fn has_members(&self) -> bool {
        !matches!(self.body(), ShapeKind::Simple(_) | ShapeKind::Unresolved)
    }

    ///
    /// Does this shape have a member named `member`?
    ///
    pub fn has_member(&self, member: &Identifier) -> bool {
        self.member(member).is_some()
    }

    ///
    /// Return the value of this shapes member named `member`, if one exists.
    ///
    pub fn member(&self, member: &Identifier) -> Option<&MemberShape> {
        match self.body() {
            ShapeKind::Simple(_) => None,
            ShapeKind::List(v) => {
                if member == MEMBER_MEMBER.deref() {
                    Some(v.member())
                } else {
                    None
                }
            }
            ShapeKind::Set(v) => {
                if member == MEMBER_MEMBER.deref() {
                    Some(v.member())
                } else {
                    None
                }
            }
            ShapeKind::Map(v) => {
                if member == MEMBER_KEY.deref() {
                    Some(v.key())
                } else if member == MEMBER_VALUE.deref() {
                    Some(v.value())
                } else {
                    None
                }
            }
            ShapeKind::Structure(v) => v.member(member),
            ShapeKind::Union(v) => v.member(member),
            ShapeKind::Service(_) => None,
            ShapeKind::Operation(_) => None,
            ShapeKind::Resource(_) => None,
            ShapeKind::Unresolved => None,
        }
    }
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

    /// Returns `true` if this trait is defined in the prelude.
    pub fn is_prelude_trait(&self) -> bool {
        self.id().namespace().to_string() == *PRELUDE_NAMESPACE
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

#[inline]
fn prelude_name(name: &str) -> ShapeID {
    ShapeID::new_unchecked(PRELUDE_NAMESPACE, name, None)
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
use std::ops::Deref;
