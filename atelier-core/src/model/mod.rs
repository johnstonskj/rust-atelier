/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::Result;
use crate::model::shapes::{Shape, Trait};
use crate::model::statements::{Apply, Metadata};
use crate::Version;
use std::collections::HashMap;
use std::fmt::Debug;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Model {
    version: Version,
    namespace: Namespace,
    uses: Vec<ShapeID>,
    shapes: HashMap<Identifier, Shape>,
    applies: Vec<Apply>,
    metadata: Vec<Metadata>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ObjectKey {
    String(String),
    Identifier(Identifier),
}

pub trait Named<I> {
    fn id(&self) -> &I;
}

pub trait Documented {
    fn documentation(&self) -> &Option<String>;
    fn set_documentation(&mut self, documentation: &str);
    fn unset_documentation(&mut self);
}

pub trait Annotated {
    fn has_trait(&self, id: &ShapeID) -> bool;
    fn traits(&self) -> &Vec<Trait>;
    fn add_trait(&mut self, a_trait: Trait);
    fn remove_trait(&mut self, id: &ShapeID);
}

pub trait Validator {
    fn validate(model: &Model) -> Result<()>;
}

pub trait Transformer {
    fn transform(model: Model) -> Result<Model>;
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

impl Model {
    pub fn new(namespace: Namespace) -> Self {
        Self {
            version: Version::V10,
            namespace,
            uses: Default::default(),
            shapes: Default::default(),
            applies: Default::default(),
            metadata: Default::default(),
        }
    }

    pub fn version(&self) -> &Version {
        &self.version
    }

    pub fn namespace(&self) -> &Namespace {
        &self.namespace
    }

    pub fn set_namespace(&mut self, namespace: Namespace) {
        self.namespace = namespace
    }

    // --------------------------------------------------------------------------------------------

    pub fn uses(&self) -> impl Iterator<Item = &ShapeID> {
        self.uses.iter()
    }

    pub fn add_usage(&mut self, id: ShapeID) {
        self.uses.push(id);
    }

    pub fn remove_usage(&mut self, id: &ShapeID) {
        self.uses.retain(|u| u != id);
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_shape(&self, shape_id: &Identifier) -> bool {
        self.shapes.contains_key(shape_id)
    }

    pub fn shape(&self, shape_id: &Identifier) -> Option<&Shape> {
        self.shapes.get(shape_id)
    }

    pub fn shapes(&self) -> impl Iterator<Item = &Shape> {
        self.shapes.values()
    }

    pub fn add_shape(&mut self, shape: Shape) {
        self.shapes.insert(shape.id().clone(), shape);
    }

    pub fn remove_shape(&mut self, shape_id: &Identifier) {
        self.shapes.remove(shape_id);
    }

    // --------------------------------------------------------------------------------------------

    pub fn applies(&self) -> impl Iterator<Item = &Apply> {
        self.applies.iter()
    }

    pub fn add_apply(&mut self, apply: Apply) {
        self.applies.push(apply);
    }

    pub fn remove_apply(&mut self, apply: &Apply) {
        self.applies.retain(|a| a != apply);
    }

    // --------------------------------------------------------------------------------------------

    pub fn metadata(&self) -> impl Iterator<Item = &Metadata> {
        self.metadata.iter()
    }

    pub fn add_metadata(&mut self, metadata: Metadata) {
        self.metadata.push(metadata);
    }

    pub fn remove_metadata(&mut self, metadata: &Metadata) {
        self.metadata.retain(|a| a != metadata);
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod builder;

pub mod identity;
pub use identity::{Identifier, Namespace, ShapeID};

pub mod select;

pub mod shapes;

pub mod statements;

pub mod values;
