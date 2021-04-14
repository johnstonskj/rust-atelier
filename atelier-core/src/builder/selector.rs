/*!
* Builders to construct selector models in a more fluent style.
*
* # Example
*
* The following example constructs the simple shape selector `"*"` using the model API.
*
* ```rust
* use atelier_core::model::selector::{SelectorExpression, ShapeType, Selector};
*
* let selector: Selector = SelectorExpression::ShapeType(ShapeType::Any).into();
* ```
*
* The following uses `SelectorBuilder` but still accepts the model `ShapeType`.
*
* ```rust
* use atelier_core::model::selector::{ShapeType, Selector};
* use atelier_core::builder::selector::SelectorBuilder;
*
* let selector: Selector = SelectorBuilder::from(ShapeType::Any.into()).into();
* ```
*
* The selector builder has a number of helper functions to add common selector expressions and so
* the model API is not required at all.
*
* ```rust
* use atelier_core::model::selector::Selector;
* use atelier_core::builder::selector::SelectorBuilder;
*
* let selector: Selector = SelectorBuilder::any_shape().into();
* ```
*
*/

use crate::model::selector::{
    AttributeComparison, AttributeSelector, Comparator, Function, Key, KeyPathSegment,
    NeighborSelector, ScopedAttributeAssertion, ScopedAttributeSelector, ScopedValue, Selector,
    SelectorExpression, ShapeType, Value, VariableDefinition, VariableReference,
};
use crate::model::values::Number;
use crate::model::{Identifier, ShapeID};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// Builder for `Selector` instances.
#[derive(Clone, Debug)]
pub struct SelectorBuilder {
    exs: Vec<SelectorExpression>,
}

/// Builder for `AttributeSelector` instances.
#[derive(Clone, Debug)]
pub struct AttributeBuilder {
    selector: AttributeSelector,
}

/// Builder for `ScopedAttributeSelector` instances.
#[derive(Clone, Debug)]
pub struct ScopedAttributeBuilder {
    key: Option<Key>,
    assertions: Vec<ScopedAttributeAssertion>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl From<&mut SelectorBuilder> for SelectorBuilder {
    fn from(v: &mut SelectorBuilder) -> Self {
        v.clone()
    }
}

impl From<SelectorBuilder> for Selector {
    fn from(v: SelectorBuilder) -> Self {
        let mut v = v;
        Selector::from(&mut v)
    }
}

impl From<&mut SelectorBuilder> for Vec<Selector> {
    fn from(v: &mut SelectorBuilder) -> Self {
        vec![v.into()]
    }
}

impl From<&mut SelectorBuilder> for Selector {
    fn from(v: &mut SelectorBuilder) -> Self {
        assert!(!v.exs.is_empty());
        Selector::from(v.exs.clone())
    }
}

impl SelectorBuilder {
    /// Returns `true` if the list of expressions is empty, else `false`.
    pub fn is_empty(&self) -> bool {
        self.exs.is_empty()
    }

    /// Add a single selector expression to this selector.
    pub fn add(&mut self, ex: SelectorExpression) -> &mut Self {
        self.exs.push(ex);
        self
    }

    /// Construct a new `ExpressionListBuilder` from a single expression.
    pub fn from(v: SelectorExpression) -> Self {
        Self { exs: vec![v] }
    }

    /// Construct a single `ShapeType` selector expression.
    pub fn shape(v: ShapeType) -> Self {
        Self::from(v.into())
    }

    /// Add a single `ShapeType` selector expression to this selector.
    pub fn add_shape(&mut self, v: ShapeType) -> &mut Self {
        self.add(v.into())
    }

    /// Construct a single `ShapeType` selector expression.
    pub fn any_shape() -> Self {
        Self::shape(ShapeType::Any)
    }

    /// Add a single `ShapeType` selector expression to this selector.
    pub fn add_any_shape(&mut self) -> &mut Self {
        self.add_shape(ShapeType::Any)
    }

    /// Construct a single `ShapeType` selector expression.
    pub fn any_number() -> Self {
        Self::shape(ShapeType::Number)
    }

    /// Add a single `ShapeType` selector expression to this selector.
    pub fn add_any_number(&mut self) -> &mut Self {
        self.add_shape(ShapeType::Number)
    }

    /// Construct a single `ShapeType` selector expression.
    pub fn any_simple_type() -> Self {
        Self::shape(ShapeType::SimpleType)
    }

    /// Add a single `ShapeType` selector expression to this selector.
    pub fn add_any_simple_type(&mut self) -> &mut Self {
        self.add_shape(ShapeType::SimpleType)
    }

    /// Construct a single `ShapeType` selector expression.
    pub fn any_collection() -> Self {
        Self::shape(ShapeType::Collection)
    }

    /// Add a single `ShapeType` selector expression to this selector.
    pub fn add_any_collection(&mut self) -> &mut Self {
        self.add_shape(ShapeType::Collection)
    }

