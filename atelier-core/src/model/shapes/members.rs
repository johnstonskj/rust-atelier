use crate::model::values::NodeValue;
use crate::model::{Annotated, Documented, Identifier, Named, ShapeID};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub struct Member {
    id: Identifier,
    doc: Option<String>,
    traits: Vec<Trait>,
    value: Option<NodeValue>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Trait {
    id: ShapeID,
    value: Option<NodeValue>,
}

pub trait Valued {
    fn value(&self) -> &Option<NodeValue>;

    fn value_mut(&mut self) -> &mut Option<NodeValue>;

    fn has_value(&self) -> bool {
        self.value().is_some()
    }

    fn set_value(&mut self, value: NodeValue) {
        *self.value_mut() = Some(value)
    }

    fn unset_value(&mut self) {
        *self.value_mut() = None
    }
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
    fn value(&self) -> &Option<NodeValue> {
        &self.value
    }

    fn value_mut(&mut self) -> &mut Option<NodeValue> {
        &mut self.value
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

    pub fn with_value(id: Identifier, value: NodeValue) -> Self {
        Self {
            id,
            doc: None,
            traits: Default::default(),
            value: Some(value),
        }
    }

    pub fn with_reference(id: Identifier, ref_id: ShapeID) -> Self {
        Self {
            id,
            doc: None,
            traits: Default::default(),
            value: Some(NodeValue::ShapeID(ref_id)),
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
    fn value(&self) -> &Option<NodeValue> {
        &self.value
    }

    fn value_mut(&mut self) -> &mut Option<NodeValue> {
        &mut self.value
    }

    fn set_value(&mut self, value: NodeValue) {
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

    pub fn with_value(id: ShapeID, value: NodeValue) -> Self {
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
