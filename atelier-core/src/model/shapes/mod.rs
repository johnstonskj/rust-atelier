/*!
Model structures common across all shape types.

The concept of a _shape_ in Smithy is abstract, the ABNF contains productions `shape_statements`
and `shape_body` but they are not concrete. Shapes are then classified as _simple_, _aggregate_,
and _service_. The model here introduces `Shape` as a common concrete structure which contains an
enumeration, `ShapeInner`, to represent each of the productions referenced by `shape_body`.

*/

use crate::model::{Annotated, Identifier, Named, ShapeID};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This structure represents a shape within the model. The shape-specific data is within the
/// `ShapeInner` enumeration.
///
#[derive(Clone, Debug)]
pub struct Shape {
    id: Identifier,
    traits: Vec<Trait>,
    inner: ShapeInner,
}

///
/// This enumeration represents the set of shape types supported by Smithy.
///
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug)]
pub enum ShapeInner {
    /// Corresponds to the ABNF production `simple_shape_statement`.
    SimpleType(SimpleType),
    /// Corresponds to the ABNF production `list_statement`.
    List(ListOrSet),
    /// Corresponds to the ABNF production `set_statement`.
    Set(ListOrSet),
    /// Corresponds to the ABNF production `map_statement`.
    Map(Map),
    /// Corresponds to the ABNF production `structure_statement`.
    Structure(StructureOrUnion),
    /// Corresponds to the ABNF production `union_statement`.
    Union(StructureOrUnion),
    /// Corresponds to the ABNF production `service_statement`.
    Service(Service),
    /// Corresponds to the ABNF production `operation_statement`.
    Operation(Operation),
    /// Corresponds to the ABNF production `resource_statement`.
    Resource(Resource),
    /// Corresponds to the ABNF production `apply_statement`.
    Apply,
}
// ------------------------------------------------------------------------------------------------
// Macros
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
macro_rules! is_as {
    ($is_fn:ident, $variant:ident) => {
        /// Returns `true` if `self` is the corresponding variant, else `false`.
        pub fn $is_fn(&self) -> bool {
            match self {
                Self::$variant => true,
                _ => false,
            }
        }
    };
    ($is_fn:ident, $variant:ident, $as_fn:ident, $ret_type:ty) => {
        /// Returns `true` if `self` is the corresponding variant, else `false`.
        pub fn $is_fn(&self) -> bool {
            match self {
                Self::$variant(_) => true,
                _ => false,
            }
        }

        /// Returns `Some(v)` if `self` is the corresponding variant, else `None`.
        pub fn $as_fn(&self) -> Option<&$ret_type> {
            match self {
                Self::$variant(v) => Some(v),
                _ => None,
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl ShapeInner {
    is_as! { is_simple, SimpleType, as_simple, SimpleType }
    is_as! { is_list, List, as_list, ListOrSet }
    is_as! { is_set, Set, as_set, ListOrSet }
    is_as! { is_map, Map, as_map, Map}
    is_as! { is_structure, Structure, as_structure, StructureOrUnion}
    is_as! { is_union, Union, as_union, StructureOrUnion}
    is_as! { is_service, Service, as_service, Service }
    is_as! { is_operation, Operation, as_operation, Operation }
    is_as! { is_resource, Resource, as_resource, Resource }
    is_as! { is_apply, Apply }
}

impl Named<Identifier> for Shape {
    fn id(&self) -> &Identifier {
        &self.id
    }
}

impl Annotated for Shape {
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

impl Shape {
    ///
    /// Construct a new shape with the given identifier (shape name) and shape-specific data.
    ///
    pub fn new(id: Identifier, inner: ShapeInner) -> Self {
        Self {
            id,
            traits: Default::default(),
            inner,
        }
    }

    ///
    /// Return a reference to the shape-specific data within the shape.
    ///
    pub fn inner(&self) -> &ShapeInner {
        &self.inner
    }

    ///
    /// Return a mutable reference to the shape-specific data within the shape.
    ///
    pub fn inner_mut(&mut self) -> &mut ShapeInner {
        &mut self.inner
    }

    ///
    /// Set the shape-specific data for this shape.
    ///
    pub fn set_inner(&mut self, inner: ShapeInner) {
        self.inner = inner
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
pub mod members;
pub use members::{Member, Trait, Valued};

#[doc(hidden)]
pub mod services;
pub use services::{Operation, Resource, Service};

#[doc(hidden)]
pub mod types;
pub use types::{ListOrSet, Map, SimpleType, StructureOrUnion};