    /// Construct a single `ShapeType` selector expression.
    pub fn blob() -> Self {
        Self::shape(ShapeType::Blob)
    }

    /// Add a single `ShapeType` selector expression to this selector.
    pub fn add_blob(&mut self) -> &mut Self {
        self.add_shape(ShapeType::Blob)
    }

    /// Construct a single `ShapeType` selector expression.
    pub fn boolean() -> Self {
        Self::shape(ShapeType::Boolean)
    }

    /// Add a single `ShapeType` selector expression to this selector.
    pub fn add_boolean(&mut self) -> &mut Self {
        self.add_shape(ShapeType::Boolean)
    }

    /// Construct a single `ShapeType` selector expression.
    pub fn document() -> Self {
        Self::shape(ShapeType::Document)
    }

    /// Add a single `ShapeType` selector expression to this selector.
    pub fn add_document(&mut self) -> &mut Self {
        self.add_shape(ShapeType::Document)
    }

    /// Construct a single `ShapeType` selector expression.
    pub fn string() -> Self {
        Self::shape(ShapeType::String)
    }

    /// Add a single `ShapeType` selector expression to this selector.
    pub fn add_string(&mut self) -> &mut Self {
        self.add_shape(ShapeType::String)
    }

    /// Construct a single `ShapeType` selector expression.
    pub fn integer() -> Self {
        Self::shape(ShapeType::Integer)
    }

    /// Add a single `ShapeType` selector expression to this selector.
    pub fn add_integer(&mut self) -> &mut Self {
        self.add_shape(ShapeType::Integer)
    }

    /// Construct a single `ShapeType` selector expression.
    pub fn byte() -> Self {
        Self::shape(ShapeType::Byte)
    }

    /// Add a single `ShapeType` selector expression to this selector.
    pub fn add_byte(&mut self) -> &mut Self {
        self.add_shape(ShapeType::Byte)
    }

    /// Construct a single `ShapeType` selector expression.
    pub fn short() -> Self {
        Self::shape(ShapeType::Short)
    }

    /// Add a single `ShapeType` selector expression to this selector.
    pub fn add_short(&mut self) -> &mut Self {
        self.add_shape(ShapeType::Short)
    }

    /// Construct a single `ShapeType` selector expression.
    pub fn long() -> Self {
        Self::shape(ShapeType::Long)
    }

    /// Add a single `ShapeType` selector expression to this selector.
    pub fn add_long(&mut self) -> &mut Self {
        self.add_shape(ShapeType::Long)
    }

    /// Construct a single `ShapeType` selector expression.
    pub fn float() -> Self {
        Self::shape(ShapeType::Float)
    }

    /// Add a single `ShapeType` selector expression to this selector.
    pub fn add_float(&mut self) -> &mut Self {
        self.add_shape(ShapeType::Float)
    }

    /// Construct a single `ShapeType` selector expression.
    pub fn double() -> Self {
        Self::shape(ShapeType::Double)
    }

    /// Add a single `ShapeType` selector expression to this selector.
    pub fn add_double(&mut self) -> &mut Self {
        self.add_shape(ShapeType::Double)
    }

    /// Construct a single `ShapeType` selector expression.
    pub fn big_decimal() -> Self {
        Self::shape(ShapeType::BigDecimal)
    }

    /// Add a single `ShapeType` selector expression to this selector.
    pub fn add_big_decimal(&mut self) -> &mut Self {
        self.add_shape(ShapeType::BigDecimal)
    }

    /// Construct a single `ShapeType` selector expression.
    pub fn big_integer() -> Self {
        Self::shape(ShapeType::BigInteger)
    }

    /// Add a single `ShapeType` selector expression to this selector.
    pub fn add_big_integer(&mut self) -> &mut Self {
        self.add_shape(ShapeType::BigInteger)
    }

    /// Construct a single `ShapeType` selector expression.
    pub fn timestamp() -> Self {
        Self::shape(ShapeType::Timestamp)
    }

    /// Add a single `ShapeType` selector expression to this selector.
    pub fn add_timestamp(&mut self) -> &mut Self {
        self.add_shape(ShapeType::Timestamp)
    }

    /// Construct a single `ShapeType` selector expression.
    pub fn list() -> Self {
        Self::shape(ShapeType::List)
    }

    /// Add a single `ShapeType` selector expression to this selector.
    pub fn add_list(&mut self) -> &mut Self {
        self.add_shape(ShapeType::List)
    }

    /// Construct a single `ShapeType` selector expression.
    pub fn set() -> Self {
        Self::shape(ShapeType::Set)
    }

    /// Add a single `ShapeType` selector expression to this selector.
    pub fn add_set(&mut self) -> &mut Self {
        self.add_shape(ShapeType::Set)
    }

    /// Construct a single `ShapeType` selector expression.
    pub fn map() -> Self {
        Self::shape(ShapeType::Map)
    }

