use crate::error::invalid_value_variant;
use crate::model::shapes::{Member, Valued};
use crate::model::values::NodeValue;
use crate::model::{Identifier, Named, ShapeID};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Corresponds to the simple shape within Smithy, these are atomic values.
///
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum SimpleType {
    /// Corresponds to the `simple_type_name` production's "blob" terminal.
    Blob,
    /// Corresponds to the `simple_type_name` production's "boolean" terminal.
    Boolean,
    /// Corresponds to the `simple_type_name` production's "document" terminal.
    Document,
    /// Corresponds to the `simple_type_name` production's "string" terminal.
    String,
    /// Corresponds to the `simple_type_name` production's "byte" terminal.
    Byte,
    /// Corresponds to the `simple_type_name` production's "short" terminal.
    Short,
    /// Corresponds to the `simple_type_name` production's "integer" terminal.
    Integer,
    /// Corresponds to the `simple_type_name` production's "long" terminal.
    Long,
    /// Corresponds to the `simple_type_name` production's "float" terminal.
    Float,
    /// Corresponds to the `simple_type_name` production's "double" terminal.
    Double,
    /// Corresponds to the `simple_type_name` production's "bigInteger" terminal.
    BigInteger,
    /// Corresponds to the `simple_type_name` production's "bigDecimal" terminal.
    BigDecimal,
    /// Corresponds to the `simple_type_name` production's "timestamp" terminal.
    Timestamp,
}

///
/// Corresponds to the Smithy List and Set shape. It has a single member, named `member` which determines
/// the shape type for each member of the list.
///
#[derive(Clone, Debug)]
pub struct ListOrSet {
    member: Member, // Value::ShapeID
}

///
/// Corresponds to the Smithy Map shape. It has two members, `key` and `value` which determine the
/// shape types for each element within the map.
///
#[derive(Clone, Debug)]
pub struct Map {
    key: Member,   // Value::ShapeID
    value: Member, // Value::ShapeID
}

///
/// Corresponds to the Smithy Structure or Union shape. It has two members, `key` and `value` which determine the
/// shape types for each element within the map.
///
#[derive(Clone, Debug)]
pub struct StructureOrUnion {
    members: HashMap<Identifier, Member>, // Value::ShapeID
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for SimpleType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SimpleType::Blob => "blob",
                SimpleType::Boolean => "boolean",
                SimpleType::Document => "document",
                SimpleType::String => "string",
                SimpleType::Byte => "byte",
                SimpleType::Short => "short",
                SimpleType::Integer => "integer",
                SimpleType::Long => "long",
                SimpleType::Float => "float",
                SimpleType::Double => "double",
                SimpleType::BigInteger => "bigInteger",
                SimpleType::BigDecimal => "bigDecimal",
                SimpleType::Timestamp => "timestamp",
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl ListOrSet {
    /// Construct a new list or set with the given `ShapeID` as the reference to the member type.
    pub fn new(member: ShapeID) -> Self {
        Self {
            member: Member::with_value(
                Identifier::from_str("member").unwrap(),
                NodeValue::ShapeID(member),
            ),
        }
    }

    /// Return the identifier for the type of each member of the list or set.
    pub fn member(&self) -> &ShapeID {
        match &self.member.value().as_ref().unwrap() {
            NodeValue::ShapeID(id) => id,
            _ => invalid_value_variant("Ref"),
        }
    }

    /// Set the identifier of the type of each member of the list or set.
    pub fn set_member(&mut self, member: ShapeID) {
        self.member.set_value(NodeValue::ShapeID(member))
    }
}

// ------------------------------------------------------------------------------------------------

impl Map {
    /// Construct a new map with the given `ShapeID`s as the reference to the key and value types.
    pub fn new(key: ShapeID, value: ShapeID) -> Self {
        Self {
            key: Member::with_value(
                Identifier::from_str("key").unwrap(),
                NodeValue::ShapeID(key),
            ),
            value: Member::with_value(
                Identifier::from_str("value").unwrap(),
                NodeValue::ShapeID(value),
            ),
        }
    }

    /// Return the identifier for the type of the key for each member of the list or set.
    pub fn key(&self) -> &ShapeID {
        match &self.key.value().as_ref().unwrap() {
            NodeValue::ShapeID(id) => id,
            _ => invalid_value_variant("Ref"),
        }
    }

    /// Set the identifier for the type of the key for each member of the list or set.
    pub fn set_key(&mut self, key: ShapeID) {
        self.key.set_value(NodeValue::ShapeID(key))
    }

    /// Return the identifier for the type of the value for each member of the list or set.
    pub fn value(&self) -> &ShapeID {
        match &self.value.value().as_ref().unwrap() {
            NodeValue::ShapeID(id) => id,
            _ => invalid_value_variant("Ref"),
        }
    }

    /// Set the identifier for the type of the value for each member of the list or set.
    pub fn set_value(&mut self, value: ShapeID) {
        self.value.set_value(NodeValue::ShapeID(value))
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

    /// Construct a new structure or union with the provided group of members. Note that member IDs
    /// must be unique, so duplicates in the slice will be overridden.
    pub fn with_members(members: &[Member]) -> Self {
        Self {
            members: members
                .iter()
                .map(|m| (m.id().clone(), m.clone()))
                .collect(),
        }
    }

    /// Returns `true` if this structure or union has _any_ members, else `false`.
    pub fn has_members(&self) -> bool {
        !self.members.is_empty()
    }

    /// Returns `true` if this structure or union has a member with the given ID, else `false`.
    pub fn has_member(&mut self, id: &Identifier) -> bool {
        self.members.contains_key(id)
    }

    /// Return an iterator over all members of the structure or union.
    pub fn members(&self) -> impl Iterator<Item = &Member> {
        self.members.values()
    }

    /// Add the given member to this structure or union; this will overwrite any existing member
    /// with the same ID.
    pub fn add_member(&mut self, member: Member) {
        let _ = self.members.insert(member.id().clone(), member);
    }

    /// Create and add a new member with the given ID and value to this structure or union; this
    /// will overwrite any existing member with the same ID.
    pub fn add_member_value(&mut self, id: Identifier, value: Option<NodeValue>) {
        let _ = match value {
            None => self.members.insert(id.clone(), Member::new(id)),
            Some(value) => self
                .members
                .insert(id.clone(), Member::with_value(id, value)),
        };
    }

    // Add all members of the provided group of members to this structure or union. Note that member IDs
    // must be unique, so duplicates in the slice will be overridden.
    pub fn append_members(&mut self, members: &[Member]) {
        for member in members {
            self.add_member(member.clone());
        }
    }

    /// Remove the member with the given identifier.
    pub fn remove_member(&mut self, id: &Identifier) {
        let _ = self.members.remove(id);
    }
}
