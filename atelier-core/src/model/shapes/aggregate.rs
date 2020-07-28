use crate::error::{ErrorKind, Result as ModelResult};
use crate::model::shapes::{AppliedTrait, Shape};
use crate::model::{Identifier, ShapeID};
use crate::syntax::{MEMBER_KEY, MEMBER_MEMBER, MEMBER_VALUE};
use std::collections::HashMap;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Represents a member shape, part of an aggregate top-level shape. The `target` is the target
/// type for this member.
///
#[derive(Clone, Debug)]
pub struct MemberShape {
    id: ShapeID,
    traits: Vec<AppliedTrait>,
    target: ShapeID,
}

///
/// Corresponds to the Smithy List and Set shape. It has a single member, named `member` which determines
/// the shape type for each member of the list.
///
#[derive(Clone, Debug)]
pub struct ListOrSet {
    pub(crate) member: MemberShape,
}

///
/// Corresponds to the Smithy Map shape. It has two members, `key` and `value` which determine the
/// shape types for each element within the map.
///
#[derive(Clone, Debug)]
pub struct Map {
    pub(crate) key: MemberShape,
    pub(crate) value: MemberShape,
}

///
/// Corresponds to the Smithy Structure or Union shape. It has two members, `key` and `value` which determine the
/// shape types for each element within the map.
///
#[derive(Clone, Debug)]
pub struct StructureOrUnion {
    pub(crate) members: HashMap<Identifier, MemberShape>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Shape for MemberShape {
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
        true
    }
}

impl MemberShape {
    /// Construct a new Member shape with the given target shape (type).
    pub fn new(id: ShapeID, target: ShapeID) -> Self {
        Self {
            id,
            traits: Default::default(),
            target,
        }
    }

    /// Construct a new Member shape with the given target shape (type).
    pub fn new_from(parent_id: &ShapeID, id: Identifier, target: ShapeID) -> Self {
        Self {
            id: parent_id.make_member(id),
            traits: Default::default(),
            target,
        }
    }

    /// Construct a new Member shape with the given target shape (type).
    pub fn with_traits(id: ShapeID, target: ShapeID, traits: &[AppliedTrait]) -> Self {
        Self {
            id,
            traits: traits.to_vec(),
            target,
        }
    }

    /// Return the shape identifier which is the target type for this member.
    pub fn target(&self) -> &ShapeID {
        &self.target
    }

    /// Set the shape identifier which is the target type for this member.
    pub fn set_target(&mut self, target: ShapeID) {
        self.target = target;
    }
}

// ------------------------------------------------------------------------------------------------

impl ListOrSet {
    /// Construct a new list with the given `ShapeID` as the reference to the member type.
    pub fn new(parent_id: &ShapeID, target: ShapeID) -> Self {
        Self {
            member: MemberShape::new_from(
                parent_id,
                Identifier::new_unchecked(MEMBER_MEMBER),
                target,
            ),
        }
    }

    pub fn from(member: MemberShape) -> Self {
        Self { member }
    }

    /// Return the identifier for the type of each member of the list or set.
    pub fn member(&self) -> &MemberShape {
        &self.member
    }

    /// Set the identifier of the type of each member of the list or set.
    pub fn set_member(&mut self, member: MemberShape) {
        assert_eq!(member.id(), self.member.id());
        assert!(member.is_member());
        self.member = member
    }
}

// ------------------------------------------------------------------------------------------------

impl Map {
    /// Construct a new map with the given `ShapeID`s as the reference to the key and value types.
    pub fn new(parent_id: &ShapeID, key_shape: ShapeID, value_shape: ShapeID) -> Self {
        Self {
            key: MemberShape::new_from(parent_id, Identifier::new_unchecked(MEMBER_KEY), key_shape),
            value: MemberShape::new_from(
                parent_id,
                Identifier::new_unchecked(MEMBER_VALUE),
                value_shape,
            ),
        }
    }

    pub fn from(key: MemberShape, value: MemberShape) -> Self {
        Self { key, value }
    }

    /// Return the identifier for the type of the key for each member of the list or set.
    pub fn key(&self) -> &MemberShape {
        &self.key
    }

    /// Set the identifier for the type of the key for each member of the list or set.
    pub fn set_key(&mut self, key: MemberShape) {
        assert_eq!(key.id(), self.key.id());
        assert!(key.is_member());
        self.key = key;
    }

    /// Return the identifier for the type of the value for each member of the list or set.
    pub fn value(&self) -> &MemberShape {
        &self.value
    }

    /// Set the identifier for the type of the value for each member of the list or set.
    pub fn set_value(&mut self, value: MemberShape) {
        assert_eq!(value.id(), self.value.id());
        assert!(value.is_member());
        self.value = value;
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for StructureOrUnion {
    fn default() -> Self {
        Self {
            members: Default::default(),
        }
    }
}

impl StructureOrUnion {
    /// Construct a new, empty, structure or union.
    pub fn new() -> Self {
        Self::default()
    }

    /// Construct a new structure or union with the provided group of members. Note that member
    /// identifiers must be unique, so duplicates in the slice will be overridden.
    ///
    /// Note: that all members must have a body variant `ShapeBody::Member`, otherwise this method
    /// will panic.
    pub fn with_members(members: &[MemberShape]) -> Self {
        assert!(members.iter().all(|shape| shape.is_member()));
        let mut new = Self {
            members: Default::default(),
        };
        for member in members {
            let _ = new.add_a_member(member.clone());
        }
        new
    }

    /// Returns `true` if this structure or union has _any_ members, else `false`.
    pub fn has_members(&self) -> bool {
        !self.members.is_empty()
    }

    /// Returns `true` if this structure or union has a member with the given name, else `false`.
    pub fn has_member(&self, member_name: &Identifier) -> bool {
        !self.members.contains_key(member_name)
    }

    /// Returns the member in the structure or union with the given name, else `None`.
    pub fn member(&self, member_name: &Identifier) -> Option<&MemberShape> {
        self.members.get(member_name)
    }

    /// Remove the member in the structure or union with the given name.
    pub fn remove_member(&mut self, member_name: &Identifier) -> Option<MemberShape> {
        self.members.remove(member_name)
    }

    /// Return an iterator over all members in this structure or union.
    pub fn members(&self) -> impl Iterator<Item = &MemberShape> {
        self.members.values()
    }

    pub fn add_member(&mut self, member_name: ShapeID, target: ShapeID) {
        let shape = MemberShape::new(member_name, target);
        let _ = self.add_a_member(shape);
    }

    pub fn add_member_from(&mut self, parent_id: &ShapeID, id: Identifier, target: ShapeID) {
        let shape = MemberShape::new_from(parent_id, id, target);
        let _ = self.add_a_member(shape);
    }

    pub fn add_a_member(&mut self, member: MemberShape) -> ModelResult<Option<MemberShape>> {
        if !member.id().is_member() {
            Err(ErrorKind::MemberIDExpected(member.id().clone()).into())
        } else {
            Ok(self
                .members
                .insert(member.id().member_name().clone().unwrap(), member))
        }
    }
}