    /// Add a single `ShapeType` selector expression to this selector.
    pub fn add_map(&mut self) -> &mut Self {
        self.add_shape(ShapeType::Map)
    }

    /// Construct a single `ShapeType` selector expression.
    pub fn structure() -> Self {
        Self::shape(ShapeType::Structure)
    }

    /// Add a single `ShapeType` selector expression to this selector.
    pub fn add_structure(&mut self) -> &mut Self {
        self.add_shape(ShapeType::Structure)
    }

    /// Construct a single `ShapeType` selector expression.
    pub fn union() -> Self {
        Self::shape(ShapeType::Union)
    }

    /// Add a single `ShapeType` selector expression to this selector.
    pub fn add_union(&mut self) -> &mut Self {
        self.add_shape(ShapeType::Union)
    }

    /// Construct a single `ShapeType` selector expression.
    pub fn service() -> Self {
        Self::shape(ShapeType::Service)
    }

    /// Add a single `ShapeType` selector expression to this selector.
    pub fn add_service(&mut self) -> &mut Self {
        self.add_shape(ShapeType::Service)
    }

    /// Construct a single `ShapeType` selector expression.
    pub fn operation() -> Self {
        Self::shape(ShapeType::Operation)
    }

    /// Add a single `ShapeType` selector expression to this selector.
    pub fn add_operation(&mut self) -> &mut Self {
        self.add_shape(ShapeType::Operation)
    }

    /// Construct a single `ShapeType` selector expression.
    pub fn resource() -> Self {
        Self::shape(ShapeType::Resource)
    }

    /// Add a single `ShapeType` selector expression to this selector.
    pub fn add_resource(&mut self) -> &mut Self {
        self.add_shape(ShapeType::Resource)
    }

    /// Construct a single `ShapeType` selector expression.
    pub fn member() -> Self {
        Self::shape(ShapeType::Member)
    }

    /// Add a single `ShapeType` selector expression to this selector.
    pub fn add_member(&mut self) -> &mut Self {
        self.add_shape(ShapeType::Member)
    }

    /// Construct a single `Function` selector expression.
    pub fn function(v: Function) -> Self {
        Self::from(v.into())
    }

    /// Add a single `Function` selector expression to this selector.
    pub fn add_function(&mut self, v: Function) -> &mut Self {
        self.add(v.into())
    }

    /// Construct a single `Function` selector expression.
    pub fn function_from(name: Identifier, arguments: &[SelectorBuilder]) -> Self {
        let arguments = arguments
            .iter()
            .cloned()
            .map(|e| e.into())
            .collect::<Vec<Selector>>();
        Self::function(Function::new(name, &arguments))
    }

    /// Add a single `Function` selector expression to this selector.
    pub fn add_function_from(
        &mut self,
        name: Identifier,
        arguments: &[SelectorBuilder],
    ) -> &mut Self {
        let arguments = arguments
            .iter()
            .cloned()
            .map(|e| e.into())
            .collect::<Vec<Selector>>();
        self.add(Function::new(name, &arguments).into())
    }

    /// Add a single named `Function` selector expression to this selector.
    pub fn fn_test(arguments: &[SelectorBuilder]) -> Self {
        Self::function_from(Identifier::new_unchecked("test"), arguments)
    }

    /// Add a single named `Function` selector expression to this selector.
    pub fn add_test_function(&mut self, arguments: &[SelectorBuilder]) -> &mut Self {
        self.add_function_from(Identifier::new_unchecked("test"), arguments)
    }

    /// Add a single named `Function` selector expression to this selector.
    pub fn fn_is(arguments: &[SelectorBuilder]) -> Self {
        Self::function_from(Identifier::new_unchecked("is"), arguments)
    }

    /// Add a single named `Function` selector expression to this selector.
    pub fn add_is_function(&mut self, arguments: &[SelectorBuilder]) -> &mut Self {
        self.add_function_from(Identifier::new_unchecked("is"), arguments)
    }

    /// Add a single named `Function` selector expression to this selector.
    pub fn fn_not(arguments: &[SelectorBuilder]) -> Self {
        Self::function_from(Identifier::new_unchecked("not"), arguments)
    }

    /// Add a single named `Function` selector expression to this selector.
    pub fn add_not_function(&mut self, arguments: &[SelectorBuilder]) -> &mut Self {
        self.add_function_from(Identifier::new_unchecked("not"), arguments)
    }

    /// Add a single named `Function` selector expression to this selector.
    pub fn fn_topdown(arguments: &[SelectorBuilder]) -> Self {
        Self::function_from(Identifier::new_unchecked("topdown"), arguments)
    }

    /// Add a single named `Function` selector expression to this selector.
    pub fn add_topdown_function(&mut self, arguments: &[SelectorBuilder]) -> &mut Self {
        self.add_function_from(Identifier::new_unchecked("topdown"), arguments)
    }

    /// Construct a single `NeighborSelector` selector expression.
    pub fn neighbor(neighbor: NeighborSelector) -> Self {
        Self::from(neighbor.into())
    }

