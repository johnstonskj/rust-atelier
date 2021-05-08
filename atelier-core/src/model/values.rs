/*!
Model structures for values, these are used to capture the right-hand side of member declarations
within shapes, but only shape IDs, as well as the values provided to trait applications and metadata
statements.
*/

use crate::model::ShapeID;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The Smithy specification deals with numbers that are representable in JSON, such a production
/// does not distinguish between integer, decimal, or floating point values however Rust cares.
///
/// Corresponds to the `number` production in §2.5,
///   [Node values](https://awslabs.github.io/smithy/1.0/spec/core/lexical-structure.html#node-values),
///   of the Smithy 1.0 Specification.
///
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Number {
    /// An integer value
    Integer(i64),
    /// A floating point value
    Float(f64),
}

///
/// A Node value is used to carry specific data items; it is a key field in the [`Member`](../shapes/struct.Member.html)
/// structure for example. The `node_keywords` production also includes the value "null", where
/// nullable values are necessary they are represented as `Option<NodeValue>`.
///
/// Corresponds to the `node_value` production in §2.5,
///   [Node values](https://awslabs.github.io/smithy/1.0/spec/core/lexical-structure.html#node-values),
///   of the Smithy 1.0 Specification.
///
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    /// An array (Smithy list or set) of other node values.
    Array(Vec<Value>),
    /// An object (Smithy structure) mapping keys to other node values.
    Object(ValueMap),
    /// A numeric value, either integer or float.
    Number(Number),
    /// A boolean value. Corresponds to "true" and "false" in the `node_keywords` production.
    Boolean(bool),
    /// A quoted string, between double quotes `"`, corresponding to the `quoted_text` production.
    String(String),
    /// An empty, non-existent, value.
    None,
}

/// The type of an Object value.
pub type ValueMap = HashMap<String, Value>;

// ------------------------------------------------------------------------------------------------
// Implementations
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

impl From<Number> for Value {
    fn from(n: Number) -> Self {
        Self::Number(n)
    }
}

impl From<i8> for Value {
    fn from(n: i8) -> Self {
        Self::Number((n as i64).into())
    }
}

impl From<i16> for Value {
    fn from(n: i16) -> Self {
        Self::Number((n as i64).into())
    }
}

impl From<i32> for Value {
    fn from(n: i32) -> Self {
        Self::Number((n as i64).into())
    }
}

impl From<i64> for Value {
    fn from(n: i64) -> Self {
        Self::Number(n.into())
    }
}

impl From<f32> for Value {
    fn from(n: f32) -> Self {
        Self::Number((n as f64).into())
    }
}

impl From<f64> for Value {
    fn from(n: f64) -> Self {
        Self::Number(n.into())
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Self::Boolean(b)
    }
}

impl From<String> for Value {
    fn from(v: String) -> Self {
        Self::String(v)
    }
}

impl From<&str> for Value {
    fn from(v: &str) -> Self {
        Self::String(v.to_string())
    }
}

impl From<ShapeID> for Value {
    fn from(v: ShapeID) -> Self {
        Self::String(v.to_string())
    }
}

impl From<&ShapeID> for Value {
    fn from(v: &ShapeID) -> Self {
        Self::String(v.to_string())
    }
}

impl From<Vec<Value>> for Value {
    fn from(v: Vec<Value>) -> Self {
        Self::Array(v)
    }
}

impl From<&[Value]> for Value {
    fn from(v: &[Value]) -> Self {
        Self::from(v.to_vec())
    }
}

impl From<HashMap<String, Value>> for Value {
    fn from(v: HashMap<String, Value>) -> Self {
        Self::Object(v)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Array(vs) => write!(
                f,
                "[ {} ]",
                vs.iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Value::Object(vs) => write!(
                f,
                "{{ {} }}",
                vs.iter()
                    .map(|(k, v)| format!("{}: {}", k, v.to_string()))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Value::Number(v) => write!(f, "{}", v),
            Value::Boolean(v) => write!(f, "{}", v),
            Value::String(v) => write!(f, "\"{}\"", v),
            Value::None => write!(f, "None"),
        }
    }
}

impl Value {
    is_as! { array, Array, Vec<Value> }

    is_as! { object, Object, HashMap<String, Value> }

    is_as! { number, Number, Number }

    is_as! { boolean, Boolean, bool }

    is_as! { string, String, String }

    is_only! { none, None }
}
