/*!
The Smithy semantic model itself, consisting of shapes, members, values, and model statements.

For more information, see [the Rust Atelier book](https://rust-atelier.dev/using/model_api.html).
*/

use crate::error::{ErrorKind, Result as ModelResult};
use crate::model::shapes::TopLevelShape;
use crate::model::values::{Value, ValueMap};
use crate::Version;
use std::collections::HashMap;
use std::fmt::Debug;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The core model structure, this corresponds to a single Smithy file according to the
/// specification. It contains:
///
/// * Optionally, the version of Smithy it conforms to.
/// * Any metadata associated with the model (with the `metadata` statement).
/// * A map of shapes declared by the model.
///
#[derive(Clone, Debug)]
pub struct Model {
    pub(crate) smithy_version: Version,
    pub(crate) metadata: ValueMap,
    pub(crate) shapes: HashMap<ShapeID, TopLevelShape>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for Model {
    fn default() -> Self {
        Self::new(Version::current())
    }
}

impl Model {
    /// Create a new model using the definition of Smithy with the given version.
    pub fn new(smithy_version: Version) -> Self {
        Self {
            smithy_version,
            metadata: Default::default(),
            shapes: Default::default(),
        }
    }

    // --------------------------------------------------------------------------------------------

    ///
    /// Merge the other model into this one. This follows the rules set out in section 20,
    /// [Merging models](https://awslabs.github.io/smithy/1.0/spec/core/merging-models.html) of
    /// the Smithy specification.
    ///
    /// > _Smithy models MAY be divided into multiple files so that they are easier to maintain and
    /// > evolve. Smithy tools MUST take the following steps to merge two models together to form a
    /// > composite model_:
    /// >
    /// > * _Assert that both models use a version that is compatible with the tool versions specified_.
    /// > * _Duplicate shape names, if found, MUST cause the model merge to fail. See Shape ID
    /// >   conflicts for more information_.
    /// > * _Merge any conflicting trait definitions using trait conflict resolution_.
    /// > * _Merge the metadata properties of both models using the metadata merge rules_.
    ///
    pub fn merge(&mut self, other: Model) -> ModelResult<()> {
        // Ensure version match
        if other.smithy_version != self.smithy_version {
            return Err(ErrorKind::InvalidVersionNumber(other.smithy_version.to_string()).into());
        }

        // shape names

        // traits

        // metadata

        Ok(())
    }

    // --------------------------------------------------------------------------------------------

    /// Return the Smithy version this model conforms to.
    pub fn smithy_version(&self) -> &Version {
        &self.smithy_version
    }

    // --------------------------------------------------------------------------------------------

    /// Returns `true` if this model's **metadata** collection has _any_ elements, else `false`.
    pub fn has_metadata(&self) -> bool {
        !self.metadata.is_empty()
    }

    /// Returns `true` if this model's **metadata** collection has a shape with the provided key`, else `false`.
    pub fn has_metadata_value(&self, key: &str) -> bool {
        !self.metadata.contains_key(key)
    }

    /// Returns the value in this model's **metadata** collection with the provide key.
    pub fn metadata_value(&self, key: &str) -> Option<&Value> {
        self.metadata.get(key)
    }

    /// Add a key/value pair to this model's **metadata** collection.
    pub fn add_metadata(&mut self, key: String, metadata_value: Value) -> Option<Value> {
        self.metadata.insert(key, metadata_value)
    }

    /// Remove the value with the associated key, from this model's **metadata** collection.
    pub fn remove_metadata(&mut self, key: &str) -> Option<Value> {
        self.metadata.remove(key)
    }

    /// Return an iterator over all key/value pairs in this model's **metadata** collection.
    pub fn metadata(&self) -> impl Iterator<Item = (&String, &Value)> {
        self.metadata.iter()
    }

    // --------------------------------------------------------------------------------------------

    /// Returns `true` if this model's **shapes** collection has _any_ elements, else `false`.
    pub fn has_shapes(&self) -> bool {
        !self.shapes.is_empty()
    }

    /// Returns `true` if this model's **shapes** collection has a shape with the provided `ShapeID`, else `false`.
    pub fn has_shape(&self, shape_id: &ShapeID) -> bool {
        !self.shapes.contains_key(shape_id)
    }

    /// Returns the shape in this model's **shapes** collection with the provided `ShapeID`.
    pub fn shape(&self, shape_id: &ShapeID) -> Option<&TopLevelShape> {
        self.shapes.get(shape_id)
    }

    /// Add an instance of `TopLevelShape` to  this model's **shapes** collection.
    pub fn add_shape(&mut self, shape: TopLevelShape) -> ModelResult<Option<TopLevelShape>> {
        if shape.id().is_member() {
            Err(ErrorKind::ShapeIDExpected(shape.id().clone()).into())
        } else {
            // TODO: check for any existing unresolved shape
            // (https://github.com/johnstonskj/rust-atelier/issues/3)
            Ok(self.shapes.insert(shape.id().clone(), shape))
        }
    }

    /// Remove any element, equal to the provided value, from this model's **shapes** collection.
    pub fn remove_shape(&mut self, shape_id: &ShapeID) -> Option<TopLevelShape> {
        self.shapes.remove(shape_id)
    }

    /// Return an iterator over all shapes in this model's **shapes** collection.
    pub fn shapes(&self) -> impl Iterator<Item = &TopLevelShape> {
        self.shapes.values()
    }

    /// Return an iterator over all names for shapes defined within this model; this filters out
    /// any shape in the model's **shapes** collection with kind `ShapeKind::Unresolved`.
    pub fn shape_names(&self) -> impl Iterator<Item = &ShapeID> {
        self.shapes
            .values()
            .filter(|shape| !shape.is_unresolved())
            .map(|shape| shape.id())
    }

    /// Return a vector containing all the namespaces used in defining shapes in this model.
    pub fn namespaces(&self) -> Vec<&NamespaceID> {
        let mut result: Vec<&NamespaceID> = self.shape_names().map(|id| id.namespace()).collect();
        result.dedup();
        result
    }

    // --------------------------------------------------------------------------------------------

    ///
    /// This returns `true` if the model contains **no** unresolved shape identifiers (see
    /// `ShapeKind::Unresolved`). An incomplete model **should** always result in a validation
    /// error and not be used.
    ///
    pub fn is_complete(&self) -> bool {
        !self.shapes.values().any(|shape| shape.is_unresolved())
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
pub mod identity;
pub use identity::{HasIdentity, Identifier, NamespaceID, ShapeID};

pub mod selector;

pub mod shapes;

pub mod values;

pub mod visitor;