    /// Add a single `NeighborSelector` selector expression to this selector.
    pub fn add_neighbor(&mut self, neighbor: NeighborSelector) -> &mut Self {
        self.add(neighbor.into())
    }

    /// Construct a single `NeighborSelector` selector expression.
    pub fn forward_undirected() -> Self {
        Self::neighbor(NeighborSelector::ForwardUndirected)
    }

    /// Add a single `NeighborSelector` selector expression to this selector.
    pub fn add_forward_undirected(&mut self) -> &mut Self {
        self.add_neighbor(NeighborSelector::ForwardUndirected)
    }

    /// Construct a single `NeighborSelector` selector expression.
    pub fn reverse_undirected() -> Self {
        Self::neighbor(NeighborSelector::ReverseUndirected)
    }

    /// Add a single `NeighborSelector` selector expression to this selector.
    pub fn add_reverse_undirected(&mut self) -> &mut Self {
        self.add_neighbor(NeighborSelector::ReverseUndirected)
    }

    /// Construct a single `NeighborSelector` selector expression.
    pub fn forward_recursive_directed() -> Self {
        Self::neighbor(NeighborSelector::ForwardRecursiveDirected)
    }

    /// Add a single `NeighborSelector` selector expression to this selector.
    pub fn add_forward_recursive_directed(&mut self) -> &mut Self {
        self.add_neighbor(NeighborSelector::ForwardRecursiveDirected)
    }

    /// Construct a single `NeighborSelector` selector expression.
    pub fn forward_directed(path: &[Identifier]) -> Self {
        Self::neighbor(NeighborSelector::ForwardDirected(path.to_vec()))
    }

    /// Add a single `NeighborSelector` selector expression to this selector.
    pub fn add_forward_directed(&mut self, path: &[Identifier]) -> &mut Self {
        self.add_neighbor(NeighborSelector::ForwardDirected(path.to_vec()))
    }

    /// Construct a single `NeighborSelector` selector expression.
    pub fn reverse_directed(path: &[Identifier]) -> Self {
        Self::neighbor(NeighborSelector::ReverseDirected(path.to_vec()))
    }

    /// Add a single `Neighbor` selector expression to this selector.
    pub fn add_reverse_directed(&mut self, path: &[Identifier]) -> &mut Self {
        self.add_neighbor(NeighborSelector::ReverseDirected(path.to_vec()))
    }

    /// Construct a single `VariableReference` selector expression.
    pub fn variable_reference(v: VariableReference) -> Self {
        Self::from(v.into())
    }

    /// Add a single `NeighborSelector` selector expression to this selector.
    pub fn add_variable_reference(&mut self, v: VariableReference) -> &mut Self {
        self.add(v.into())
    }

    /// Construct a single `VariableReference` selector expression.
    pub fn variable_reference_from(name: Identifier) -> Self {
        Self::variable_reference(VariableReference::new(name))
    }

    /// Add a single `VariableReference` selector expression to this selector.
    pub fn add_variable_reference_from(&mut self, name: Identifier) -> &mut Self {
        self.add(VariableReference::new(name).into())
    }

    /// Construct a single `VariableDefinition` selector expression.
    pub fn variable_definition(v: VariableDefinition) -> Self {
        Self::from(v.into())
    }

    /// Add a single `VariableDefinition` selector expression to this selector.
    pub fn add_variable_definition(&mut self, v: VariableDefinition) -> &mut Self {
        self.add(v.into())
    }

    /// Construct a single `VariableDefinition` selector expression.
    pub fn variable_definition_from(name: Identifier, selector: SelectorBuilder) -> Self {
        Self::variable_definition(VariableDefinition::new(name, selector.into()))
    }

    /// Add a single `VariableDefinition` selector expression to this selector.
    pub fn add_variable_definition_from(
        &mut self,
        name: Identifier,
        selector: SelectorBuilder,
    ) -> &mut Self {
        self.add(VariableDefinition::new(name, selector.into()).into())
    }

    /// Construct a single `AttributeSelector` selector expression.
    pub fn attribute(attribute: AttributeSelector) -> Self {
        Self::from(attribute.into())
    }

    /// Add a single `AttributeSelector` selector expression to this selector.
    pub fn add_attribute(&mut self, attribute: AttributeSelector) -> &mut Self {
        self.add(attribute.into())
    }

    /// Add a single `AttributeSelector` selector expression to this selector.
    pub fn add_attribute_from(
        &mut self,
        key: Key,
        comparison: Option<AttributeComparison>,
    ) -> &mut Self {
        self.add(
            match comparison {
                None => AttributeSelector::new(key),
                Some(comparison) => AttributeSelector::with_comparison(key, comparison),
            }
            .into(),
        )
    }

    /// Construct a single `ScopedAttributeSelector` selector expression.
    pub fn scoped_attribute(attribute: ScopedAttributeSelector) -> Self {
        Self::from(attribute.into())
    }

