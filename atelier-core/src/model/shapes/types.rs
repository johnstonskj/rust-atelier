/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::invalid_value_variant;
use crate::model::shapes::{Member, Valued};
use crate::model::values::Value;
use crate::model::{Identifier, ShapeID};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum SimpleType {
    Blob,
    Boolean,
    Document,
    String,
    Byte,
    Short,
    Integer,
    Long,
    Float,
    Double,
    BigInteger,
    BigDecimal,
    Timestamp,
}

#[derive(Clone, Debug)]
pub struct ListOrSet {
    member: Member, // Value::Ref
}

#[derive(Clone, Debug)]
pub struct Map {
    key: Member,   // Value::Ref
    value: Member, // Value::Ref
}

#[derive(Clone, Debug)]
pub struct StructureOrUnion {
    members: Vec<Member>, // Value::Ref
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

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
    pub fn new(member: ShapeID) -> Self {
        Self {
            member: Member::with_value(Identifier::from_str("member").unwrap(), Value::Ref(member)),
        }
    }

    pub fn member(&self) -> &ShapeID {
        match &self.member.value().as_ref().unwrap() {
            Value::Ref(id) => id,
            _ => invalid_value_variant("Ref"),
        }
    }
    pub fn set_member(&mut self, member: ShapeID) {
        self.member.set_value(Value::Ref(member))
    }
}

// ------------------------------------------------------------------------------------------------

impl Map {
    pub fn new(key: ShapeID, value: ShapeID) -> Self {
        Self {
            key: Member::with_value(Identifier::from_str("key").unwrap(), Value::Ref(key)),
            value: Member::with_value(Identifier::from_str("value").unwrap(), Value::Ref(value)),
        }
    }

    pub fn key(&self) -> &ShapeID {
        match &self.key.value().as_ref().unwrap() {
            Value::Ref(id) => id,
            _ => invalid_value_variant("Ref"),
        }
    }
    pub fn set_key(&mut self, key: ShapeID) {
        self.key.set_value(Value::Ref(key))
    }

    pub fn value(&self) -> &ShapeID {
        match &self.value.value().as_ref().unwrap() {
            Value::Ref(id) => id,
            _ => invalid_value_variant("Ref"),
        }
    }
    pub fn set_value(&mut self, value: ShapeID) {
        self.value.set_value(Value::Ref(value))
    }
}

// ------------------------------------------------------------------------------------------------

impl StructureOrUnion {
    pub fn new(members: &[Member]) -> Self {
        Self {
            members: members.to_vec(),
        }
    }

    pub fn members(&self) -> impl Iterator<Item = &Member> {
        self.members.iter()
    }
    pub fn add_member(&mut self, member: Member) {
        self.members.push(member)
    }
    pub fn append_members(&mut self, members: &[Member]) {
        let mut members = members.to_vec();
        self.members.append(&mut members);
    }
    pub fn remove_member(&mut self, member: &Member) {
        self.members.retain(|v| v == member);
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
