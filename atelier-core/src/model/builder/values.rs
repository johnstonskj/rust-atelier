use crate::model::values::{Key, NodeValue, Number};
use crate::model::ShapeID;
use std::collections::HashMap;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Builder for `NodeValue::Array` values.  This implements `From<T>` to provide the node value itself.
///
#[derive(Debug)]
pub struct ArrayBuilder {
    inner: Vec<NodeValue>,
}

///
/// Builder for `NodeValue::Object` values.  This implements `From<T>` to provide the node value itself.
///
#[derive(Debug)]
pub struct ObjectBuilder {
    inner: HashMap<Key, NodeValue>,
}

///
/// Builder for individual `NodeValue` values.  This implements `From<T>` to provide the node value itself.
///
#[derive(Debug)]
pub struct ValueBuilder {
    inner: NodeValue,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for ArrayBuilder {
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl From<&mut ArrayBuilder> for NodeValue {
    fn from(builder: &mut ArrayBuilder) -> Self {
        NodeValue::Array(builder.inner.clone())
    }
}

impl From<ArrayBuilder> for NodeValue {
    fn from(builder: ArrayBuilder) -> Self {
        NodeValue::Array(builder.inner)
    }
}

impl ArrayBuilder {
    /// Push an element onto the end of this array.
    pub fn push(&mut self, v: NodeValue) -> &mut Self {
        self.inner.push(v);
        self
    }

    /// Push a number-valued element onto the end of this array.
    pub fn number(&mut self, v: Number) -> &mut Self {
        let _ = self.push(NodeValue::Number(v));
        self
    }

    /// Push a integer-valued element onto the end of this array.
    pub fn integer(&mut self, v: i64) -> &mut Self {
        let _ = self.push(NodeValue::Number(v.into()));
        self
    }

    /// Push a float-valued element onto the end of this array.
    pub fn float(&mut self, v: f64) -> &mut Self {
        let _ = self.push(NodeValue::Number(v.into()));
        self
    }

    /// Push a boolean-valued element onto the end of this array.
    pub fn boolean(&mut self, v: bool) -> &mut Self {
        let _ = self.push(NodeValue::Boolean(v));
        self
    }

    /// Push a shape_id-valued element onto the end of this array.
    pub fn reference(&mut self, v: ShapeID) -> &mut Self {
        let _ = self.push(NodeValue::ShapeID(v));
        self
    }

    /// Push a text_block-valued element onto the end of this array.
    pub fn text_block(&mut self, v: &str) -> &mut Self {
        let _ = self.push(NodeValue::TextBlock(v.to_string()));
        self
    }

    /// Push a string-valued element onto the end of this array.
    pub fn string(&mut self, v: &str) -> &mut Self {
        let _ = self.push(NodeValue::String(v.to_string()));
        self
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for ObjectBuilder {
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl From<&mut ObjectBuilder> for NodeValue {
    fn from(builder: &mut ObjectBuilder) -> Self {
        NodeValue::Object(builder.inner.clone())
    }
}

impl From<ObjectBuilder> for NodeValue {
    fn from(builder: ObjectBuilder) -> Self {
        NodeValue::Object(builder.inner)
    }
}

impl ObjectBuilder {
    /// Insert the key/value pair into this object.
    pub fn insert(&mut self, k: Key, v: NodeValue) -> &mut Self {
        let _ = self.inner.insert(k, v);
        self
    }

    /// Insert the key and number-valued pair into this object.
    pub fn number(&mut self, k: Key, v: Number) -> &mut Self {
        let _ = self.insert(k, NodeValue::Number(v));
        self
    }

    /// Insert the key and integer-valued pair into this object.
    pub fn integer(&mut self, k: Key, v: i64) -> &mut Self {
        let _ = self.insert(k, NodeValue::Number(v.into()));
        self
    }

    /// Insert the key and float-valued pair into this object.
    pub fn float(&mut self, k: Key, v: f64) -> &mut Self {
        let _ = self.insert(k, NodeValue::Number(v.into()));
        self
    }

    /// Insert the key and boolean-valued pair into this object.
    pub fn boolean(&mut self, k: Key, v: bool) -> &mut Self {
        let _ = self.insert(k, NodeValue::Boolean(v));
        self
    }

    /// Insert the key and shape_id-valued pair into this object.
    pub fn reference(&mut self, k: Key, v: ShapeID) -> &mut Self {
        let _ = self.insert(k, NodeValue::ShapeID(v));
        self
    }

    /// Insert the key and text_block-valued pair into this object.
    pub fn text_block(&mut self, k: Key, v: &str) -> &mut Self {
        let _ = self.insert(k, NodeValue::TextBlock(v.to_string()));
        self
    }

    /// Insert the key and string-valued pair into this object.
    pub fn string(&mut self, k: Key, v: &str) -> &mut Self {
        let _ = self.insert(k, NodeValue::String(v.to_string()));
        self
    }
}

// ------------------------------------------------------------------------------------------------

impl From<&mut ValueBuilder> for NodeValue {
    fn from(builder: &mut ValueBuilder) -> Self {
        builder.inner.clone()
    }
}

impl From<ValueBuilder> for NodeValue {
    fn from(builder: ValueBuilder) -> Self {
        builder.inner
    }
}

impl ValueBuilder {
    /// Return a new `ArrayBuilder`.
    pub fn array() -> ArrayBuilder {
        Default::default()
    }

    /// Return a new `ObjectBuilder`.
    pub fn object() -> ObjectBuilder {
        Default::default()
    }

    /// Return a new `ValueBuilder` with a number value.
    pub fn number(n: Number) -> Self {
        Self {
            inner: NodeValue::Number(n),
        }
    }

    /// Return a new `ValueBuilder` with an integer value.
    pub fn integer(n: i64) -> Self {
        Self {
            inner: NodeValue::Number(n.into()),
        }
    }

    /// Return a new `ValueBuilder` with a float value.
    pub fn float(n: f64) -> Self {
        Self {
            inner: NodeValue::Number(n.into()),
        }
    }

    /// Return a new `ValueBuilder` with a boolean value.
    pub fn boolean(v: bool) -> Self {
        Self {
            inner: NodeValue::Boolean(v),
        }
    }

    /// Return a new `ValueBuilder` with a shape_id value.
    pub fn reference(v: ShapeID) -> Self {
        Self {
            inner: NodeValue::ShapeID(v),
        }
    }

    /// Return a new `ValueBuilder` with a text_block value.
    pub fn text_block(v: &str) -> Self {
        Self {
            inner: NodeValue::TextBlock(v.to_string()),
        }
    }

    /// Return a new `ValueBuilder` with a string value.
    pub fn string(v: &str) -> Self {
        Self {
            inner: NodeValue::String(v.to_string()),
        }
    }
}