    /// Add a single `ScopedAttributeSelector` selector expression to this selector.
    pub fn add_scoped_attribute(&mut self, attribute: ScopedAttributeSelector) -> &mut Self {
        self.add(attribute.into())
    }
}

// ------------------------------------------------------------------------------------------------

impl From<AttributeBuilder> for SelectorExpression {
    fn from(v: AttributeBuilder) -> Self {
        v.selector.into()
    }
}

impl From<AttributeBuilder> for SelectorBuilder {
    fn from(v: AttributeBuilder) -> Self {
        SelectorBuilder::from(v.into())
    }
}

impl From<&mut AttributeBuilder> for SelectorBuilder {
    fn from(v: &mut AttributeBuilder) -> Self {
        v.clone().into()
    }
}

impl From<&mut AttributeBuilder> for AttributeSelector {
    fn from(v: &mut AttributeBuilder) -> Self {
        v.selector.clone()
    }
}

impl From<AttributeBuilder> for AttributeSelector {
    fn from(v: AttributeBuilder) -> Self {
        v.selector
    }
}

impl From<&mut AttributeBuilder> for Selector {
    fn from(v: &mut AttributeBuilder) -> Self {
        v.clone().into()
    }
}

impl From<AttributeBuilder> for Selector {
    fn from(v: AttributeBuilder) -> Self {
        SelectorBuilder::from(v.into()).into()
    }
}

impl AttributeBuilder {
    /// Construct a new attribute builder for the provided key. In general it is easier to
    /// use one of `named_id`, `named_service`, `named_trait`, or `named_var`.
    pub fn named(identifier: Identifier) -> Self {
        Self {
            selector: AttributeSelector::new(Key::new(identifier)),
        }
    }

    /// Construct a new attribute builder with the key `id`.
    pub fn named_id() -> Self {
        Self {
            selector: AttributeSelector::new(Key::new(Identifier::new_unchecked("id"))),
        }
    }

    /// Construct a new attribute builder with the key `service`.
    pub fn named_service() -> Self {
        Self {
            selector: AttributeSelector::new(Key::new(Identifier::new_unchecked("service"))),
        }
    }

    /// Construct a new attribute builder with the key `trait`.
    pub fn named_trait() -> Self {
        Self {
            selector: AttributeSelector::new(Key::new(Identifier::new_unchecked("trait"))),
        }
    }

    /// Construct a new attribute builder with the key `var`.
    pub fn named_var() -> Self {
        Self {
            selector: AttributeSelector::new(Key::new(Identifier::new_unchecked("var"))),
        }
    }

    /// Set the key-path component to the set of provided segments.
    pub fn path(&mut self, path: &[KeyPathSegment]) -> &mut Self {
        let mut key = self.selector.key().clone();
        key.set_path(path);
        self.selector.set_key(key);
        self
    }

    /// Add the provided segment to the current key-path.
    pub fn path_segment(&mut self, segment: KeyPathSegment) -> &mut Self {
        let mut key = self.selector.key().clone();
        key.add_path_segment(segment);
        self.selector.set_key(key);
        self
    }

    /// Add the provided `Identifier` to the current key-path.
    pub fn path_segment_for_id(&mut self, segment: Identifier) -> &mut Self {
        self.path_segment(KeyPathSegment::Value(Value::RootShapeIdentifier(segment)))
    }

    /// Add the provided `ShapeID` to the current key-path.
    pub fn path_segment_for_shape(&mut self, segment: ShapeID) -> &mut Self {
        self.path_segment(KeyPathSegment::Value(Value::AbsoluteRootShapeIdentifier(
            segment,
        )))
    }

    /// Add the provided string as a `Value::Text` to the current key-path.
    pub fn path_segment_for_text(&mut self, segment: &str) -> &mut Self {
        self.path_segment(KeyPathSegment::Value(Value::Text(segment.to_string())))
    }

    /// Add the provided string as a `Value::Number` to the current key-path.
    pub fn path_segment_for_number(&mut self, segment: Number) -> &mut Self {
        self.path_segment(KeyPathSegment::Value(Value::Number(segment)))
    }

    /// Add the provided identifier as a `FunctionProperty` to the current key-path.
    pub fn path_segment_for_function(&mut self, segment: Identifier) -> &mut Self {
        self.path_segment(KeyPathSegment::FunctionProperty(segment))
    }

    /// Set the comparison part of this selector.
    pub fn compare(&mut self, comparison: AttributeComparison) -> &mut Self {
        self.selector.set_comparison(comparison);
        self
    }

    /// Set the comparison part of this selector to be a string comparison.
    pub fn string_compare(
        &mut self,
        comparator: Comparator,
        rhs: &[Value],
        case_insensitive: bool,
    ) -> &mut Self {
        self.compare(if case_insensitive {
            AttributeComparison::new_case_insensitive(comparator, rhs)
        } else {
            AttributeComparison::new(comparator, rhs)
        })
    }

