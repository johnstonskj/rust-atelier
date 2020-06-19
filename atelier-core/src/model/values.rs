/*!
Model structures for values.
*/

use crate::model::{Identifier, ShapeID};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Key {
    String(String),
    Identifier(Identifier),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Number {
    Integer(i64),
    Float(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum NodeValue {
    Array(Vec<NodeValue>),
    Object(HashMap<Key, NodeValue>),
    Number(Number),
    Boolean(bool),
    ShapeID(ShapeID),
    TextBlock(String),
    String(String),
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

impl From<String> for Key {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl From<&str> for Key {
    fn from(s: &str) -> Self {
        Self::String(s.to_string())
    }
}

impl From<Identifier> for Key {
    fn from(id: Identifier) -> Self {
        Self::Identifier(id)
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(s) => write!(f, "{:?}", s),
            Self::Identifier(id) => write!(f, "{}", id),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl From<i8> for Number {
    fn from(n: i8) -> Self {
        Self::Integer(n as i64)
    }
}

impl From<i16> for Number {
    fn from(n: i16) -> Self {
        Self::Integer(n as i64)
    }
}

impl From<i32> for Number {
    fn from(n: i32) -> Self {
        Self::Integer(n as i64)
    }
}

impl From<i64> for Number {
    fn from(n: i64) -> Self {
        Self::Integer(n)
    }
}

impl From<f32> for Number {
    fn from(n: f32) -> Self {
        Self::Float(n as f64)
    }
}

impl From<f64> for Number {
    fn from(n: f64) -> Self {
        Self::Float(n)
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer(n) => write!(f, "{}", n),
            Self::Float(n) => write!(f, "{}", n),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl From<i8> for NodeValue {
    fn from(n: i8) -> Self {
        Self::Number((n as i64).into())
    }
}

impl From<i16> for NodeValue {
    fn from(n: i16) -> Self {
        Self::Number((n as i64).into())
    }
}

impl From<i32> for NodeValue {
    fn from(n: i32) -> Self {
        Self::Number((n as i64).into())
    }
}

impl From<i64> for NodeValue {
    fn from(n: i64) -> Self {
        Self::Number(n.into())
    }
}

impl From<f32> for NodeValue {
    fn from(n: f32) -> Self {
        Self::Number((n as f64).into())
    }
}

impl From<f64> for NodeValue {
    fn from(n: f64) -> Self {
        Self::Number(n.into())
    }
}

impl From<bool> for NodeValue {
    fn from(b: bool) -> Self {
        Self::Boolean(b)
    }
}

impl From<ShapeID> for NodeValue {
    fn from(id: ShapeID) -> Self {
        Self::ShapeID(id)
    }
}

impl From<String> for NodeValue {
    fn from(v: String) -> Self {
        Self::String(v)
    }
}

impl From<Vec<NodeValue>> for NodeValue {
    fn from(v: Vec<NodeValue>) -> Self {
        Self::Array(v)
    }
}

impl From<&[NodeValue]> for NodeValue {
    fn from(v: &[NodeValue]) -> Self {
        Self::Array(v.to_vec())
    }
}

impl From<HashMap<Key, NodeValue>> for NodeValue {
    fn from(v: HashMap<Key, NodeValue>) -> Self {
        Self::Object(v)
    }
}

impl Display for NodeValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeValue::Array(vs) => writeln!(
                f,
                "[ {} ]",
                vs.iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            NodeValue::Object(vs) => writeln!(
                f,
                "{{ {} }}",
                vs.iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            NodeValue::Number(v) => write!(f, "{}", v),
            NodeValue::Boolean(v) => write!(f, "{}", v),
            NodeValue::ShapeID(v) => write!(f, "{}", v),
            NodeValue::TextBlock(v) => write!(f, "\"\"\"{}\"\"\"", v),
            NodeValue::String(v) => write!(f, "\"{}\"", v),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
