/*!
The Smithy semantic model itself, consisting of shapes, members, values, and model statements.

For more information, see [the Rust Atelier book](https://rust-atelier.dev/using/model_api.html).
*/

use crate::error::{ErrorKind, Result as ModelResult};
use crate::model::shapes::{HasTraits, NonTraitEq, ShapeKind, TopLevelShape};
use crate::model::values::{Value, ValueMap};
use crate::Version;
use std::collections::{HashMap, HashSet};
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
#[derive(Clone, Debug, PartialEq)]
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
    /// Merge the other model into this one. This follows the rules set out in the
    /// [Model files](https://awslabs.github.io/smithy/1.0/spec/core/model.html#model-files) section
    /// of the Smithy specification.
    ///
    /// > Smithy models MAY be divided into multiple files so that they are easier to maintain and
    /// > evolve. One or more model files can be assembled (or merged) together to form a semantic
    /// > model. The model files that form a semantic model are not required to all be defined in
    /// > the same representation; some models can be defined using the IDL and others can be
    /// > defined using the JSON AST.
    /// >
    /// > Model files do not explicitly include other model files; this responsibility is left to
    /// > tooling to ensure that all necessary model files are merged together to form a valid
    /// > semantic model.
    /// >
    /// > **1.3.1. Merging model files**
    /// >
    /// > Implementations MUST take the following steps when merging two or more model files to form
    /// > a semantic model:
    /// >
    /// > 1. Merge the metadata objects of all model files using the steps defined in Merging metadata.
    /// > 1. Shapes defined in a single model file are added to the semantic model as-is.
    /// > 1. Shapes with the same shape ID defined in multiple model files are reconciled using the
    /// >    following rules:
    /// >    1. All conflicting shapes MUST have the same shape type.
    /// >    1. Conflicting aggregate shapes MUST contain the same members that target the same shapes.
    /// >    1. Conflicting service shapes MUST contain the same properties and target the same shapes.
    /// > 1. Conflicting traits defined in shape definitions or through apply statements are reconciled
    /// >    using trait conflict resolution.
    /// >
    /// > **Note**
    /// > The following guidance is non-normative. Because the Smithy IDL allows forward references
    /// > to shapes that have not yet been defined or shapes that are defined in another model file,
    /// > implementations likely need to defer resolving relative shape IDs to absolute shape IDs
    /// > until all model files are loaded.
    ///
    pub fn merge(&mut self, other: Model) -> ModelResult<()> {
        // Ensure version match
        if other.smithy_version != self.smithy_version {
            return Err(ErrorKind::MergeVersionConflict(
                self.smithy_version.to_string(),
                other.smithy_version.to_string(),
            )
            .into());
        }

        for (key, value) in other.metadata {
            let _ = self.add_metadata(key, value)?;
        }

        for (_, shape) in other.shapes {
            let _ = self.add_shape(shape)?;
        }

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
        self.metadata.contains_key(key)
    }

    /// Returns the value in this model's **metadata** collection with the provide key.
    pub fn metadata_value(&self, key: &str) -> Option<&Value> {
        self.metadata.get(key)
    }

    ///
    /// Add a key/value pair to this model's **metadata** collection. This performs the Smithy
    /// conflict resolution to ensure the model is valid.
    ///
    /// From [Merging metadata](https://awslabs.github.io/smithy/1.0/spec/core/model.html#merging-metadata):
    ///
    /// > When a conflict occurs between top-level metadata key-value pairs, metadata is merged
    /// > using the following logic:
    /// >
    /// > 1. If a metadata key is only present in one model, then the entry is valid and added to
    /// >    the merged model.
    /// > 1. If both models contain the same key and both values are arrays, then the entry is
    /// >    valid; the values of both arrays are concatenated into a single array and added to the
    /// >    merged model.
    /// > 1. If both models contain the same key and both values are exactly equal, then the
    /// >    conflict is ignored and the value is added to the merged model.
    /// > 1. If both models contain the same key, the values do not both map to arrays, and the
    /// >    values are not equal, then the key is invalid and there is a metadata conflict error.
    ///
    pub fn add_metadata(&mut self, key: String, value: Value) -> ModelResult<Option<Value>> {
        Ok(if let Some(self_value) = self.metadata_value(&key) {
            if self_value.is_array() && value.is_array() {
                let mut self_array = self_value.as_array().unwrap().clone();
                let other_array = value.as_array().unwrap();
                self_array.extend(other_array.iter().cloned());
                self.metadata.insert(key.clone(), Value::Array(self_array))
            } else if *self_value == value {
                // name conflict is ignored.
                None
            } else {
                return Err(ErrorKind::MergeMetadataConflict(key.clone()).into());
            }
        } else {
            self.metadata.insert(key.clone(), value)
        })
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
        self.shapes.contains_key(shape_id)
    }

    /// Returns the shape in this model's **shapes** collection with the provided `ShapeID`.
    pub fn shape(&self, shape_id: &ShapeID) -> Option<&TopLevelShape> {
        self.shapes.get(shape_id)
    }

    /// Returns the shape in this model's **shapes** collection with the provided `ShapeID`.
    pub fn shape_mut(&mut self, shape_id: &ShapeID) -> Option<&mut TopLevelShape> {
        self.shapes.get_mut(shape_id)
    }

    ///
    /// Add an instance of `TopLevelShape` to  this model's **shapes** collection. This performs
    /// the Smithy conflict resolution to ensure the model is valid.
    ///
    /// From [Merging metadata](https://awslabs.github.io/smithy/1.0/spec/core/model.html#merging-metadata):
    ///
    /// > 1. Shapes with the same shape ID defined in multiple model files are reconciled using the
    /// >    following rules:
    /// >    1. All conflicting shapes MUST have the same shape type.
    /// >    1. Conflicting aggregate shapes MUST contain the same members that target the same shapes.
    /// >    1. Conflicting service shapes MUST contain the same properties and target the same shapes.
    ///
    pub fn add_shape(&mut self, shape: TopLevelShape) -> ModelResult<()> {
        if shape.id().is_member() && !matches!(shape.body(), ShapeKind::Unresolved) {
            error!(
                "Model::add_shape '{}' is a member ID; only allowed for unresolved shapes",
                shape.id()
            );
            return Err(ErrorKind::ShapeIDExpected(shape.id().clone()).into());
        } else if let Some(existing) = self.shape_mut(shape.id()) {
            // > 1. All conflicting shapes MUST have the same shape type.
            // > 1. Conflicting aggregate shapes MUST contain the same members that target the same shapes.
            // > 1. Conflicting service shapes MUST contain the same properties and target the same shapes.
            match (
                existing.body().is_unresolved(),
                shape.body().is_unresolved(),
            ) {
                // TODO: This does not deal with unresolved member shapes, only top-level.
                (false, false) => {
                    if existing.equal_without_traits(&shape) {
                        copy_traits(existing, &shape)?;
                    } else {
                        error!("Model::add_shape {:?} != {:?}", existing, shape);
                        return Err(ErrorKind::MergeShapeConflict(existing.id().clone()).into());
                    }
                }
                (true, false) => {
                    existing.set_body(shape.body().clone());
                    copy_traits(existing, &shape)?;
                }
                _ => {
                    copy_traits(existing, &shape)?;
                }
            }
        } else {
            let _ = self.shapes.insert(shape.id().clone(), shape);
        }
        Ok(())
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

    /// Return an iterator over all names for shapes defined outside this model; this includes
    /// only shapes in the model's **shapes** collection with kind `ShapeKind::Unresolved`.
    pub fn unresolved_shape_names(&self) -> impl Iterator<Item = &ShapeID> {
        self.shapes
            .values()
            .filter(|shape| shape.is_unresolved())
            .map(|shape| shape.id())
    }

    /// Return a set containing all the namespaces used in defining shapes in this model.
    pub fn namespaces(&self) -> HashSet<&NamespaceID> {
        self.shape_names().map(|id| id.namespace()).collect()
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
// Private Functions
// ------------------------------------------------------------------------------------------------

#[inline]
fn copy_traits(to_shape: &mut TopLevelShape, from_shape: &TopLevelShape) -> ModelResult<()> {
    for (id, value) in from_shape.traits() {
        to_shape.apply_with_value(id.clone(), value.clone())?;
    }
    Ok(())
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
