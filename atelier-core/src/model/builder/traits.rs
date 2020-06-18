use crate::model::shapes::{Trait, Valued};
use crate::model::values::Value;
use crate::model::{Identifier, ShapeID};
use std::collections::HashMap;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct TraitBuilder {
    a_trait: Trait,
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

impl TraitBuilder {
    pub fn boxed() -> Self {
        Self::new("box")
    }

    pub fn deprecated(message: Option<&str>, since: Option<&str>) -> Self {
        let mut value: HashMap<Identifier, Value> = Default::default();
        if let Some(message) = message {
            value.insert(
                Identifier::from_str("message").unwrap(),
                Value::String(message.to_string()),
            );
        }
        if let Some(since) = since {
            value.insert(
                Identifier::from_str("since").unwrap(),
                Value::String(since.to_string()),
            );
        }
        Self::with_value("deprecated", Value::RefMap(value))
    }

    pub fn documentation(value: &str) -> Self {
        let mut result = Self::new("deprecated");
        result.string(value);
        result
    }

    pub fn external_documentation(map: &[(&str, &str)]) -> Self {
        let value = map
            .into_iter()
            .map(|(k, v)| (k.to_string(), Value::String(v.to_string())))
            .collect();
        Self::with_value("externalDocumentation", Value::Map(value))
    }

    pub fn length(min: Option<usize>, max: Option<usize>) -> Self {
        assert!(min.is_some() || max.is_some());
        let mut value: HashMap<Identifier, Value> = Default::default();
        if let Some(min) = min {
            value.insert(
                Identifier::from_str("min").unwrap(),
                Value::Integer(min as i32),
            );
        }
        if let Some(max) = max {
            value.insert(
                Identifier::from_str("max").unwrap(),
                Value::Integer(max as i32),
            );
        }
        Self::with_value("length", Value::RefMap(value))
    }

    pub fn has_pattern(pat: &str) -> Self {
        assert!(!pat.is_empty());
        Self::new("pattern").string(pat).clone()
    }

    pub fn private() -> Self {
        Self::new("private")
    }

    pub fn readonly() -> Self {
        Self::new("readonly")
    }

    pub fn required() -> Self {
        Self::new("required")
    }

    pub fn sensitive() -> Self {
        Self::new("sensitive")
    }

    pub fn since(date: &str) -> Self {
        assert!(!date.is_empty());
        Self::new("since").string(date).clone()
    }

    pub fn tagged(tags: &[&str]) -> Self {
        Self::with_value(
            "tags",
            Value::List(tags.iter().map(|s| Value::String(s.to_string())).collect()),
        )
    }

    pub fn is_trait() -> Self {
        Self::new("trait")
    }

    pub fn new(id: &str) -> Self {
        Self {
            a_trait: Trait::new(ShapeID::from_str(id).unwrap()),
        }
    }

    pub fn with_value(id: &str, value: Value) -> Self {
        Self {
            a_trait: Trait::with_value(ShapeID::from_str(id).unwrap(), value),
        }
    }

    pub fn boolean(&mut self, value: bool) -> &mut Self {
        self.value(Value::Boolean(value))
    }

    pub fn string(&mut self, value: &str) -> &mut Self {
        self.value(Value::String(value.to_string()))
    }

    pub fn byte(&mut self, value: i8) -> &mut Self {
        self.value(Value::Byte(value))
    }

    pub fn short(&mut self, value: i16) -> &mut Self {
        self.value(Value::Short(value))
    }

    pub fn integer(&mut self, value: i32) -> &mut Self {
        self.value(Value::Integer(value))
    }

    pub fn long(&mut self, value: i64) -> &mut Self {
        self.value(Value::Long(value))
    }

    pub fn float(&mut self, value: f32) -> &mut Self {
        self.value(Value::Float(value))
    }

    pub fn double(&mut self, value: f64) -> &mut Self {
        self.value(Value::Double(value))
    }

    pub fn value(&mut self, value: Value) -> &mut Self {
        self.a_trait.set_value(value);
        self
    }

    pub fn build(&self) -> Trait {
        self.a_trait.clone()
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
