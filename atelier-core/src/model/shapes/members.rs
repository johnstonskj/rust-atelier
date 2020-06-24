use crate::model::values::NodeValue;
use crate::model::{Annotated, Identifier, Named, ShapeID};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A member of an aggregate, or service, shape. A member has an identity, may have traits, and
/// may also have a node value.
///
#[derive(Clone, Debug, PartialEq)]
pub struct Member {
    id: Identifier,
    traits: Vec<Trait>,
    value: Option<NodeValue>,
}

///
/// A Trait applied to a shape or member including any value associated with the trait for this
/// instance.
///
#[derive(Clone, Debug, PartialEq)]
pub struct Trait {
    id: ShapeID,
    value: Option<NodeValue>,
}

///
/// A trait that denotes a shape or statement that has a node value.
///
pub trait Valued {
    /// Returns `true` if there is a node value set, else `false`.
    fn has_value(&self) -> bool {
        self.value().is_some()
    }

    /// Return a reference to the current value, if set.
    fn value(&self) -> &Option<NodeValue>;

    /// Return a mutable reference to the current value, if set.
    fn value_mut(&mut self) -> &mut Option<NodeValue>;

    /// Set the current node value.
    fn set_value(&mut self, value: NodeValue) {
        *self.value_mut() = Some(value)
    }

    /// Set the current node value to `None`.
    fn unset_value(&mut self) {
        *self.value_mut() = None
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Named<Identifier> for Member {
    fn id(&self) -> &Identifier {
        &self.id
    }
}

impl Annotated for Member {
    fn has_traits(&self) -> bool {
        !self.traits.is_empty()
    }

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
    /// Construct a new member with the given identity and no value.
    pub fn new(id: Identifier) -> Self {
        Self {
            id,
            traits: Default::default(),
            value: None,
        }
    }

    /// Construct a new member with the given identity and value.
    pub fn with_value(id: Identifier, value: NodeValue) -> Self {
        Self {
            id,
            traits: Default::default(),
            value: Some(value),
        }
    }

    /// Construct a new member with the given identity and a value which is a shape reference.
    pub fn with_reference(id: Identifier, ref_id: ShapeID) -> Self {
        Self {
            id,
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
    /// Construct a new trait with the given identity and no value.
    pub fn new(id: ShapeID) -> Self {
        Self { id, value: None }
    }

    /// Construct a new trait with the given identity and value.
    pub fn with_value(id: ShapeID, value: NodeValue) -> Self {
        Self {
            id,
            value: Some(value),
        }
    }
}
