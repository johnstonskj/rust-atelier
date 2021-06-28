/*!
This module provides a trait and public function to help implement model visitors.

For more information, see [the Rust Atelier book](https://rust-atelier.dev/using/visitor.html).

*/

use crate::model::shapes::{
    AppliedTraits, HasTraits, ListOrSet, Map, Operation, Resource, Service, ShapeKind, Simple,
    StructureOrUnion,
};
use crate::model::values::Value;
use crate::model::{HasIdentity, Identifier, Model, ShapeID};
use crate::syntax::{
    MEMBER_COLLECTION_OPERATIONS, MEMBER_CREATE, MEMBER_DELETE, MEMBER_ERRORS, MEMBER_INPUT,
    MEMBER_LIST, MEMBER_OPERATIONS, MEMBER_OUTPUT, MEMBER_PUT, MEMBER_READ, MEMBER_RESOURCES,
    MEMBER_UPDATE,
};

// ------------------------------------------------------------------------------------------------
// Macros
// ------------------------------------------------------------------------------------------------

macro_rules! visit_fn {
    (mut $fn_name:ident, $shape_type:ty, $doc:expr) => {
        #[doc = $doc]
        #[allow(unused_variables)]
        fn $fn_name(
            &mut self,
            id: &ShapeID,
            traits: &AppliedTraits,
            shape: &$shape_type,
        ) -> Result<(), Self::Error> {
            Ok(())
        }
    };
    (mut $fn_name:ident, $doc:expr) => {
        #[doc = $doc]
        #[allow(unused_variables)]
        fn $fn_name(&mut self, id: &ShapeID, traits: &AppliedTraits) -> Result<(), Self::Error> {
            Ok(())
        }
    };
    ($fn_name:ident, $shape_type:ty, $doc:expr) => {
        #[doc = $doc]
        #[allow(unused_variables)]
        fn $fn_name(
            &self,
            id: &ShapeID,
            traits: &AppliedTraits,
            shape: &$shape_type,
        ) -> Result<(), Self::Error> {
            Ok(())
        }
    };
    ($fn_name:ident, $doc:expr) => {
        #[doc = $doc]
        #[allow(unused_variables)]
        fn $fn_name(&self, id: &ShapeID, traits: &AppliedTraits) -> Result<(), Self::Error> {
            Ok(())
        }
    };
}
// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A trait implemented by tools that wish to visit parts of the model and may choose to ignore
/// some In this way a simple filter to read structures for example can be applied.
///
/// Each method in the trait will return `Ok` by default so a particular implementation can choose
/// which methods to override.
///
pub trait ModelVisitor {
    /// The error which will be returned by this visitor.
    type Error;

    /// Called once for each key in the model's metadata.
    #[allow(unused_variables)]
    fn metadata(&self, key: &str, value: &Value) -> Result<(), Self::Error> {
        Ok(())
    }

    visit_fn! { simple_shape, Simple, "Called for each `ShapeKind::Simple` in this model's **shapes** collection." }
    visit_fn! { list, ListOrSet, "Called for each `ShapeKind::List` in this model's **shapes** collection." }
    visit_fn! { set, ListOrSet, "Called for each `ShapeKind::Set` in this model's **shapes** collection." }
    visit_fn! { map, Map, "Called for each `ShapeKind::Map` in this model's **shapes** collection." }
    visit_fn! { structure, StructureOrUnion, "Called for each `ShapeKind::Structure` in this model's **shapes** collection." }
    visit_fn! { union, StructureOrUnion, "Called for each `ShapeKind::Union` in this model's **shapes** collection." }
    visit_fn! { service, Service, "Called for each `ShapeKind::Service` in this model's **shapes** collection." }
    visit_fn! { operation, Operation, "Called for each `ShapeKind::Operation` in this model's **shapes** collection." }
    visit_fn! { resource, Resource, "Called for each `ShapeKind::Resource` in this model's **shapes** collection." }
    visit_fn! { unresolved_id, "Called for each `ShapeKind::Unresolved` shape identifier in this model's **shapes** collection." }

    /// Return a visitor that will be called for every member of an aggregate or service shape.
    fn member_visitor(&self) -> Option<&dyn MemberVisitor<Error = Self::Error>> {
        None
    }
}

///
/// This trait will be called once for each member of an aggregate, or service, shape. These
/// calls are made directly after the corresponding call to the main visitor, so for example a
/// Map will result in a `map()` method call followed me a `member()` call for the `key` and `value`
/// members.
///
/// For service shapes:
///
/// 1. Optional members will only result in a call to `member()` if the target shape id is
///    present.
/// 1. Members with multiple values will result in multiple calls to `member()`.
/// 1. The `identifiers` member on `Resource`, and the `renames` member on `Service` are
///    **not** members and need to be handled in the the corresponding shape visitor method.
///
pub trait MemberVisitor {
    /// The error which will be returned by this visitor.
    type Error;

