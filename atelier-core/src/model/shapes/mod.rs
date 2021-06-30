/*!
Model structures common across all shape types.

The concept of a _shape_ in Smithy is abstract, the ABNF contains productions `shape_statements`
and `shape_body` but they are not concrete. Shapes are then classified as _simple_, _aggregate_,
and _service_. The model here introduces `Shape` as a common concrete structure which contains an
enumeration, `ShapeBody`, to represent each of the productions referenced by `shape_body`.

*/

use crate::error::{ErrorKind, Result};
use crate::model::identity::HasIdentity;
use crate::model::{values::Value, Identifier, ShapeID};
use crate::prelude::{
    PRELUDE_NAMESPACE, TRAIT_BOX, TRAIT_DEPRECATED, TRAIT_DOCUMENTATION, TRAIT_ERROR,
    TRAIT_EXTERNALDOCUMENTATION, TRAIT_IDEMPOTENT, TRAIT_LENGTH, TRAIT_NOREPLACE, TRAIT_PAGINATED,
    TRAIT_PATTERN, TRAIT_PRIVATE, TRAIT_READONLY, TRAIT_REFERENCES, TRAIT_REQUIRED,
    TRAIT_REQUIRESLENGTH, TRAIT_SENSITIVE, TRAIT_SINCE, TRAIT_STREAMING, TRAIT_TAGS, TRAIT_TITLE,
    TRAIT_TRAIT, TRAIT_UNIQUEITEMS, TRAIT_UNSTABLE,
};
use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::Deref;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This trait defines an equality operation that compares the structure of a shape without
/// any comparison of applied traits.
///
pub trait NonTraitEq {
    /// Returns `true` if the two shapes are equal in structure, else `false`.
    fn equal_without_traits(&self, other: &Self) -> bool;
}

///
/// The value of an applied trait, this is optional for some traits.
///
pub type TraitValue = Option<Value>;

///
/// The set of traits applied to a shape.
///
pub type AppliedTraits = HashMap<ShapeID, TraitValue>;

///
/// This trait is implemented by model elements that may have Smithy traits applied.
///
pub trait HasTraits {
    /// Returns `true` if the model element has any applied traits, else `false`.
    fn has_traits(&self) -> bool {
        !self.traits().is_empty()
    }

    /// Returns `true` if the model element has any applied traits with the associated id,
    /// else `false`.
    fn has_trait(&self, id: &ShapeID) -> bool {
        self.traits().contains_key(id)
    }

    /// Return an iterator over all traits applied to this model element
    fn traits(&self) -> &AppliedTraits;

    /// Return an iterator over all traits applied to this model element
    fn traits_mut(&mut self) -> &mut AppliedTraits;

    /// Returns all traits applied to this shape with the provided id.
    fn trait_named(&self, id: &ShapeID) -> Option<&TraitValue> {
        self.traits().get(id)
    }

    /// Apply a trait with the provided identifier to this model element.
    fn apply(&mut self, id: ShapeID) -> Result<()> {
        self.apply_with_value(id, None)
    }

    ///
    /// Apply a trait with the provided identifier and value to this model element ensuring the
    /// conflict resolution rules are applied.
    ///
    /// From [Trait conflict resolution](https://awslabs.github.io/smithy/1.0/spec/core/model.html#trait-conflict-resolution):
    ///
    /// > Duplicate traits applied to shapes are allowed in the following cases:
    /// >
    /// > 1. If the trait is a list or set shape, then the conflicting trait values are concatenated
    /// >    into a single trait value.
    /// > 1. If both values are exactly equal, then the conflict is ignored.
    /// >
    /// > All other instances of trait collisions are prohibited.
    ///
    fn apply_with_value(&mut self, a_trait: ShapeID, value: TraitValue) -> Result<()>;

    /// Add all these elements to this member's collection.
    fn append_traits(&mut self, traits: &AppliedTraits) -> Result<()> {
        for (id, value) in traits {
            self.apply_with_value(id.clone(), value.clone())?;
        }
        Ok(())
    }

