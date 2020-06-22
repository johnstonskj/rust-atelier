/*!
The core model itself, consisting of shapes, members, types, values, and model statements.

*/

use crate::error::Result;
use crate::model::shapes::{Shape, Trait, Valued};
use crate::Version;
use std::collections::HashMap;
use std::fmt::Debug;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct ModelOptions {}

#[derive(Clone, Debug)]
pub struct Model {
    options: Option<ModelOptions>,
    version: Option<Version>,
    namespace: Namespace,
    references: HashMap<ShapeID, Option<Rc<Model>>>,
    shapes: HashMap<Identifier, Shape>,
    applied_traits: HashMap<ShapeID, Vec<Trait>>,
    metadata: HashMap<Key, NodeValue>,
}

pub trait Named<I> {
    fn id(&self) -> &I;
}

pub trait Annotated {
    fn has_traits(&self) -> bool;
    fn has_trait(&self, id: &ShapeID) -> bool;
    fn traits(&self) -> &Vec<Trait>;
    fn add_trait(&mut self, a_trait: Trait);
    fn remove_trait(&mut self, id: &ShapeID);
    fn documentation(&mut self, doc: &str) {
        let mut doc_trait = Trait::new(ShapeID::from_str("documentation").unwrap());
        doc_trait.set_value(NodeValue::String(doc.to_string()));
        let _ = self.add_trait(doc_trait);
    }
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
            options: None,
            version: None,
            namespace,
            references: Default::default(),
            shapes: Default::default(),
            applied_traits: Default::default(),
            metadata: Default::default(),
        }
    }

    pub fn with_options(namespace: Namespace, options: ModelOptions) -> Self {
        Self {
            options: Some(options),
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

    pub fn metadata(&self) -> impl Iterator<Item = (&Key, &NodeValue)> {
        self.metadata.iter()
    }

    pub fn add_metadata(&mut self, key: Key, value: NodeValue) {
        let _ = self.metadata.insert(key, value);
    }

    pub fn remove_metadata(&mut self, key: &Key) {
        let _ = self.metadata.remove(key);
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
use std::str::FromStr;

pub mod select;

pub mod shapes;

pub mod values;