    /// Set the comparison part of this selector to the string comparison "=".
    pub fn string_equal(&mut self, rhs: &[Value], case_insensitive: bool) -> &mut Self {
        self.string_compare(Comparator::StringEqual, rhs, case_insensitive)
    }

    /// Set the comparison part of this selector to the string comparison "!=".
    pub fn string_not_equal(&mut self, rhs: &[Value], case_insensitive: bool) -> &mut Self {
        self.string_compare(Comparator::StringNotEqual, rhs, case_insensitive)
    }

    /// Set the comparison part of this selector to the string comparison "^=".
    pub fn string_starts_with(&mut self, rhs: &[Value], case_insensitive: bool) -> &mut Self {
        self.string_compare(Comparator::StringStartsWith, rhs, case_insensitive)
    }

    /// Set the comparison part of this selector to the string comparison "$=".
    pub fn string_ends_with(&mut self, rhs: &[Value], case_insensitive: bool) -> &mut Self {
        self.string_compare(Comparator::StringEndsWith, rhs, case_insensitive)
    }

    /// Set the comparison part of this selector to the string comparison "*=".
    pub fn string_contains(&mut self, rhs: &[Value], case_insensitive: bool) -> &mut Self {
        self.string_compare(Comparator::StringContains, rhs, case_insensitive)
    }

    /// Set the comparison part of this selector to the string comparison "?=".
    pub fn string_exists(&mut self, rhs: bool, case_insensitive: bool) -> &mut Self {
        self.string_compare(
            Comparator::StringExists,
            &[Value::Text(rhs.to_string())],
            case_insensitive,
        )
    }

    /// Set the comparison part of this selector to the numeric comparison ">".
    pub fn number_greater(&mut self, rhs: &[Number]) -> &mut Self {
        self.compare(AttributeComparison::new(
            Comparator::NumberGreaterThan,
            &rhs.iter()
                .cloned()
                .map(Value::Number)
                .collect::<Vec<Value>>(),
        ))
    }

    /// Set the comparison part of this selector to the numeric comparison ">=".
    pub fn number_greater_or_equal(&mut self, rhs: &[Number]) -> &mut Self {
        self.compare(AttributeComparison::new(
            Comparator::NumberGreaterOrEqual,
            &rhs.iter()
                .cloned()
                .map(Value::Number)
                .collect::<Vec<Value>>(),
        ))
    }

    /// Set the comparison part of this selector to the numeric comparison "<".
    pub fn number_less(&mut self, rhs: &[Number]) -> &mut Self {
        self.compare(AttributeComparison::new(
            Comparator::NumberLessThan,
            &rhs.iter()
                .cloned()
                .map(Value::Number)
                .collect::<Vec<Value>>(),
        ))
    }

    /// Set the comparison part of this selector to the numeric comparison "<=".
    pub fn number_less_or_equal(&mut self, rhs: &[Number]) -> &mut Self {
        self.compare(AttributeComparison::new(
            Comparator::NumberLessOrEqual,
            &rhs.iter()
                .cloned()
                .map(Value::Number)
                .collect::<Vec<Value>>(),
        ))
    }

    /// Set the comparison part of this selector to the projection comparison "{=}".
    pub fn projection_equal(&mut self, rhs: &[Value]) -> &mut Self {
        self.compare(AttributeComparison::new(Comparator::ProjectionEqual, rhs))
    }

    /// Set the comparison part of this selector to the projection comparison "{!=}".
    pub fn projection_not_equal(&mut self, rhs: &[Value]) -> &mut Self {
        self.compare(AttributeComparison::new(
            Comparator::ProjectionNotEqual,
            rhs,
        ))
    }

    /// Set the comparison part of this selector to the projection comparison "{<}".
    pub fn projection_subset(&mut self, rhs: &[Value]) -> &mut Self {
        self.compare(AttributeComparison::new(Comparator::ProjectionSubset, rhs))
    }

