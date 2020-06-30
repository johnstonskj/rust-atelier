use crate::error::{invalid_value_variant, Error, ErrorKind, Result};
use crate::model::shapes::{HasMembers, Member, Valued};
use crate::model::values::NodeValue;
use crate::model::{Identifier, Named, ShapeID};
use crate::syntax::{
    MEMBER_KEY, MEMBER_MEMBER, MEMBER_VALUE, SHAPE_BIG_DECIMAL, SHAPE_BIG_INTEGER, SHAPE_BLOB,
    SHAPE_BOOLEAN, SHAPE_BYTE, SHAPE_DOCUMENT, SHAPE_DOUBLE, SHAPE_FLOAT, SHAPE_INTEGER,
    SHAPE_LONG, SHAPE_SHORT, SHAPE_STRING, SHAPE_TIMESTAMP,
};
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
                SimpleType::Blob => SHAPE_BLOB,
                SimpleType::Boolean => SHAPE_BOOLEAN,
                SimpleType::Document => SHAPE_DOCUMENT,
                SimpleType::String => SHAPE_STRING,
                SimpleType::Byte => SHAPE_BYTE,
                SimpleType::Short => SHAPE_SHORT,
                SimpleType::Integer => SHAPE_INTEGER,
                SimpleType::Long => SHAPE_LONG,
                SimpleType::Float => SHAPE_FLOAT,
                SimpleType::Double => SHAPE_DOUBLE,
                SimpleType::BigInteger => SHAPE_BIG_INTEGER,
                SimpleType::BigDecimal => SHAPE_BIG_DECIMAL,
                SimpleType::Timestamp => SHAPE_TIMESTAMP,
            }
        )
    }
}

impl FromStr for SimpleType {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            SHAPE_BLOB => Ok(SimpleType::Blob),
            SHAPE_BOOLEAN => Ok(SimpleType::Boolean),
            SHAPE_DOCUMENT => Ok(SimpleType::Document),
            SHAPE_STRING => Ok(SimpleType::String),
            SHAPE_BYTE => Ok(SimpleType::Byte),
            SHAPE_SHORT => Ok(SimpleType::Short),
            SHAPE_INTEGER => Ok(SimpleType::Integer),
            SHAPE_LONG => Ok(SimpleType::Long),
            SHAPE_FLOAT => Ok(SimpleType::Float),
            SHAPE_DOUBLE => Ok(SimpleType::Double),
            SHAPE_BIG_INTEGER => Ok(SimpleType::BigInteger),
            SHAPE_BIG_DECIMAL => Ok(SimpleType::BigDecimal),
            SHAPE_TIMESTAMP => Ok(SimpleType::Timestamp),
            _ => Err(ErrorKind::UnknownType(s.to_string()).into()),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl HasMembers for ListOrSet {
    fn has_member_named(&self, member_name: &Identifier) -> bool {
        member_name.to_string() == MEMBER_MEMBER
    }

    fn get_member_named(&self, member_name: &Identifier) -> Option<&Member> {
        if self.has_member_named(member_name) {
            Some(&self.member)
        } else {
            None
        }
    }

    fn set_member(&mut self, member: Member) -> Result<()> {
        if self.has_member_named(member.id()) {
            if let Some(NodeValue::ShapeID(_)) = member.value() {
                self.member = member;
                Ok(())
            } else {
                Err(ErrorKind::InvalidValueVariant("ShapeID".to_string()).into())
            }
        } else {
            Err(ErrorKind::UnknownMember(member.id().to_string()).into())
        }
    }
}

impl ListOrSet {
    /// Construct a new list or set with the given `ShapeID` as the reference to the member type.
    pub fn new(member: ShapeID) -> Self {
        Self {
            member: Member::with_value(
                Identifier::from_str(MEMBER_MEMBER).unwrap(),
                NodeValue::ShapeID(member),
            ),
        }
    }

    /// Return the identifier for the type of each member of the list or set.
    pub fn member(&self) -> &ShapeID {
        match &self.member.value().as_ref().unwrap() {
            NodeValue::ShapeID(id) => id,
            _ => invalid_value_variant("ShapeID"),
        }
    }

