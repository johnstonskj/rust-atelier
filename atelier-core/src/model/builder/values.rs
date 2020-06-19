use crate::model::values::{Key, NodeValue, Number};
use crate::model::ShapeID;
use std::collections::HashMap;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct ArrayBuilder {
    inner: Vec<NodeValue>,
}

#[derive(Clone, Debug)]
pub struct ObjectBuilder {
    inner: HashMap<Key, NodeValue>,
}

#[derive(Clone, Debug)]
pub struct ValueBuilder {
    inner: NodeValue,
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

impl Default for ArrayBuilder {
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl ArrayBuilder {
    pub fn push(&mut self, v: NodeValue) -> &mut Self {
        self.inner.push(v);
        self
    }

    pub fn number(&mut self, v: Number) -> &mut Self {
        self.push(NodeValue::Number(v))
    }

    pub fn integer(&mut self, v: i64) -> &mut Self {
        self.push(NodeValue::Number(v.into()));
        self
    }

    pub fn float(&mut self, v: f64) -> &mut Self {
        self.push(NodeValue::Number(v.into()));
        self
    }

    pub fn boolean(&mut self, v: bool) -> &mut Self {
        self.push(NodeValue::Boolean(v));
        self
    }

    pub fn reference(&mut self, v: ShapeID) -> &mut Self {
        self.push(NodeValue::ShapeID(v));
        self
    }

    pub fn text_block(&mut self, v: &str) -> &mut Self {
        self.push(NodeValue::TextBlock(v.to_string()));
        self
    }

    pub fn string(&mut self, v: &str) -> &mut Self {
        self.push(NodeValue::String(v.to_string()));
        self
    }

    pub fn build(&self) -> NodeValue {
        NodeValue::Array(self.inner.clone())
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

impl ObjectBuilder {
    pub fn insert(&mut self, k: Key, v: NodeValue) -> &mut Self {
        self.inner.insert(k, v);
        self
    }

    pub fn number(&mut self, k: Key, v: Number) -> &mut Self {
        self.insert(k, NodeValue::Number(v))
    }

    pub fn integer(&mut self, k: Key, v: i64) -> &mut Self {
        self.insert(k, NodeValue::Number(v.into()));
        self
    }

    pub fn float(&mut self, k: Key, v: f64) -> &mut Self {
        self.insert(k, NodeValue::Number(v.into()));
        self
    }

    pub fn boolean(&mut self, k: Key, v: bool) -> &mut Self {
        self.insert(k, NodeValue::Boolean(v));
        self
    }

    pub fn reference(&mut self, k: Key, v: ShapeID) -> &mut Self {
        self.insert(k, NodeValue::ShapeID(v));
        self
    }

    pub fn text_block(&mut self, k: Key, v: &str) -> &mut Self {
        self.insert(k, NodeValue::TextBlock(v.to_string()));
        self
    }

    pub fn string(&mut self, k: Key, v: &str) -> &mut Self {
        self.insert(k, NodeValue::String(v.to_string()));
        self
    }

    pub fn build(&self) -> NodeValue {
        NodeValue::Object(self.inner.clone())
    }
}

// ------------------------------------------------------------------------------------------------

impl ValueBuilder {
    pub fn array() -> ArrayBuilder {
        Default::default()
    }

    pub fn object() -> ObjectBuilder {
        Default::default()
    }

    pub fn number(n: Number) -> Self {
        Self {
            inner: NodeValue::Number(n),
        }
    }

    pub fn integer(n: i64) -> Self {
        Self {
            inner: NodeValue::Number(n.into()),
        }
    }

    pub fn float(n: f64) -> Self {
        Self {
            inner: NodeValue::Number(n.into()),
        }
    }

    pub fn boolean(v: bool) -> Self {
        Self {
            inner: NodeValue::Boolean(v),
        }
    }

    pub fn reference(v: ShapeID) -> Self {
        Self {
            inner: NodeValue::ShapeID(v),
        }
    }

    pub fn text_block(v: &str) -> Self {
        Self {
            inner: NodeValue::TextBlock(v.to_string()),
        }
    }

    pub fn string(v: &str) -> Self {
        Self {
            inner: NodeValue::String(v.to_string()),
        }
    }

    pub fn build(&self) -> NodeValue {
        self.inner.clone()
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
