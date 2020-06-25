/*!
The core model itself, consisting of shapes, members, types, values, and model statements.

# Model Naming Conventions

As the majority of the structures in the core model are simply data carrying representations it is
useful to have a set of patterns for how different fields are accessed in these structures. The
following are the general patterns.

For property _version_ of type `T` (required, single value):

* `fn version(&self) -> T;` returns a reference to the current value.
* `fn set_version(&self, v: T);` sets the current value.

For property _input_ of type `Option<T>` (optional, single value):

* `fn has_input(&self) -> bool;` returns `true` if the value is `Some(T)`, else `false`.
* `fn input(&self) -> &Option<T>;` returns a reference to the current value.
* `fn set_input(&self, v: T);` sets the current value.
* `fn unset_input(&self);` sets the current value to `None`.

For property _traits_ of type `Vec<T>` (multi-valued, no identity):

* `fn has_traits(&self) -> bool;` returns `true` if there are any values in the vector, else `false`.
* `fn traits(&self) -> impl Iterator<Item = &T>;` returns an iterator over all the items in the vector.
* `fn add_trait(&mut self, v: T);` add (push) the value into the vector.
* `fn append_traits(&mut self, vs: &[T]);` add all the elements from the slice using `add_trait`.
* `fn remove_trait(&mut self, v: &T);` remove _all_ traits that are equal to the provided value from the vector.

For property _references_ of type `HashSet<T>` (multi-valued, with identity):

* `fn has_references(&self) -> bool;` returns `true` if there are any values in the set, else `false`.
* `fn has_reference(&self, v: &T) -> bool;` returns `true` if the set contains the provided value, else `false`.
* `fn references(&self) -> impl Iterator<Item = &T>;` returns an iterator over all the items in the set.
* `fn add_reference(&mut self, v: T);` add (insert) the value into the set.
* `fn append_references(&mut self, vs: &[T]);` add all the elements from the slice using `add_reference`.
* `fn remove_reference(&mut self, v: &T);` remove the provided value from the set.

For property _shapes_ of type `HashMap<K, V> where V: Named<I>` (a map of identity to value):

* `fn has_shapes(&self) -> bool;` returns `true` if there are any values in the map, else `false`.
* `fn has_shape(&self, k: &K) -> bool;` returns `true` if the map contains the provided key value, else `false`.
* `fn shapes(&self) -> impl Iterator<Item = (&K, &V)>;` returns an iterator over all the items in the map.
* `fn add_shape(&mut self, k: K, v: V);` add (insert) the value into the map.
* `fn append_shapes(&mut self, v: &[V]);` add all the elements from the slice using `add_shape`.
* `fn remove_shape(&mut self, k: &K);` remove any entry from the map with the provided key.


*/

use crate::model::shapes::{Shape, Trait, Valued};
use crate::model::values::{Key, NodeValue};
use crate::Version;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The core model structure, this corresponds to a single Smithy file according to the
/// specification. It contains:
///
/// * Optionally, the version of Smithy it conforms to.
/// * Control values.
/// * Any metadata associated with the model (with the `metadata` statement).
/// * The namespace it represents.
/// * A list of external shape references (with the `use` statement).
/// * A map of shapes declared by the model.
///
#[derive(Clone, Debug)]
pub struct Model {
    version: Option<Version>,
    // control_section
    control_data: HashMap<Key, NodeValue>,
    // metadata_section
    metadata: HashMap<Key, NodeValue>,
    // shape_section
    // shape_section > namespace_statement
    namespace: Namespace,
    // shape_section > use_section
    references: HashSet<ShapeID>,
    // shape_section > shape_statements : *(shape_statement / apply_statement)
    shapes: HashMap<Identifier, Shape>,
}

///
/// A trait implemented by model elements that have a strong name/identity. Note that identity is
/// immutable, no model element has a `set_id` or `unset_id` method.
///
pub trait Named<I> {
    /// The identity of this model element.
    fn id(&self) -> &I;
}

///
/// A trait implemented by model elements that may have traits applied to them.
///
pub trait Annotated {
    /// Returns `true` if the model element has any applied traits, else `false`.
    fn has_traits(&self) -> bool;

    /// Returns `true` if the model element has any applied traits with the associated id, else `false`.
    fn has_trait(&self, id: &ShapeID) -> bool;

    /// Return an iterator over all traits applied to this model element
    fn traits(&self) -> &Vec<Trait>;

    /// Add a new trait to this model element.
    fn add_trait(&mut self, a_trait: Trait);

    /// Add all the traits to this model element.
    fn append_traits(&mut self, traits: &[Trait]) {
        for a_trait in traits {
            self.add_trait(a_trait.clone());
        }
    }

    /// Remove _any_ trait from this model element with the provided id.
    fn remove_trait(&mut self, id: &ShapeID);