    /// Set the identifier of the type of each member of the list or set.
    pub fn set_member(&mut self, member: ShapeID) {
        self.member.set_value(NodeValue::ShapeID(member))
    }
}

// ------------------------------------------------------------------------------------------------

impl HasMembers for Map {
    fn has_member_named(&self, member_name: &Identifier) -> bool {
        [MEMBER_KEY, MEMBER_VALUE].contains(&member_name.to_string().as_str())
    }

    fn get_member_named(&self, member_name: &Identifier) -> Option<&Member> {
        if member_name.to_string() == MEMBER_KEY {
            Some(&self.key)
        } else if member_name.to_string() == MEMBER_VALUE {
            Some(&self.value)
        } else {
            None
        }
    }

    fn set_member(&mut self, member: Member) -> Result<()> {
        if member.id().to_string() == MEMBER_KEY {
            if let Some(NodeValue::ShapeID(_)) = member.value() {
                self.key = member;
                Ok(())
            } else {
                Err(ErrorKind::InvalidValueVariant("ShapeID".to_string()).into())
            }
        } else if member.id().to_string() == MEMBER_VALUE {
            if let Some(NodeValue::ShapeID(_)) = member.value() {
                self.value = member;
                Ok(())
            } else {
                Err(ErrorKind::InvalidValueVariant("ShapeID".to_string()).into())
            }
        } else {
            Err(ErrorKind::UnknownMember(member.id().to_string()).into())
        }
    }
}

impl Map {
    /// Construct a new map with the given `ShapeID`s as the reference to the key and value types.
    pub fn new(key: ShapeID, value: ShapeID) -> Self {
        Self {
            key: Member::with_value(
                Identifier::from_str(MEMBER_KEY).unwrap(),
                NodeValue::ShapeID(key),
            ),
            value: Member::with_value(
                Identifier::from_str(MEMBER_VALUE).unwrap(),
                NodeValue::ShapeID(value),
            ),
        }
    }

    /// Return the identifier for the type of the key for each member of the list or set.
    pub fn key(&self) -> &ShapeID {
        match &self.key.value().as_ref().unwrap() {
            NodeValue::ShapeID(id) => id,
            _ => invalid_value_variant("ShapeID"),
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
            _ => invalid_value_variant("ShapeID"),
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

impl HasMembers for StructureOrUnion {
    fn has_member_named(&self, member_name: &Identifier) -> bool {
        self.has_member(member_name)
    }

    fn get_member_named(&self, member_name: &Identifier) -> Option<&Member> {
        self.member(member_name)
    }

    fn set_member(&mut self, member: Member) -> Result<()> {
        if self.has_member_named(member.id()) {
            if let Some(NodeValue::ShapeID(_)) = member.value() {
                // TODO: check inner types match
                let _ = self.members.insert(member.id().clone(), member);
                Ok(())
            } else {
                Err(ErrorKind::InvalidValueVariant("ShapeID".to_string()).into())
            }
        } else {
            Err(ErrorKind::UnknownMember(member.id().to_string()).into())
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

    /// Returns `true` if this structure or union has a member with the given identifier, else `false`.
    pub fn has_member(&self, id: &Identifier) -> bool {
        self.members.contains_key(id)
    }

    /// Return an iterator over all members of the structure or union.
    pub fn members(&self) -> impl Iterator<Item = &Member> {
        self.members.values()
    }

    /// Return the member with the given identifier, else `None`.
    pub fn member(&self, id: &Identifier) -> Option<&Member> {
        self.members.get(id)
    }

    /// Add the given member to this structure or union; this will overwrite any existing member
    /// with the same identifier.
    pub fn add_member(&mut self, member: Member) {
        let _ = self.members.insert(member.id().clone(), member);
    }

    /// Create and add a new member with the given identifier and value to this structure or union; this
    /// will overwrite any existing member with the same identifier.
    pub fn add_member_value(&mut self, id: Identifier, value: NodeValue) {
        let _ = self
            .members
            .insert(id.clone(), Member::with_value(id, value));
    }

    // Add all members of the provided group of members to this structure or union. Note that member
    // identifiers must be unique, so duplicates in the slice will be overridden.
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
