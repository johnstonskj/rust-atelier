/*!
Model structures for values, these are used to capture the right-hand side of member declarations
within shapes, but only shape IDs, as well as the values provided to trait applications and metadata
statements.
*/

use crate::model::{Identifier, ShapeID};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The key to a `NodeValue::Object` value, which may be either a quoted string or an identifier.
///
/// Corresponds to the `node_object_key` production in §2.5,
///   [Node values](https://awslabs.github.io/smithy/1.0/spec/core/lexical-structure.html#node-values),
///   of the Smithy 1.0 Specification.
///
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Key {
    /// A `quoted_text` string (see [String values](https://awslabs.github.io/smithy/1.0/spec/core/lexical-structure.html#string-values)).
    String(String),
    /// An identifier value (see [Shape ID ABNF](https://awslabs.github.io/smithy/1.0/spec/core/lexical-structure.html#shape-id-abnf)).
    Identifier(Identifier),
}

///
/// The Smithy specification deals with numbers that are representable in JSON, such a production
/// does not distinguish between integer, decimal, or floating point values however Rust cares.
///
/// Corresponds to the `number` production in §2.5,
///   [Node values](https://awslabs.github.io/smithy/1.0/spec/core/lexical-structure.html#node-values),
///   of the Smithy 1.0 Specification.
///
#[derive(Clone, Debug, PartialEq)]
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
pub enum NodeValue {
    /// An array (Smithy list or set) of other node values.
    Array(Vec<NodeValue>),
    /// An object (Smithy structure) mapping keys to other node values.
    Object(HashMap<Key, NodeValue>),
    /// A numeric value, either integer or float.
    Number(Number),
    /// A boolean value. Corresponds to "true" and "false" in the `node_keywords` production.
    Boolean(bool),
    /// A `ShapeID` which implies a reference to another shape.
    ShapeID(ShapeID),
    /// A block of text between three double quotes `"""`, corresponding to the `text_block` production.
    TextBlock(String),
    /// A quoted string, between double quotes `"`, corresponding to the `quoted_text` production.
    String(String),
}

// ------------------------------------------------------------------------------------------------
// Macros
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
macro_rules! is_as {
    ($is_fn:ident, $as_fn:ident, $variant:ident, $ret_type:ty) => {
        /// Returns `true` if `self` is the corresponding variant, else `false`.
        pub fn $is_fn(&self) -> bool {
            match self {
                Self::$variant(_) => true,
                _ => false,
            }
        }

        /// Returns `Some(v)` if `self` is the corresponding variant, else `None`.
        pub fn $as_fn(&self) -> Option<&$ret_type> {
            match self {
                Self::$variant(v) => Some(v),
                _ => None,
            }
        }
    };
}

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

impl Key {
    is_as! { is_string, as_string, String, String }

    is_as! { is_identifier, as_identifier, Identifier, Identifier }
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

impl From<Number> for NodeValue {
    fn from(n: Number) -> Self {
        Self::Number(n)
    }
}

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
            NodeValue::Array(vs) => write!(
                f,
                "[ {} ]",
                vs.iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            NodeValue::Object(vs) => write!(
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

impl NodeValue {
    ///
    /// Construct a new `NodeValue` as a reference to another shape. This is added as a convenience
    /// where it makes clear the usage of the `NodeValue` as an explicit reference.
    ///
    pub fn reference(shape_id: ShapeID) -> Self {
        Self::ShapeID(shape_id)
    }

    is_as! { is_array, as_array, Array, Vec<NodeValue> }

    is_as! { is_object, as_object, Object, HashMap<Key, NodeValue> }

    is_as! { is_number, as_number, Number, Number }

    is_as! { is_boolean, as_boolean, Boolean, bool }

    is_as! { is_shape_id, as_shape_id, ShapeID, ShapeID }

    is_as! { is_reference, as_reference, ShapeID, ShapeID }

    is_as! { is_text_block, as_text_block, TextBlock, String }

    is_as! { is_string, as_string, String, String }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
