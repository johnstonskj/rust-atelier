use crate::model::values::{Number, Value};
use crate::model::ShapeID;
use std::collections::HashMap;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Builder for `NodeValue::Array` values.  This implements `From<T>` to provide the node value itself.
///
#[derive(Debug)]
pub struct ArrayBuilder {
    inner: Vec<Value>,
}

///
/// Builder for `NodeValue::Object` values.  This implements `From<T>` to provide the node value itself.
///
#[derive(Debug)]
pub struct ObjectBuilder {
    inner: HashMap<String, Value>,
}

///
/// Builder for individual `NodeValue` values.  This implements `From<T>` to provide the node value itself.
///
#[derive(Debug)]
pub struct ValueBuilder {
    inner: Value,
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

impl From<&mut ArrayBuilder> for Value {
    fn from(builder: &mut ArrayBuilder) -> Self {
        Value::Array(builder.inner.clone())
    }
}

impl From<ArrayBuilder> for Value {
    fn from(builder: ArrayBuilder) -> Self {
        Value::Array(builder.inner)
    }
}

impl ArrayBuilder {
    /// Push an element onto the end of this array.
    pub fn push(&mut self, v: Value) -> &mut Self {
        self.inner.push(v);
        self
    }

    /// Push a number-valued element onto the end of this array.
    pub fn number(&mut self, v: Number) -> &mut Self {
        let _ = self.push(Value::Number(v));
        self
    }

    /// Push a integer-valued element onto the end of this array.
    pub fn integer(&mut self, v: i64) -> &mut Self {
        let _ = self.push(Value::Number(v.into()));
        self
    }

    /// Push a float-valued element onto the end of this array.
    pub fn float(&mut self, v: f64) -> &mut Self {
        let _ = self.push(Value::Number(v.into()));
        self
    }

    /// Push a boolean-valued element onto the end of this array.
    pub fn boolean(&mut self, v: bool) -> &mut Self {
        let _ = self.push(Value::Boolean(v));
        self
    }

    /// Push a string-valued element onto the end of this array.
    pub fn string(&mut self, v: &str) -> &mut Self {
        let _ = self.push(Value::String(v.to_string()));
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

impl From<&mut ObjectBuilder> for Value {
    fn from(builder: &mut ObjectBuilder) -> Self {
        Value::Object(builder.inner.clone())
    }
}

impl From<ObjectBuilder> for Value {
    fn from(builder: ObjectBuilder) -> Self {
        Value::Object(builder.inner)
    }
}

impl ObjectBuilder {
    /// Insert the key/value pair into this object.
    pub fn insert(&mut self, k: &str, v: Value) -> &mut Self {
        let _ = self.inner.insert(k.to_string(), v);
        self
    }

    /// Insert the key and number-valued pair into this object.
    pub fn number(&mut self, k: &str, v: Number) -> &mut Self {
        let _ = self.insert(k, Value::Number(v));
        self
    }

    /// Insert the key and integer-valued pair into this object.
    pub fn integer(&mut self, k: &str, v: i64) -> &mut Self {
        let _ = self.insert(k, Value::Number(v.into()));
        self
    }

    /// Insert the key and float-valued pair into this object.
    pub fn float(&mut self, k: &str, v: f64) -> &mut Self {
        let _ = self.insert(k, Value::Number(v.into()));
        self
    }

    /// Insert the key and boolean-valued pair into this object.
    pub fn boolean(&mut self, k: &str, v: bool) -> &mut Self {
        let _ = self.insert(k, Value::Boolean(v));
        self
    }

    /// Insert the key and string-valued pair into this object.
    pub fn string(&mut self, k: &str, v: &str) -> &mut Self {
        let _ = self.insert(k, Value::String(v.to_string()));
        self
    }

    /// Insert the key and string-valued pair into this object.
    pub fn reference(&mut self, k: &str, v: &str) -> &mut Self {
        let _ = self.insert(k, Value::String(ShapeID::from_str(v).unwrap().to_string()));
        self
    }
}

// ------------------------------------------------------------------------------------------------

impl From<&mut ValueBuilder> for Value {
    fn from(builder: &mut ValueBuilder) -> Self {
        builder.inner.clone()
    }
}

impl From<ValueBuilder> for Value {
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
            inner: Value::Number(n),
        }
    }

    /// Return a new `ValueBuilder` with an integer value.
    pub fn integer(n: i64) -> Self {
        Self {
            inner: Value::Number(n.into()),
        }
    }

    /// Return a new `ValueBuilder` with a float value.
    pub fn float(n: f64) -> Self {
        Self {
            inner: Value::Number(n.into()),
        }
    }

    /// Return a new `ValueBuilder` with a boolean value.
    pub fn boolean(v: bool) -> Self {
        Self {
            inner: Value::Boolean(v),
        }
    }

    /// Return a new `ValueBuilder` with a string value.
    pub fn string(v: &str) -> Self {
        Self {
            inner: Value::String(v.to_string()),
        }
    }
}