    /// Add all the traits to this model element.
    fn remove_trait(&mut self, id: &ShapeID) {
        let _ = self.traits_mut().remove(id);
    }

    // --------------------------------------------------------------------------------------------

    /// Returns `true` if the model element has the prelude trait `documentation` applied.
    fn has_documentation(&self) -> bool {
        self.has_trait(&prelude_name(TRAIT_DOCUMENTATION))
    }

    /// Returns `true` if the model element has the prelude trait `external_documentation` applied.
    fn has_external_documentation(&self) -> bool {
        self.has_trait(&prelude_name(TRAIT_EXTERNALDOCUMENTATION))
    }

    /// Returns `true` if the model element has the prelude trait `length` applied.
    fn has_length(&self) -> bool {
        self.has_trait(&prelude_name(TRAIT_LENGTH))
    }

    /// Returns `true` if the model element has the prelude trait `pattern` applied.
    fn has_pattern(&self) -> bool {
        self.has_trait(&prelude_name(TRAIT_PATTERN))
    }

    /// Returns `true` if the model element has the prelude trait `requiresLength` applied.
    fn has_required_length(&self) -> bool {
        self.has_trait(&prelude_name(TRAIT_REQUIRESLENGTH))
    }

    // --------------------------------------------------------------------------------------------

    /// Returns `true` if the model element has the prelude trait `boxed` applied.
    fn is_boxed(&self) -> bool {
        self.has_trait(&prelude_name(TRAIT_BOX))
    }

    /// Returns `true` if the model element has the prelude trait `deprecated` applied.
    fn is_deprecated(&self) -> bool {
        self.has_trait(&prelude_name(TRAIT_DEPRECATED))
    }

    /// Returns `true` if the model element has the prelude trait `error` applied.
    fn is_error(&self) -> bool {
        self.has_trait(&prelude_name(TRAIT_ERROR))
    }

    /// Returns `true` if the model element has the prelude trait `idempotent` applied.
    fn is_idempotent(&self) -> bool {
        self.has_trait(&prelude_name(TRAIT_IDEMPOTENT))
    }

    /// Returns `true` if the model element has the prelude trait `no_replace` applied.
    fn is_no_replace(&self) -> bool {
        self.has_trait(&prelude_name(TRAIT_NOREPLACE))
    }