    /// A short-cut to add the standard documentation trait.
    fn documentation(&mut self, doc: &str) {
        let mut doc_trait = Trait::new(ShapeID::from_str("documentation").unwrap());
        doc_trait.set_value(NodeValue::String(doc.to_string()));
        let _ = self.add_trait(doc_trait);
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Model {
    /// Create a new model with the provided namespace.
    pub fn new(namespace: Namespace) -> Self {
        Self {
            version: None,
            control_data: Default::default(),
            namespace,
            references: Default::default(),
            shapes: Default::default(),
            metadata: Default::default(),
        }
    }

    /// Return the Smithy version this model conforms to.
    pub fn version(&self) -> &Option<Version> {
        &self.version
    }

    /// Set the Smithy version this model conforms to.
    pub fn set_version(&mut self, version: Version) {
        self.version = Some(version);
    }

    // --------------------------------------------------------------------------------------------

    /// Return the namespace of this model.
    pub fn namespace(&self) -> &Namespace {
        &self.namespace
    }

    /// Set the namespace of this model.
    pub fn set_namespace(&mut self, namespace: Namespace) {
        self.namespace = namespace
    }

    // --------------------------------------------------------------------------------------------

    /// Returns `true` if this model contains _any_ references, else `false`.
    pub fn has_references(&self) -> bool {
        !self.references.is_empty()
    }

    /// Returns `true` if this model contains a references with the given `ShapeID`, else `false`.
    pub fn has_reference(&self, id: &ShapeID) -> bool {
        self.references.contains(id)
    }

    /// Returns an iterator over all the references in this model.
    pub fn references(&self) -> impl Iterator<Item = &ShapeID> {
        self.references.iter()
    }

    /// Add a reference to the shape, with the given `ShapeID`, to this model.
    pub fn add_reference(&mut self, id: ShapeID) {
        let _ = self.references.insert(id);
    }

    /// Append all the given shape identifiers as references to this model.
    pub fn append_references(&mut self, ids: &[ShapeID]) {
        for id in ids {
            let _ = self.references.insert(id.clone());
        }
    }

    /// Remove any reference to the given `ShapeID` from this model.
    pub fn remove_reference(&mut self, id: &ShapeID) {
        let _ = self.references.remove(id);
    }

    // --------------------------------------------------------------------------------------------

    /// Returns `true` if this model contains _any_ shapes, else `false`.
    pub fn has_shapes(&self) -> bool {
        !self.shapes.is_empty()
    }

    /// Returns `true` if this model contains a shape with the given `Identifier`, else `false`.
    pub fn has_shape(&self, shape_id: &Identifier) -> bool {
        self.shapes.contains_key(shape_id)
    }

    /// Return the shape in this model with the given `Identifier`.
    pub fn shape(&self, shape_id: &Identifier) -> Option<&Shape> {
        self.shapes.get(shape_id)
    }

    /// Returns an iterator over all the shapes in this model.
    pub fn shapes(&self) -> impl Iterator<Item = &Shape> {
        self.shapes.values()
    }

    /// Add the given shape to this model.
    pub fn add_shape(&mut self, shape: Shape) {
        let _ = self.shapes.insert(shape.id().clone(), shape);
    }

    /// Append all the given shapes to this model.
    pub fn append_shapes(&mut self, shapes: &[Shape]) {
        for shape in shapes {
            let _ = self.shapes.insert(shape.id().clone(), shape.clone());
        }
    }

    /// Remove any shape with the given `Identifier` from this model.
    pub fn remove_shape(&mut self, shape_id: &Identifier) {
        let _ = self.shapes.remove(shape_id);
    }

    // --------------------------------------------------------------------------------------------

    /// Returns `true` if this model contains _any_ control data, else `false`.
    pub fn has_control_data(&self) -> bool {
        !self.control_data.is_empty()
    }

    /// Returns an iterator over all the control data in this model.
    pub fn control_data(&self) -> impl Iterator<Item = (&Key, &NodeValue)> {
        self.control_data.iter()
    }

    /// Add the given control data key and value to this model.
    pub fn add_control_data(&mut self, key: Key, value: NodeValue) {
        let _ = self.control_data.insert(key, value);
    }

    /// Remove the control data with the given `Key` from this model.
    pub fn remove_control_data(&mut self, key: &Key) {
        let _ = self.control_data.remove(key);
    }

    // --------------------------------------------------------------------------------------------

    /// Returns `true` if this model contains _any_ metadata, else `false`.
    pub fn has_metadata(&self) -> bool {
        !self.metadata.is_empty()
    }

    /// Returns an iterator over all the metadata in this model.
    pub fn metadata(&self) -> impl Iterator<Item = (&Key, &NodeValue)> {
        self.metadata.iter()
    }

    /// Add the given metadata key and value to this model.
    pub fn add_metadata(&mut self, key: Key, value: NodeValue) {
        let _ = self.metadata.insert(key, value);
    }

    /// Remove the metadata with the given `Key` from this model.
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
pub use identity::{Identifier, Namespace, ShapeID};

pub mod select;

pub mod shapes;

pub mod values;
