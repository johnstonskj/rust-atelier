use crate::model::values::Value;
use crate::model::{Annotated, Documented, Identifier, Named, ShapeID};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub struct Member {
    id: Identifier,
    doc: Option<String>,
    traits: Vec<Trait>,
    value: Option<Value>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Trait {
    id: ShapeID,
    value: Option<Value>,
}

pub trait Valued {
    fn value(&self) -> &Option<Value>;
    fn value_mut(&mut self) -> &mut Option<Value>;
    fn set_value(&mut self, value: Value);
    fn unset_value(&mut self);
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

impl Named<Identifier> for Member {
    fn id(&self) -> &Identifier {
        &self.id
    }
}

impl Documented for Member {
    fn documentation(&self) -> &Option<String> {
        &self.doc
    }

    fn set_documentation(&mut self, documentation: &str) {
        self.doc = Some(documentation.to_owned());
    }

    fn unset_documentation(&mut self) {
        self.doc = None;
    }
}

impl Annotated for Member {
    fn has_trait(&self, id: &ShapeID) -> bool {
        self.traits.iter().any(|t| t.id() == id)
    }

    fn traits(&self) -> &Vec<Trait> {
        &self.traits
    }

    fn add_trait(&mut self, a_trait: Trait) {
        self.traits.push(a_trait);
    }

    fn remove_trait(&mut self, id: &ShapeID) {
        self.traits.retain(|t| t.id() != id);
    }
}

impl Valued for Member {
    fn value(&self) -> &Option<Value> {
        &self.value
    }

    fn value_mut(&mut self) -> &mut Option<Value> {
        &mut self.value
    }

    fn set_value(&mut self, value: Value) {
        self.value = Some(value)
    }

    fn unset_value(&mut self) {
        self.value = None
    }
}

impl Member {
    pub fn new(id: Identifier) -> Self {
        Self {
            id,
            doc: None,
            traits: Default::default(),
            value: None,
        }
    }

    pub fn with_value(id: Identifier, value: Value) -> Self {
        Self {
            id,
            doc: None,
            traits: Default::default(),
            value: Some(value),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Named<ShapeID> for Trait {
    fn id(&self) -> &ShapeID {
        &self.id
    }
}

impl Valued for Trait {
    fn value(&self) -> &Option<Value> {
        &self.value
    }

    fn value_mut(&mut self) -> &mut Option<Value> {
        &mut self.value
    }

    fn set_value(&mut self, value: Value) {
        self.value = Some(value)
    }

    fn unset_value(&mut self) {
        self.value = None
    }
}

impl Trait {
    pub fn new(id: ShapeID) -> Self {
        Self { id, value: None }
    }

    pub fn with_value(id: ShapeID, value: Value) -> Self {
        Self {
            id,
            value: Some(value),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
