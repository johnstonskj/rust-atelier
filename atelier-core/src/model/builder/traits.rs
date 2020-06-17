/*!
One-line description.

More detailed description, with

# Example

*/

use crate::model::shapes::{Trait, Valued};
use crate::model::values::Value;
use crate::model::{Identifier, ShapeID};
use std::collections::HashMap;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub struct TraitBuilder {
    a_trait: Trait,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn is_boxed() -> Trait {
    Trait::new(ShapeID::from_str("box").unwrap())
}

pub fn has_length(min: usize, max: usize) -> Trait {
    let value: HashMap<Identifier, Value> = [
        (
            Identifier::from_str("min").unwrap(),
            Value::Integer(min as i32),
        ),
        (
            Identifier::from_str("max").unwrap(),
            Value::Integer(max as i32),
        ),
    ]
    .iter()
    .cloned()
    .collect();
    Trait::with_value(ShapeID::from_str("length").unwrap(), Value::Map(value))
}

pub fn has_pattern(pat: &str) -> Trait {
    Trait::with_value(
        ShapeID::from_str("pattern").unwrap(),
        Value::String(pat.to_string()),
    )
}

pub fn is_private() -> Trait {
    Trait::new(ShapeID::from_str("private").unwrap())
}

pub fn is_readonly() -> Trait {
    Trait::new(ShapeID::from_str("readonly").unwrap())
}

pub fn is_required() -> Trait {
    Trait::new(ShapeID::from_str("required").unwrap())
}

pub fn is_trait() -> Trait {
    Trait::new(ShapeID::from_str("trait").unwrap())
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl TraitBuilder {
    pub fn new(id: &str) -> Self {
        Self {
            a_trait: Trait::new(ShapeID::from_str(id).unwrap()),
        }
    }

    pub fn add(&mut self, value: Value) {
        self.a_trait.set_value(value)
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