    /// Called for any member, note that for service shapes the `traits` parameter will always
    /// be `None`.
    #[allow(unused_variables)]
    fn member(
        &self,
        parent_shape_id: &ShapeID,
        member_name: &Identifier,
        target: &ShapeID,
        traits: Option<&AppliedTraits>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Walk the provided model calling out to the visitor as necessary. This is a useful tool for use
/// cases where you do not need to cross-validate model elements but can process the model shape by
/// shape independently.
///
pub fn walk_model<V>(model: &Model, visitor: &V) -> Result<(), V::Error>
where
    V: ModelVisitor,
{
    for (key, value) in model.metadata() {
        visitor.metadata(key, value)?;
    }

    for shape in model.shapes() {
        match &shape.body() {
            ShapeKind::Simple(body) => visitor.simple_shape(shape.id(), &shape.traits(), &body)?,
            ShapeKind::List(body) => {
                visitor.list(shape.id(), &shape.traits(), &body)?;
                if let Some(member_visitor) = visitor.member_visitor() {
                    member_visitor.member(
                        shape.id(),
                        body.member().id(),
                        body.member().target(),
                        Some(body.member().traits()),
                    )?;
                }
            }
            ShapeKind::Set(body) => {
                visitor.set(shape.id(), &shape.traits(), &body)?;
                if let Some(member_visitor) = visitor.member_visitor() {
                    member_visitor.member(
                        shape.id(),
                        body.member().id(),
                        body.member().target(),
                        Some(body.member().traits()),
                    )?;
                }
            }
            ShapeKind::Map(body) => {
                visitor.map(shape.id(), &shape.traits(), &body)?;
                if let Some(member_visitor) = visitor.member_visitor() {
                    member_visitor.member(
                        shape.id(),
                        body.key().id(),
                        body.key().target(),
                        Some(body.key().traits()),
                    )?;
                    member_visitor.member(
                        shape.id(),
                        body.value().id(),
                        body.value().target(),
                        Some(body.value().traits()),
                    )?;
                }
            }
            ShapeKind::Structure(body) => {
                visitor.structure(shape.id(), &shape.traits(), &body)?;
                if let Some(member_visitor) = visitor.member_visitor() {
                    for member in body.members.values() {
                        member_visitor.member(
                            shape.id(),
                            member.id(),
                            member.target(),
                            Some(member.traits()),
                        )?;
                    }
                }
            }
            ShapeKind::Union(body) => {
                visitor.union(shape.id(), &shape.traits(), &body)?;
                if let Some(member_visitor) = visitor.member_visitor() {
                    for member in body.members.values() {
                        member_visitor.member(
                            shape.id(),
                            member.id(),
                            member.target(),
                            Some(member.traits()),
                        )?;
                    }
                }
            }
            ShapeKind::Service(body) => {
                visitor.service(shape.id(), &shape.traits(), &body)?;
                if let Some(member_visitor) = visitor.member_visitor() {
                    for target in body.operations() {
                        member_visitor.member(
                            shape.id(),
                            &Identifier::new_unchecked(MEMBER_OPERATIONS),
                            target,
                            None,
                        )?;
                    }
                    for target in body.resources() {
                        member_visitor.member(
                            shape.id(),
                            &Identifier::new_unchecked(MEMBER_RESOURCES),
                            target,
                            None,
                        )?;
                    }
                }
            }
            ShapeKind::Operation(body) => {
                visitor.operation(shape.id(), &shape.traits(), &body)?;
                if let Some(member_visitor) = visitor.member_visitor() {
                    if let Some(target) = body.input() {
                        member_visitor.member(
                            shape.id(),
                            &Identifier::new_unchecked(MEMBER_INPUT),
                            target,
                            None,
                        )?;
                    }
                    if let Some(target) = body.output() {
                        member_visitor.member(
                            shape.id(),
                            &Identifier::new_unchecked(MEMBER_OUTPUT),
                            target,
                            None,
                        )?;
                    }
                    for target in body.errors() {
                        member_visitor.member(
                            shape.id(),
                            &Identifier::new_unchecked(MEMBER_ERRORS),
                            target,
                            None,
                        )?;
                    }
                }
            }
            ShapeKind::Resource(body) => {
                visitor.resource(shape.id(), &shape.traits(), &body)?;
                if let Some(member_visitor) = visitor.member_visitor() {
                    if let Some(target) = body.create() {
                        member_visitor.member(
                            shape.id(),
                            &Identifier::new_unchecked(MEMBER_CREATE),
                            target,
                            None,
                        )?;
                    }
                    if let Some(target) = body.put() {
                        member_visitor.member(
                            shape.id(),
                            &Identifier::new_unchecked(MEMBER_PUT),
                            target,
                            None,
                        )?;
                    }
                    if let Some(target) = body.read() {
                        member_visitor.member(
                            shape.id(),
                            &Identifier::new_unchecked(MEMBER_READ),
                            target,
                            None,
                        )?;
                    }
                    if let Some(target) = body.update() {
                        member_visitor.member(
                            shape.id(),
                            &Identifier::new_unchecked(MEMBER_UPDATE),
                            target,
                            None,
                        )?;
                    }
                    if let Some(target) = body.delete() {
                        member_visitor.member(
                            shape.id(),
                            &Identifier::new_unchecked(MEMBER_DELETE),
                            target,
                            None,
                        )?;
                    }
                    if let Some(target) = body.list() {
                        member_visitor.member(
                            shape.id(),
                            &Identifier::new_unchecked(MEMBER_LIST),
                            target,
                            None,
                        )?;
                    }
                    for target in body.operations() {
                        member_visitor.member(
                            shape.id(),
                            &Identifier::new_unchecked(MEMBER_OPERATIONS),
                            target,
                            None,
                        )?;
                    }
                    for target in body.collection_operations() {
                        member_visitor.member(
                            shape.id(),
                            &Identifier::new_unchecked(MEMBER_COLLECTION_OPERATIONS),
                            target,
                            None,
                        )?;
                    }
                    for target in body.resources() {
                        member_visitor.member(
                            shape.id(),
                            &Identifier::new_unchecked(MEMBER_RESOURCES),
                            target,
                            None,
                        )?;
                    }
                }
            }
            ShapeKind::Unresolved => visitor.unresolved_id(shape.id(), &shape.traits())?,
        }
    }

    Ok(())
}
