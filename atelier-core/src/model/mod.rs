/*!
The Smithy semantic model itself, consisting of shapes, members, values, and model statements.
*/

use crate::error::{ErrorKind, Result as ModelResult};
use crate::model::shapes::{Shape, TopLevelShape};
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
    object_member! { metadata, metadata_value, String => Value, has_metadata, has_metadata_value, add_metadata, remove_metadata }

    object_member! { shapes, shape, ShapeID, TopLevelShape, has_shapes, has_shape, add_shape = add_a_shape, remove_shape }

    /// Return a list of all the shape IDs representing shapes defined in the model.
    pub fn shape_names(&self) -> impl Iterator<Item = &ShapeID> {
        self.shapes.keys()
    }

    /// Add an instance of `Shape`.
    pub fn add_a_shape(&mut self, shape: TopLevelShape) -> ModelResult<Option<TopLevelShape>> {
        if shape.id().is_member() {
            Err(ErrorKind::ShapeIDExpected(shape.id().clone()).into())
        } else {
            // TODO: check for any existing unresolved shape
            Ok(self.shapes.insert(shape.id().clone(), shape))
        }
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
pub use identity::{Identifier, NamespaceID, ShapeID};

pub mod shapes;

pub mod values;

pub mod visitor;