    /// Set the comparison part of this selector to the projection comparison "{<<}".
    pub fn projection_proper_subset(&mut self, rhs: &[Value]) -> &mut Self {
        self.compare(AttributeComparison::new(
            Comparator::ProjectionProperSubset,
            rhs,
        ))
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for ScopedAttributeBuilder {
    fn default() -> Self {
        Self {
            key: None,
            assertions: Default::default(),
        }
    }
}

impl From<ScopedAttributeBuilder> for SelectorExpression {
    fn from(v: ScopedAttributeBuilder) -> Self {
        assert!(!v.assertions.is_empty());
        ScopedAttributeSelector::from(v).into()
    }
}

impl From<ScopedAttributeBuilder> for SelectorBuilder {
    fn from(v: ScopedAttributeBuilder) -> Self {
        SelectorBuilder::from(v.into())
    }
}

impl From<&mut ScopedAttributeBuilder> for SelectorBuilder {
    fn from(v: &mut ScopedAttributeBuilder) -> Self {
        v.clone().into()
    }
}

impl From<&mut ScopedAttributeBuilder> for ScopedAttributeSelector {
    fn from(v: &mut ScopedAttributeBuilder) -> Self {
        match &v.key {
            None => ScopedAttributeSelector::new(&v.assertions),
            Some(key) => ScopedAttributeSelector::with_key(key.clone(), &v.assertions),
        }
    }
}

impl From<ScopedAttributeBuilder> for ScopedAttributeSelector {
    fn from(v: ScopedAttributeBuilder) -> Self {
        let mut v = v;
        ScopedAttributeSelector::from(&mut v)
    }
}

impl From<&mut ScopedAttributeBuilder> for Selector {
    fn from(v: &mut ScopedAttributeBuilder) -> Self {
        v.clone().into()
    }
}

impl From<ScopedAttributeBuilder> for Selector {
    fn from(v: ScopedAttributeBuilder) -> Self {
        SelectorBuilder::from(v.into()).into()
    }
}

impl ScopedAttributeBuilder {
    /// Construct a new scoped attribute builder for the provided key. In general it is easier to
    /// use one of `named_id`, `named_service`, `named_trait`, or `named_var`.
    pub fn named(identifier: Identifier) -> Self {
        Self {
            key: Some(Key::new(identifier)),
            assertions: Default::default(),
        }
    }

    /// Construct a new attribute builder with the key `id`.
    pub fn new_id() -> Self {
        Self {
            key: Some(Key::new(Identifier::new_unchecked("id"))),
            assertions: Default::default(),
        }
    }

    /// Construct a new scoped attribute builder with the key `service`.
    pub fn new_service() -> Self {
        Self {
            key: Some(Key::new(Identifier::new_unchecked("service"))),
            assertions: Default::default(),
        }
    }

    /// Construct a new scoped attribute builder with the key `trait`.
    pub fn new_trait() -> Self {
        Self {
            key: Some(Key::new(Identifier::new_unchecked("trait"))),
            assertions: Default::default(),
        }
    }

    /// Construct a new scoped attribute builder with the key `var`.
    pub fn new_var() -> Self {
        Self {
            key: Some(Key::new(Identifier::new_unchecked("var"))),
            assertions: Default::default(),
        }
    }

    /// Set the key-path component to the set of provided segments.
    pub fn path(&mut self, path: &[KeyPathSegment]) -> &mut Self {
        match &mut self.key {
            Some(key) => key.set_path(path),
            None => panic!("No key set, setting path is invalid"),
        }
        self
    }

    /// Add the provided segment to the current key-path.
    pub fn path_segment(&mut self, segment: KeyPathSegment) -> &mut Self {
        match &mut self.key {
            Some(key) => key.add_path_segment(segment),
            None => panic!("No key set, setting path is invalid"),
        }
        self
    }

    /// Add the provided `Identifier` to the current key-path.
    pub fn path_segment_for_id(&mut self, segment: Identifier) -> &mut Self {
        self.path_segment(KeyPathSegment::Value(Value::RootShapeIdentifier(segment)))
    }

    /// Add the provided `ShapeID` to the current key-path.
    pub fn path_segment_for_shape(&mut self, segment: ShapeID) -> &mut Self {
        self.path_segment(KeyPathSegment::Value(Value::AbsoluteRootShapeIdentifier(
            segment,
        )))
    }

    /// Add the provided string as a `Value::Text` to the current key-path.
    pub fn path_segment_for_text(&mut self, segment: &str) -> &mut Self {
        self.path_segment(KeyPathSegment::Value(Value::Text(segment.to_string())))
    }

    /// Add the provided string as a `Value::Number` to the current key-path.
    pub fn path_segment_for_number(&mut self, segment: Number) -> &mut Self {
        self.path_segment(KeyPathSegment::Value(Value::Number(segment)))
    }

    /// Add the provided identifier as a `FunctionProperty` to the current key-path.
    pub fn path_segment_for_function(&mut self, segment: Identifier) -> &mut Self {
        self.path_segment(KeyPathSegment::FunctionProperty(segment))
    }

    /// Add the provided scoped attribute assertion to this selector.
    pub fn assertion(&mut self, assertion: ScopedAttributeAssertion) -> &mut Self {
        self.assertions.push(assertion);
        self
    }

    /// Create a new string assertion between `lhs` and `rhs` with the provided operator.
    pub fn string_assertion(
        &mut self,
        lhs: ScopedValue,
        comparator: Comparator,
        rhs: &[ScopedValue],
        case_insensitive: bool,
    ) -> &mut Self {
        self.assertion(if case_insensitive {
            ScopedAttributeAssertion::new_case_insensitive(lhs, comparator, rhs)
        } else {
            ScopedAttributeAssertion::new(lhs, comparator, rhs)
        })
    }

    /// Create a new assertion between `lhs` and `rhs` with the `=` operator.
    pub fn string_equal(
        &mut self,
        lhs: ScopedValue,
        rhs: ScopedValue,
        case_insensitive: bool,
    ) -> &mut Self {
        self.string_assertion(lhs, Comparator::StringEqual, &[rhs], case_insensitive)
    }

    /// Create a new assertion between `lhs` and `rhs` with the `!=` operator.
    pub fn string_not_equal(
        &mut self,
        lhs: ScopedValue,
        rhs: ScopedValue,
        case_insensitive: bool,
    ) -> &mut Self {
        self.string_assertion(lhs, Comparator::StringNotEqual, &[rhs], case_insensitive)
    }

    /// Create a new assertion between `lhs` and `rhs` with the `^=` operator.
    pub fn string_starts_with(
        &mut self,
        lhs: ScopedValue,
        rhs: ScopedValue,
        case_insensitive: bool,
    ) -> &mut Self {
        self.string_assertion(lhs, Comparator::StringStartsWith, &[rhs], case_insensitive)
    }

    /// Create a new assertion between `lhs` and `rhs` with the `$=` operator.
    pub fn string_ends_with(
        &mut self,
        lhs: ScopedValue,
        rhs: ScopedValue,
        case_insensitive: bool,
    ) -> &mut Self {
        self.string_assertion(lhs, Comparator::StringEndsWith, &[rhs], case_insensitive)
    }

    /// Create a new assertion between `lhs` and `rhs` with the `*=` operator.
    pub fn string_contains(
        &mut self,
        lhs: ScopedValue,
        rhs: ScopedValue,
        case_insensitive: bool,
    ) -> &mut Self {
        self.string_assertion(lhs, Comparator::StringContains, &[rhs], case_insensitive)
    }

    /// Create a new assertion between `lhs` and `rhs` with the `?=` operator.
    pub fn string_exists(&mut self, lhs: ScopedValue, rhs: bool) -> &mut Self {
        self.string_assertion(
            lhs,
            Comparator::StringExists,
            &[Value::Text(rhs.to_string()).into()],
            false,
        )
    }

    /// Create a new assertion between `lhs` and `rhs` with the `>` operator.
    pub fn number_greater(&mut self, lhs: ScopedValue, rhs: Number) -> &mut Self {
        self.assertion(ScopedAttributeAssertion::new(
            lhs,
            Comparator::NumberGreaterThan,
            &[Value::Text(rhs.to_string()).into()],
        ))
    }

    /// Create a new assertion between `lhs` and `rhs` with the `>=` operator.
    pub fn number_greater_or_equal(&mut self, lhs: ScopedValue, rhs: Number) -> &mut Self {
        self.assertion(ScopedAttributeAssertion::new(
            lhs,
            Comparator::NumberGreaterOrEqual,
            &[Value::Text(rhs.to_string()).into()],
        ))
    }

    /// Create a new assertion between `lhs` and `rhs` with the `<` operator.
    pub fn number_less(&mut self, lhs: ScopedValue, rhs: Number) -> &mut Self {
        self.assertion(ScopedAttributeAssertion::new(
            lhs,
            Comparator::NumberLessThan,
            &[Value::Text(rhs.to_string()).into()],
        ))
    }

    /// Create a new assertion between `lhs` and `rhs` with the `<=` operator.
    pub fn number_less_or_equal(&mut self, lhs: ScopedValue, rhs: Number) -> &mut Self {
        self.assertion(ScopedAttributeAssertion::new(
            lhs,
            Comparator::NumberLessOrEqual,
            &[Value::Text(rhs.to_string()).into()],
        ))
    }

    /// Create a new assertion between `lhs` and `rhs` with the `{=}` operator.
    pub fn projection_equal(&mut self, lhs: ScopedValue, rhs: ScopedValue) -> &mut Self {
        self.assertion(ScopedAttributeAssertion::new(
            lhs,
            Comparator::ProjectionEqual,
            &[rhs],
        ))
    }

    /// Create a new assertion between `lhs` and `rhs` with the `{!=}` operator.
    pub fn projection_not_equal(&mut self, lhs: ScopedValue, rhs: ScopedValue) -> &mut Self {
        self.assertion(ScopedAttributeAssertion::new(
            lhs,
            Comparator::ProjectionNotEqual,
            &[rhs],
        ))
    }

    /// Create a new assertion between `lhs` and `rhs` with the `{<}` operator.
    pub fn projection_subset(&mut self, lhs: ScopedValue, rhs: &[ScopedValue]) -> &mut Self {
        self.assertion(ScopedAttributeAssertion::new(
            lhs,
            Comparator::ProjectionSubset,
            rhs,
        ))
    }

    /// Create a new assertion between `lhs` and `rhs` with the `{<<}` operator.
    pub fn projection_proper_subset(&mut self, lhs: ScopedValue, rhs: &[ScopedValue]) -> &mut Self {
        self.assertion(ScopedAttributeAssertion::new(
            lhs,
            Comparator::ProjectionProperSubset,
            rhs,
        ))
    }
}
