/*!
One-line description.

More detailed description, with

# Example

*/

use crate::Projection;
use atelier_core::model::selector::{
    AttributeSelector, Function, NeighborSelector, ScopedAttributeSelector, Selector,
    SelectorExpression, ShapeType,
};
use atelier_core::model::shapes::{Simple, TopLevelShape};
use atelier_core::model::Model;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn query(query: &Selector, over_model: &Model) -> Projection {
    query
        .expressions()
        .fold(Projection::from(over_model), |projected, expression| {
            query_one(expression, projected, over_model)
        })
}

pub fn assert_true(_constraint: &Selector, _context: &TopLevelShape, _in_model: &Model) -> bool {
    unimplemented!()
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn query_one(
    query: &SelectorExpression,
    previously_selected: Projection,
    over_model: &Model,
) -> Projection {
    println!("query_one({:?} |-> {})", previously_selected, query);
    match query {
        SelectorExpression::ShapeType(v) => query_by_shape_type(v, previously_selected, over_model),
        SelectorExpression::AttributeSelector(v) => {
            query_by_attribute(v, previously_selected, over_model)
        }
        SelectorExpression::ScopedAttributeSelector(v) => {
            query_by_scoped_attribute(v, previously_selected, over_model)
        }
        SelectorExpression::NeighborSelector(v) => {
            query_by_neighbor(v, previously_selected, over_model)
        }
        SelectorExpression::Function(v) => query_by_function(v, previously_selected, over_model),
        SelectorExpression::VariableDefinition(v) => previously_selected,
        SelectorExpression::VariableReference(v) => previously_selected,
    }
}

fn query_by_shape_type(
    query: &ShapeType,
    previously_selected: Projection,
    over_model: &Model,
) -> Projection {
    match previously_selected {
        Projection::Members(mut shape_ids) => {
            shape_ids.retain(|shape_id| {
                if let Some(shape) = over_model.shape(&shape_id.shape_only()) {
                    shape.has_member(&shape_id.member_name().as_ref().unwrap())
                } else {
                    false
                }
            });
            Projection::Shapes(shape_ids)
        }
        Projection::Shapes(mut shape_ids) => {
            shape_ids.retain(|shape_id| {
                if let Some(shape) = over_model.shape(shape_id) {
                    match_by_shape_type(query, shape)
                } else {
                    false
                }
            });
            Projection::Shapes(shape_ids)
        }
        _ => Projection::default(),
    }
}

fn query_by_attribute(
    _query: &AttributeSelector,
    _previously_selected: Projection,
    _over_model: &Model,
) -> Projection {
    unimplemented!()
}

fn query_by_scoped_attribute(
    _query: &ScopedAttributeSelector,
    _previously_selected: Projection,
    _over_model: &Model,
) -> Projection {
    unimplemented!()
}

fn query_by_neighbor(
    _query: &NeighborSelector,
    _previously_selected: Projection,
    _over_model: &Model,
) -> Projection {
    unimplemented!()
}

fn query_by_function(
    _query: &Function,
    _previously_selected: Projection,
    _over_model: &Model,
) -> Projection {
    unimplemented!()
}

fn match_by_shape_type(shape_type: &ShapeType, shape: &TopLevelShape) -> bool {
    let shape_body = shape.body();
    match shape_type {
        ShapeType::All => true,
        ShapeType::Number => {
            if shape_body.is_simple() {
                let shape_body = shape.body().as_simple().unwrap();
                matches!(
                    shape_body,
                    Simple::BigDecimal
                        | Simple::BigInteger
                        | Simple::Double
                        | Simple::Float
                        | Simple::Integer
                        | Simple::Short
                )
            } else {
                false
            }
        }
        ShapeType::SimpleType => shape_body.is_simple(),
        ShapeType::Collection => shape_body.is_list() || shape_body.is_set() || shape_body.is_map(),
        ShapeType::Blob => {
            shape_body.is_simple() && matches!(shape.body().as_simple().unwrap(), Simple::Blob)
        }
        ShapeType::Boolean => {
            shape_body.is_simple() && matches!(shape.body().as_simple().unwrap(), Simple::Boolean)
        }
        ShapeType::Document => {
            shape_body.is_simple() && matches!(shape.body().as_simple().unwrap(), Simple::Document)
        }
        ShapeType::String => {
            shape_body.is_simple() && matches!(shape.body().as_simple().unwrap(), Simple::String)
        }
        ShapeType::Integer => {
            shape_body.is_simple() && matches!(shape.body().as_simple().unwrap(), Simple::Integer)
        }
        ShapeType::Byte => {
            shape_body.is_simple() && matches!(shape.body().as_simple().unwrap(), Simple::Byte)
        }
        ShapeType::Short => {
            shape_body.is_simple() && matches!(shape.body().as_simple().unwrap(), Simple::Short)
        }
        ShapeType::Long => {
            shape_body.is_simple() && matches!(shape.body().as_simple().unwrap(), Simple::Long)
        }
        ShapeType::Float => {
            shape_body.is_simple() && matches!(shape.body().as_simple().unwrap(), Simple::Float)
        }
        ShapeType::Double => {
            shape_body.is_simple() && matches!(shape.body().as_simple().unwrap(), Simple::Double)
        }
        ShapeType::BigDecimal => {
            shape_body.is_simple()
                && matches!(shape.body().as_simple().unwrap(), Simple::BigDecimal)
        }
        ShapeType::BigInteger => {
            shape_body.is_simple()
                && matches!(shape.body().as_simple().unwrap(), Simple::BigInteger)
        }
        ShapeType::Timestamp => {
            shape_body.is_simple() && matches!(shape_body.as_simple().unwrap(), Simple::Timestamp)
        }
        ShapeType::List => shape.body().is_list(),
        ShapeType::Set => shape.body().is_set(),
        ShapeType::Map => shape.body().is_map(),
        ShapeType::Structure => shape.body().is_structure(),
        ShapeType::Union => shape.body().is_union(),
        ShapeType::Service => shape.body().is_service(),
        ShapeType::Operation => shape.body().is_operation(),
        ShapeType::Resource => shape.body().is_resource(),
        ShapeType::Member => false,
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
