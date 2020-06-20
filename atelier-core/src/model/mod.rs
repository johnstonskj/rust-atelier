/*!
The core model itself, consisting of shapes, members, types, values, and model statements.

*/

use crate::error::Result;
use crate::model::shapes::{Shape, Trait};
use crate::Version;
use std::collections::HashMap;
use std::fmt::Debug;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Model {
    version: Option<Version>,
    namespace: Namespace,
    references: HashMap<ShapeID, Option<Rc<Model>>>,
    shapes: HashMap<Identifier, Shape>,
    applied_traits: HashMap<ShapeID, Vec<Trait>>,
    metadata: HashMap<Key, Vec<NodeValue>>,
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
// Implementations
// ------------------------------------------------------------------------------------------------

impl Model {
    pub fn new(namespace: Namespace) -> Self {
        Self {
            version: None,
            namespace,
            references: Default::default(),
            shapes: Default::default(),
            applied_traits: Default::default(),
            metadata: Default::default(),
        }
    }

    pub fn version(&self) -> &Option<Version> {
        &self.version
    }

    pub fn set_version(&mut self, version: Version) {
        self.version = Some(version);
    }

    // --------------------------------------------------------------------------------------------

    pub fn namespace(&self) -> &Namespace {
        &self.namespace
    }

    pub fn set_namespace(&mut self, namespace: Namespace) {
        self.namespace = namespace
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_references(&self) -> bool {
        !self.references.is_empty()
    }

    pub fn references(&self) -> impl Iterator<Item = &ShapeID> {
        self.references.keys()
    }

    pub fn add_reference(&mut self, id: ShapeID) {
        let _ = self.references.insert(id, None);
    }

    pub fn add_reference_from(&mut self, id: ShapeID, from_model: Rc<Model>) {
        let _ = self.references.insert(id, Some(from_model));
    }

    pub fn remove_reference(&mut self, id: &ShapeID) {
        let _ = self.references.remove(id);
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_shapes(&self) -> bool {
        !self.shapes.is_empty()
    }

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
        let _ = self.shapes.insert(shape.id().clone(), shape);
    }

    pub fn remove_shape(&mut self, shape_id: &Identifier) {
        let _ = self.shapes.remove(shape_id);
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_trait_applies(&self) -> bool {
        !self.applied_traits.is_empty()
    }

    pub fn all_applied_traits(&self) -> impl Iterator<Item = (&ShapeID, &Vec<Trait>)> {
        self.applied_traits.iter()
    }

    pub fn apply_trait_to(&mut self, a_trait: Trait, id: ShapeID) {
        match self.applied_traits.get_mut(&id) {
            None => {
                let _ = self.applied_traits.insert(id, vec![a_trait]);
            }
            Some(vec) => {
                vec.push(a_trait);
            }
        }
    }

    pub fn remove_trait_from(&mut self, a_trait: &Trait, id: &ShapeID) {
        if let Some(traits) = self.applied_traits.get_mut(id) {
            traits.retain(|t| t != a_trait);
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_metadata(&self) -> bool {
        !self.metadata.is_empty()
    }

    pub fn metadata(&self) -> impl Iterator<Item = (&Key, &Vec<NodeValue>)> {
        self.metadata.iter()
    }

    pub fn add_metadata(&mut self, key: Key, value: NodeValue) {
        match self.metadata.get_mut(&key) {
            None => {
                let _ = self.metadata.insert(key, vec![value]);
            }
            Some(vec) => {
                vec.push(value);
            }
        }
    }

    pub fn remove_metadata_for(&mut self, key: &Key) {
        let _ = self.metadata.remove(key);
    }

    pub fn merge_metadata(&mut self) {
        unimplemented!()
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod builder;

#[doc(hidden)]
pub mod identity;
use crate::model::values::{Key, NodeValue};
pub use identity::{Identifier, Namespace, ShapeID};
use std::rc::Rc;

pub mod select;

pub mod shapes;

pub mod values;