    /// Returns `true` if the model element has the prelude trait `paginated` applied.
    fn is_paginated(&self) -> bool {
        self.has_trait(&prelude_name(TRAIT_PAGINATED))
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
/// This structure represents a top-level shape within a model. The shape-specific data is within
/// the `ShapeKind` enumeration. Aggregate shapes may have members of type `MemberShape`, but a
/// model only directly contains top-level shapes.
///
#[derive(Clone, Debug, PartialEq)]
pub struct TopLevelShape {
    id: ShapeID,
    traits: HashMap<ShapeID, Option<Value>>,
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

// ------------------------------------------------------------------------------------------------
// Macros
// ------------------------------------------------------------------------------------------------

macro_rules! has_traits_impl {
    ($struct_name:ident . $field_name:ident) => {
        impl HasTraits for $struct_name {
            fn traits(&self) -> &AppliedTraits {
                &self.$field_name
            }

            fn traits_mut(&mut self) -> &mut AppliedTraits {
                &mut self.$field_name
            }

            fn apply_with_value(
                &mut self,
                id: ShapeID,
                value: Option<Value>,
            ) -> $crate::error::Result<()> {
                if id.is_member() {
                    return Err(crate::error::ErrorKind::ShapeIDExpected(id).into());
                } else if let Some(trait_value) = self.trait_named(&id) {
                    let new_value = $crate::model::shapes::merge_traits(&id, &trait_value, &value)?;
                    let _ = self.$field_name.insert(id, new_value);
                } else {
                    let _ = self.$field_name.insert(id, value);
                }
                Ok(())
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

impl NonTraitEq for TopLevelShape {
    fn equal_without_traits(&self, other: &Self) -> bool {
        self.id() == other.id()
            && match (self.body(), other.body()) {
                (ShapeKind::Simple(l), ShapeKind::Simple(r)) => l == r,
                (ShapeKind::List(l), ShapeKind::List(r)) => {
                    l.member.equal_without_traits(&r.member)
                }
                (ShapeKind::Set(l), ShapeKind::Set(r)) => l.member.equal_without_traits(&r.member),
                (ShapeKind::Map(l), ShapeKind::Map(r)) => {
                    l.key.equal_without_traits(&r.key) && l.value.equal_without_traits(&r.value)
                }
                (ShapeKind::Structure(l), ShapeKind::Structure(r)) => {
                    l.members.keys().count() == r.members.keys().count()
                        && l.members.keys().all(|k| {
                            l.member(k)
                                .unwrap()
                                .equal_without_traits(r.member(k).unwrap())
                        })
                }
                (ShapeKind::Union(l), ShapeKind::Union(r)) => {
                    l.members.keys().count() == r.members.keys().count()
                        && l.members.keys().all(|k| {
                            l.member(k)
                                .unwrap()
                                .equal_without_traits(r.member(k).unwrap())
                        })
                }
                (ShapeKind::Service(l), ShapeKind::Service(r)) => l == r,
                (ShapeKind::Operation(l), ShapeKind::Operation(r)) => l == r,
                (ShapeKind::Resource(l), ShapeKind::Resource(r)) => l == r,
                (ShapeKind::Unresolved, ShapeKind::Unresolved) => true,
                (_, _) => false,
            }
    }
}

impl HasIdentity<ShapeID> for TopLevelShape {
    fn id(&self) -> &ShapeID {
        &self.id
    }

    fn set_id(&mut self, id: ShapeID) {
        self.id = id
    }
}

has_traits_impl! { TopLevelShape . traits }

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
    pub fn with_traits(
        id: ShapeID,
        body: ShapeKind,
        traits: HashMap<ShapeID, Option<Value>>,
    ) -> Self {
        Self { id, traits, body }
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
    /// Returns `true` if the namespace of this shape corresponds to the Smithy
    /// [prelude namespace](../../prelude/constant.PRELUDE_NAMESPACE.html), else `false`.
    ///
    pub fn is_prelude_shape(&self) -> bool {
        self.id().namespace().to_string() == *PRELUDE_NAMESPACE
    }

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
// Private Functions
// ------------------------------------------------------------------------------------------------

#[inline]
fn prelude_name(name: &str) -> ShapeID {
    ShapeID::new_unchecked(PRELUDE_NAMESPACE, name, None)
}

///
/// From [Trait conflict resolution](https://awslabs.github.io/smithy/1.0/spec/core/model.html#trait-conflict-resolution):
///
/// > Duplicate traits applied to shapes are allowed in the following cases:
/// >
/// > 1. If the trait is a list or set shape, then the conflicting trait values are concatenated
/// >    into a single trait value.
/// > 1. If both values are exactly equal, then the conflict is ignored.
/// >
/// > All other instances of trait collisions are prohibited.
///
pub(crate) fn merge_traits(
    id: &ShapeID,
    left: &TraitValue,
    right: &TraitValue,
) -> Result<TraitValue> {
    match (left, right) {
        (Some(Value::Array(left)), Some(Value::Array(right))) => {
            if left.is_empty() {
                Ok(Some(Value::Array(right.clone())))
            } else if right.is_empty() {
                Ok(Some(Value::Array(left.clone())))
            } else {
                let mut result = left.clone();
                result.extend(right.iter().cloned());
                Ok(Some(Value::Array(result)))
            }
        }
        (left, right) => {
            if left == right {
                Ok(left.clone())
            } else {
                Err(ErrorKind::MergeTraitConflict(id.clone()).into())
            }
        }
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
