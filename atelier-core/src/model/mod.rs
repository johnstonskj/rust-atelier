/*!
The Smithy semantic model itself, consisting of shapes, members, values, and model statements.
*/

use crate::error::{ErrorKind, Result as ModelResult};
use crate::model::shapes::Shape;
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
    smithy_version: Version,
    metadata: ValueMap,
    shapes: HashMap<ShapeID, Shape>,
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

    object_member! { shapes, shape, ShapeID, Shape, has_shapes, has_shape, add_shape = add_a_shape, remove_shape }

    /// Return a list of all the shape IDs representing shapes defined in the model.
    pub fn shape_names(&self) -> impl Iterator<Item = &ShapeID> {
        self.shapes.keys()
    }

    /// Add an instance of `Shape`.
    pub fn add_a_shape(&mut self, shape: Shape) -> Option<Shape> {
        // TODO: check for any existing unresolved shape
        self.shapes.insert(shape.id().clone(), shape)
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

    // --------------------------------------------------------------------------------------------

    ///
    /// This performs the resolution of a shape ID in the model, it handles both absolute IDs as
    /// well as relative ones according to the following section from the Smithy specification,
    /// section 3.1.2.1.
    /// [Relative shape ID resolution](https://awslabs.github.io/smithy/1.0/spec/core/shapes.html#relative-shape-id-resolution)
    ///
    /// > In the Smithy IDL, relative shape IDs are resolved using the following process:
    /// >
    /// > 1. If a `use_statement` has imported a shape with the same name, the shape ID resolves to the imported shape ID.
    /// > 1. If a shape is defined in the same namespace as the shape with the same name, the namespace of the shape resolves to the current namespace.
    /// > 1. If a shape is defined in the prelude with the same name, the namespace resolves to smithy.api.
    /// > 1. If a relative shape ID does not satisfy one of the above cases, the shape ID is invalid, and the namespace is inherited from the current namespace.
    ///
    /// If `and_absolute` is true however this implementation will attempt to resolve even absolute
    /// shape identifiers against the model, any references and the prelude. Also, this implementation
    /// also resolves members, so that if the shape can be resolved the member within it will also
    /// be checked.
    ///
    #[cfg(feature = "resolver")]
    pub fn resolve_id(&self, id: &ShapeID, and_absolute: bool) -> Option<ShapeID> {
        // Cache the member name, if present, for later.
        let (id, member) = if id.is_member() {
            (
                ShapeID::new(id.namespace().clone(), id.shape_name().clone(), None),
                id.member_name().clone(),
            )
        } else {
            (id.clone(), None)
        };

        if id.is_absolute() && and_absolute {
            if self.references.contains(&id) {
                return Some(id);
            } else if id.namespace().as_ref().unwrap() == &self.default_namespace {
                return if let Some(shape) = self.shape(&id.to_relative()) {
                    if let Some(member_name) = member {
                        if match shape.body() {
                            ShapeKind::List(body) => body.has_member_named(&member_name),
                            ShapeKind::Set(body) => body.has_member_named(&member_name),
                            ShapeKind::Map(body) => body.has_member_named(&member_name),
                            ShapeKind::Structure(body) => body.has_member_named(&member_name),
                            ShapeKind::Union(body) => body.has_member_named(&member_name),
                            ShapeKind::Service(body) => body.has_member_named(&member_name),
                            ShapeKind::Operation(body) => body.has_member_named(&member_name),
                            ShapeKind::Resource(body) => body.has_member_named(&member_name),
                            _ => false,
                        } {
                            Some(id.to_member(member_name))
                        } else {
                            None
                        }
                    } else {
                        Some(id)
                    }
                } else {
                    None
                };
            } else if id.namespace().as_ref().unwrap()
                == &Namespace::from_str(PRELUDE_NAMESPACE).unwrap()
                && prelude_model_shape_ids(&self.smithy_version).contains(&id)
            {
                return Some(id);
            }
        } else if id.is_absolute() && !and_absolute {
            return match member {
                None => Some(id),
                Some(member) => Some(id.to_member(member)),
            };
        } else if let Some(id) = self
            .references
            .iter()
            .find(|shape_ref| shape_ref.to_relative() == id)
        {
            return Some(id.clone());
        } else if self.has_shape(&id) {
            let absolute_id = id.to_absolute(self.default_namespace.clone());
            return if let Some(shape) = self.shape(&id.to_relative()) {
                if let Some(member_name) = member {
                    if match shape.body() {
                        ShapeKind::List(body) => body.has_member_named(&member_name),
                        ShapeKind::Set(body) => body.has_member_named(&member_name),
                        ShapeKind::Map(body) => body.has_member_named(&member_name),
                        ShapeKind::Structure(body) => body.has_member_named(&member_name),
                        ShapeKind::Union(body) => body.has_member_named(&member_name),
                        ShapeKind::Service(body) => body.has_member_named(&member_name),
                        ShapeKind::Operation(body) => body.has_member_named(&member_name),
                        ShapeKind::Resource(body) => body.has_member_named(&member_name),
                        _ => false,
                    } {
                        Some(absolute_id.to_member(member_name))
                    } else {
                        None
                    }
                } else {
                    Some(absolute_id)
                }
            } else {
                None
            };
        } else if let Some(id) = prelude_model_shape_ids(&self.smithy_version)
            .iter()
            .find(|shape_ref| shape_ref.to_relative() == id)
        {
            return Some(id.clone());
        }
        None
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
pub mod identity;
pub use identity::{Identifier, Namespace, ShapeID};

pub mod shapes;

pub mod values;

pub mod visitor;
