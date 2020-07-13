use crate::model::shapes::{Member, Shape};
use crate::model::{Identifier, ShapeID};
use crate::prelude::PRELUDE_NAMESPACE;
use crate::syntax::{MEMBER_KEY, MEMBER_MEMBER, MEMBER_VALUE, SHAPE_LIST, SHAPE_MAP, SHAPE_SET};
use std::collections::HashMap;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Corresponds to the Smithy List and Set shape. It has a single member, named `member` which determines
/// the shape type for each member of the list.
///
#[derive(Clone, Debug)]
pub struct ListOrSet {
    member: Box<Shape>,
}

///
/// Corresponds to the Smithy Map shape. It has two members, `key` and `value` which determine the
/// shape types for each element within the map.
///
#[derive(Clone, Debug)]
pub struct Map {
    key: Box<Shape>,
    value: Box<Shape>,
}

///
/// Corresponds to the Smithy Structure or Union shape. It has two members, `key` and `value` which determine the
/// shape types for each element within the map.
///
#[derive(Clone, Debug)]
pub struct StructureOrUnion {
    members: HashMap<Identifier, Box<Shape>>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl ListOrSet {
    /// Construct a new list with the given `ShapeID` as the reference to the member type.
    pub fn new_list(member_shape: ShapeID) -> Self {
        Self {
            member: Box::new(Shape::new(
                ShapeID::new_unchecked(Some(PRELUDE_NAMESPACE), SHAPE_LIST, Some(MEMBER_MEMBER)),
                Member::new(Identifier::new_unchecked(MEMBER_MEMBER), member_shape).into(),
            )),
        }
    }

    /// Construct a new set with the given `ShapeID` as the reference to the member type.
    pub fn new_set(member_shape: ShapeID) -> Self {
        Self {
            member: Box::new(Shape::new(
                ShapeID::new_unchecked(Some(PRELUDE_NAMESPACE), SHAPE_SET, Some(MEMBER_MEMBER)),
                Member::new(Identifier::new_unchecked(MEMBER_MEMBER), member_shape).into(),
            )),
        }
    }

    /// Return the identifier for the type of each member of the list or set.
    pub fn member(&self) -> &Shape {
        &self.member
    }

    /// Set the identifier of the type of each member of the list or set.
    pub fn set_member(&mut self, member: Shape) {
        assert_eq!(member.id(), self.member.id());
        assert!(member.is_member());
        self.member = Box::new(member)
    }
}

// ------------------------------------------------------------------------------------------------

impl Map {
    /// Construct a new map with the given `ShapeID`s as the reference to the key and value types.
    pub fn new(key_shape: ShapeID, value_shape: ShapeID) -> Self {
        Self {
            key: Box::new(Shape::new(
                ShapeID::new_unchecked(Some(PRELUDE_NAMESPACE), SHAPE_MAP, Some(MEMBER_KEY)),
                Member::new(Identifier::new_unchecked(MEMBER_KEY), key_shape).into(),
            )),
            value: Box::new(Shape::new(
                ShapeID::new_unchecked(Some(PRELUDE_NAMESPACE), SHAPE_MAP, Some(MEMBER_VALUE)),
                Member::new(Identifier::new_unchecked(MEMBER_VALUE), value_shape).into(),
            )),
        }
    }

    /// Return the identifier for the type of the key for each member of the list or set.
    pub fn key(&self) -> &Shape {
        &self.key
    }

    /// Set the identifier for the type of the key for each member of the list or set.
    pub fn set_key(&mut self, key: Shape) {
        assert_eq!(key.id(), self.key.id());
        assert!(key.is_member());
        self.key = Box::new(key);
    }

    /// Return the identifier for the type of the value for each member of the list or set.
    pub fn value(&self) -> &Shape {
        &self.value
    }

    /// Set the identifier for the type of the value for each member of the list or set.
    pub fn set_value(&mut self, value: Shape) {
        assert_eq!(value.id(), self.value.id());
        assert!(value.is_member());
        self.value = Box::new(value);
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
    pub fn with_members(members: &[Shape]) -> Self {
        assert!(members.iter().all(|shape| shape.is_member()));
        let mut new = Self {
            members: Default::default(),
        };
        for member in members {
            let _ = new.add_a_member(Box::new(member.clone()));
        }
        new
    }

    object_member! { members, member, Identifier, Box<Shape>, has_members, has_member, add_member = add_a_member, remove_member }

    pub fn add_a_member(&mut self, member: Box<Shape>) -> Option<Box<Shape>> {
        assert!(member.is_member());
        assert!(member.id().is_member());
        self.members
            .insert(member.id().member_name().clone().unwrap(), member)
    }
}
