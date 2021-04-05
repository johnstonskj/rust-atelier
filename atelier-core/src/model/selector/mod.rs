/*!
This module provides a model to construct `Selector`s. These are described in §14 of the Smithy
specification.
*/

use crate::error::Error;
use crate::error::ErrorKind::InvalidSelector;
use crate::model::values::Number;
use crate::model::{Identifier, ShapeID};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A Selector expression that matches simply by shape type.
///
#[derive(Clone, Debug, PartialEq)]
pub enum ShapeType {
    /// Matches all shapes
    All,
    /// Matches all `byte`, `short`, `integer`, `long`, `float`, `double`, `bigDecimal`, and `bigInteger` shapes
    Number,
    /// Matches all simple types
    SimpleType,
    /// Matches both a `list` and `set` shape
    Collection,
    /// Matches `blob` shapes
    Blob,
    /// Matches `boolean` shapes
    Boolean,
    /// Matches `document` shapes
    Document,
    /// Matches `string` shapes
    String,
    /// Matches `integer` shapes
    Integer,
    /// Matches `byte` shapes
    Byte,
    /// Matches `short` shapes
    Short,
    /// Matches `long` shapes
    Long,
    /// Matches `float` shapes
    Float,
    /// Matches `double` shapes
    Double,
    /// Matches `bigDecimal` shapes
    BigDecimal,
    /// Matches `bigInteger` shapes
    BigInteger,
    /// Matches `timestamp` shapes
    Timestamp,
    /// Matches `list` shapes
    List,
    /// Matches `set` shapes
    Set,
    /// Matches `map` shapes
    Map,
    /// Matches `structure` shapes
    Structure,
    /// Matches `union` shapes
    Union,
    /// Matches `service` shapes
    Service,
    /// Matches `operation` shapes
    Operation,
    /// Matches `resource` shapes
    Resource,
    /// Matches `member` shapes
    Member,
}

///
/// This denotes a literal value in an expression.
///
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    /// A text, or string, literal value.
    Text(String),
    /// A numeric literal value.
    Number(Number),
    /// A shape identifier literal value.
    RootShapeIdentifier(Identifier),
    /// An absolute shape identifier literal value.
    AbsoluteRootShapeIdentifier(ShapeID),
}

///
/// Use in the `path` field of the `Key` struct.
#[derive(Clone, Debug, PartialEq)]
pub enum KeyPathSegment {
    /// A literal value.
    Value(Value),
    /// A functional property.
    FunctionProperty(Identifier),
}

///
/// A key used to anchor certain expression types.
///
#[derive(Clone, Debug, PartialEq)]
pub struct Key {
    identifier: Identifier,
    path: Vec<KeyPathSegment>,
}

///
/// An attribute selector with a comparator checks for the existence of an attribute and compares
/// the resolved attribute value to a comma separated list of possible values. The resolved
/// attribute value on the left hand side of the comparator MUST match one or more of the comma
/// separated values on the right hand side of the comparator.
///
/// * String comparators are used to compare the string representation of values. Attributes that
///   do not have a string representation are treated as an empty string when these comparisons are
///   performed.
/// * Relative comparators only match if both values being compared contain valid number productions
///   when converted to a string. If either value is not a valid number, then the selector does not
///   match.
/// * Projection comparators are used to compare projections to test if they are equal, not equal,
///   a subset, or a proper subset to another projection. With the exception of the `{!=}`
///   comparator, projection comparators match if and only if both the left hand side of the
///   comparator and the right hand side of the comparator are projections.
///
#[derive(Clone, Debug, PartialEq)]
pub enum Comparator {
    ///
    /// Matches if the attribute value is equal to the comparison value. This comparator never
    /// matches if either value does not exist.
    ///
    /// The following selector matches shapes in the "smithy.example" namespace.
    ///
    /// `[id|namespace = 'smithy.example']`
    ///
    /// The following selector matches shapes that have the since trait with a value of 2019 or 2020:
    ///
    /// `[trait|since = 2019, 2020]`
    ///
    StringEqual,
    ///
    /// Matches if the attribute value is not equal to the comparison value. This comparator never
    /// matches if either value does not exist.
    ///
    /// The following selector matches shapes that are not in the "smithy.example" namespace.
    ///
    /// `[id|namespace != 'smithy.example']`
    ///
    StringNotEqual,
    ///
    /// Matches if the attribute value starts with the comparison value. This comparator never
    /// matches if either value does not exist.
    ///
    /// The following selector matches shapes where the name starts with "_".
    ///
    /// `[id|name ^= '_']`
    ///
    StringStartsWith,
    ///
    /// Matches if the attribute value ends with the comparison value. This comparator never matches
    /// if either value does not exist.
    ///
    /// The following selector matches shapes where the name ends with "_".
    ///
    /// `[trait|required $= '_']`
    ///
    StringEndsWith,
    ///
    /// Matches if the attribute value contains the comparison value. This comparator never matches
    /// if either value does not exist.
    ///
    /// The following selector matches shapes where the name contains "_".
    ///
    /// `[id|name *= '_']`
    ///
    StringContains,
    ///
    /// Matches based on the existence of a value. This comparator uses the same rules defined in
    /// Attribute existence. The comparator matches if the value exists and the right hand side of
    /// the comparator is true, or if the value does not exist and the right hand side of the
    /// comparator is set to false. This selector is most useful in scoped attribute selectors.
    ///
    /// The following selector matches shapes marked as required.
    ///
    /// `[trait|required ?= true]`
    ///
    StringExists,
    ///
    /// Matches if the attribute value is greater than the comparison value.
    ///
    /// The following selector matches shapes with an httpError trait value that is greater than 500:
    ///
    /// `[trait|httpError > 500]`
    ///
    NumberGreaterThan,
    ///
    /// Matches if the attribute value is greater than or equal to the comparison value.
    ///
    NumberGreaterOrEqual,
    ///
    /// Matches if the attribute value is less than the comparison value.
    ///
    NumberLessThan,
    ///
    /// Matches if the attribute value is less than or equal to the comparison value.
    ///
    NumberLessOrEqual,
    ///
    /// Matches if every value in the left hand side can be found in the right hand side using the
    /// `=` comparator for equality. Projection comparisons are unordered, and the projections are
    /// not required to have the same number of items.
    ///
    ProjectionEqual,
    ///
    /// This comparator is the negation of the result of `{=}`. Comparing a projection to a
    /// non-projection value will always return true.
    ///
    ProjectionNotEqual,
    ///
    /// Matches if the left projection is a subset of the right projection. Every value in the left
    /// projection MUST be found in the right projection using the `=` comparator for equality.
    ///
    ProjectionSubset,
    ///
    /// Matches if the left projection is a *proper subset* of the right projection. Every value in
    /// the left projection MUST be found in the right projection using the `=` comparator for
    /// equality, but the projections themselves are not equal, meaning that the left projection is
    /// missing one or more values found in the right projection.
    ///
    ProjectionProperSubset,
}

///
/// An attribute selector with a comparator checks for the existence of an attribute and compares
/// the resolved attribute value to a comma separated list of possible values. The resolved
/// attribute value on the left hand side of the comparator MUST match one or more of the comma
/// separated values on the right hand side of the comparator.
///
#[derive(Clone, Debug, PartialEq)]
pub struct AttributeComparison {
    comparator: Comparator,
    rhs_values: Vec<Value>,
    case_insensitive: bool,
}

///
/// Attribute selectors are used to match shapes based on shape IDs, traits, and other attributes.
///
#[derive(Clone, Debug, PartialEq)]
pub struct AttributeSelector {
    key: Key,
    comparison: Option<AttributeComparison>,
}

///
/// A scoped attribute selector is similar to an attribute selector, but it allows multiple complex
/// comparisons to be made against a scoped attribute.
///
#[derive(Clone, Debug, PartialEq)]
pub struct ScopedAttributeSelector {
    key: Option<Key>,
    assertions: Vec<ScopedAttributeAssertion>,
}

///
/// Values used in scoped attribute assertions.
///
#[derive(Clone, Debug, PartialEq)]
pub enum ScopedValue {
    /// A literal value
    Value(Value),
    /// A context value
    ContextValue(Vec<KeyPathSegment>),
}

///
/// The first part of a scoped attribute selector is the attribute that is scoped for the
/// expression, followed by :. The scoped attribute is accessed using a context value in the form of
/// `z@{ identifier }`.
///
/// In the following selector, the trait|range attribute is used as the scoped attribute of the
/// expression, and the selector matches shapes marked with the `range` trait where the `min` value
/// is greater than the `max` value:
///
/// `[@trait|range: @{min} > @{max}]`
///
/// The scope can also be set to the current shape being evaluated by omitting an expression before
/// the : character.
///
/// The following selector matches shapes that are traits that are applied to themselves as traits
/// (for example, this matches `smithy.api#trait`, `smithy.api#documentation`, etc.):
///
/// `[trait|trait][@: @{trait|(keys)} = @{id}]`
///
/// A projection MAY be used as the scoped attribute context value. When the scoped attribute
/// context value is a projection, each recursively flattened value of the projection is
/// individually tested against each assertion. If any value from the projection matches the
/// assertions, then the selector matches the shape.
///
/// The following selector matches shapes that have an enum trait where one or more of the enum
/// definitions is both marked as `deprecated` and contains an entry in its tags property named
/// `deprecated`.
///
/// `[@trait|enum|(values):
///     @{deprecated} = true &&
///     @{tags|(values)} = "deprecated"]`
///
#[derive(Clone, Debug, PartialEq)]
pub struct ScopedAttributeAssertion {
    lhs_value: ScopedValue,
    comparator: Comparator,
    rhs_values: Vec<ScopedValue>,
    case_insensitive: bool,
}

///
/// Neighbor selectors yield shapes that are connected to the current shape. Most selectors are used
/// to determine if a shape matches some criteria, meaning the selector yields zero or exactly one
/// shape. However, neighbor selectors yield zero or more shapes by traversing the relationships of
/// a shape.
///
#[derive(Clone, Debug, PartialEq)]
pub enum NeighborSelector {
    ///
    /// A *forward undirected neighbor* (`>`) yields every shape that is connected to the current
    /// shape. For example, the following selector matches the key and value members of every map:
    ///
    /// `map > member`
    ///
    /// Neighbors can be chained to traverse further into a shape. The following selector yields
    /// strings that are targeted by list members:
    ///
    /// `list > member > string`
    ///
    ForwardUndirected,
    ///
    /// A *reverse undirected neighbor* yields all of the shapes that have a relationship that
    ///
    /// points to the current shape.
    //
    /// The following selector matches strings that are targeted by members of lists:
    ///
    /// `string :test(< member < list)`
    ///
    /// The following selector yields all shapes that are not traits and are not referenced by
    /// other shapes:
    ///
    /// `:not([trait|trait]) :not(< *)`
    ///
    /// The following selectors are equivalent; however, a forward neighbor traversal is preferred
    /// over a reverse neighbor traversal when possible.
    ///
    /// * Reverse: `string < member < list`
    /// * Forward: `list :test(> member > string)`
    ///
    ReverseUndirected,
    ///
    /// The *forward undirected neighbor selector* (`>`) is an undirected edge traversal. Sometimes,
    /// a directed edge traversal is necessary. For example, the following selector matches the
    /// "bound", "input", "output", and "error" relationships of each operation:
    ///
    /// `operation > *`
    ///
    /// A forward directed edge traversal is applied using selector_forward_directed_neighbor
    /// (`-[X, Y, Z]->`). The following selector matches all structure shapes referenced as  
    /// operation input or output.
    ///
    /// `operation -[input, output]-> structure`
    ///
    /// The `:test` function can be used to check if a shape has a named relationship. The following  
    /// selector matches all resource shapes that define an identifier:
    ///
    /// `resource :test(-[identifier]->)`
    ///
    /// Relationships from a shape to the traits applied to the shape can be traversed using a
    /// forward directed relationship named trait. It is atypical to traverse trait relationships,
    /// therefore they are only yielded by selectors when explicitly requested using a trait
    /// directed relationship. The following selector finds all service shapes that have a protocol
    /// trait applied to it (that is, a trait that is marked with the `protocolDefinition` trait):
    ///
    /// `service :test(-[trait]-> [trait|protocolDefinition])`
    ///
    ForwardDirected(Vec<Identifier>),
    ///
    /// A *reverse directed neighbor* yields all of the shapes that have a relationship of a
    /// specific type that points to the current shape.
    ///
    /// For example, shapes marked with the streaming trait can only be targeted by top-level
    /// members of operation input or output structures. The following selector finds all shapes
    /// that target a streaming shape and violate this constraint:
    ///
    /// `[trait|streaming]
    ///     :test(<)
    ///     :not(< member < structure <-[input, output]- operation)`
    ///
    /// Like forward directed neighbors, trait relationships are only included when explicitly
    /// provided in the list of relationships to traverse. The following selector yields all
    /// shapes that are traits that are not applied to any shapes:
    ///
    /// `[trait|trait] :not(<-[trait]-)`
    ///
    ReverseDirected(Vec<Identifier>),
    ///
    /// The *forward recursive neighbor* selector (`~>`) yields all shapes that are recursively
    /// connected in the closure of another shape. The shapes yielded by this selector are
    /// equivalent to yielding every shape connected to the current shape using a forward undirected
    /// neighbor, yielding every shape connected to those shapes, and so on.
    ///
    /// The following selector matches operations that are connected to a service:
    ///
    /// `service ~> operation`
    ///
    /// The following selector finds operations that do not have the http trait that are in the
    /// closure of a service marked with the `aws.protocols#restJson` trait:
    ///
    /// `service[trait|aws.protocols#restJson1]
    ///     ~> operation:not([trait|http])`
    ///
    ForwardRecursiveDirected,
}

///
/// Functions are used to filter and yield shapes using a variadic argument list of selectors
/// separated by a comma (,). Functions always start with a colon (:).
///
/// > **Important**: Implementations MUST tolerate parsing unknown function names. When evaluated,
/// > an unknown function yields no shapes.
///
#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    name: Identifier,
    arguments: Vec<SelectorExpression>,
}

///
/// A variable is set using a *selector_variable_set* expression. Variables can be reassigned
/// without error.
///
/// The following selector defines a variable named foo that sets the variable to the result of
/// applying the `*` selector to the current shape.
///
/// `$foo(*)`
///
#[derive(Clone, Debug, PartialEq)]
pub struct VariableDefinition {
    name: Identifier,
    expressions: Vec<SelectorExpression>,
}

///
/// A variable is retrieved by name using a *selector_variable_get* expression. Retrieving a
/// variable yields the set of shapes stored in the variable. Attempting to get a variable that
/// does not exist yields no shapes.
///
/// `${foo}`
///
/// Variables can also be accessed inside of scoped attribute selectors from shapes using the `var`
/// attribute.
///
#[derive(Clone, Debug, PartialEq)]
pub struct VariableReference {
    name: Identifier,
}

///
/// This enum represents the set of possible selector expressions that comprise the `Selector`.
#[derive(Clone, Debug, PartialEq)]
pub enum SelectorExpression {
    /// Match by shape type.
    ShapeType(ShapeType),
    /// Attribute selectors are used to match shapes based on shape IDs, traits, and other attributes.
    AttributeSelector(AttributeSelector),
    /// A scoped attribute selector is similar to an attribute selector, but it allows multiple
    /// complex comparisons to be made against a scoped attribute.
    ScopedAttributeSelector(ScopedAttributeSelector),
    /// Neighbor selectors yield shapes that are connected to the current shape.
    NeighborSelector(NeighborSelector),
    /// Functions are used to filter and yield shapes using a variadic argument list of selectors
    /// separated by a comma (,).
    Function(Function),
    /// Variables are used to store eagerly computed, named intermediate results that can be
    /// accessed later in a selector.
    VariableDefinition(VariableDefinition),
    /// Retrieving a variable yields the set of shapes stored in the variable.
    VariableReference(VariableReference),
}

///
/// Selectors can be composed of multiple *selector expressions*. Selectors are evaluated from left
/// to right, yielding the results from one selector to the next. The following selector matches
/// all string shapes marked as sensitive:
///
/// `string [trait|sensitive]`
///
#[derive(Clone, Debug, PartialEq)]
pub struct Selector {
    expressions: Vec<SelectorExpression>,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for ShapeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ShapeType::All => "*",
                ShapeType::Number => "number",
                ShapeType::SimpleType => "simpleType",
                ShapeType::Collection => "collection",
                ShapeType::Blob => "blob",
                ShapeType::Boolean => "boolean",
                ShapeType::Document => "document",
                ShapeType::String => "string",
                ShapeType::Integer => "integer",
                ShapeType::Byte => "byte",
                ShapeType::Short => "short",
                ShapeType::Long => "long",
                ShapeType::Float => "float",
                ShapeType::Double => "double",
                ShapeType::BigDecimal => "bigDecimal",
                ShapeType::BigInteger => "bigInteger",
                ShapeType::Timestamp => "timestamp",
                ShapeType::List => "list",
                ShapeType::Set => "set",
                ShapeType::Map => "map",
                ShapeType::Structure => "structure",
                ShapeType::Union => "union",
                ShapeType::Service => "service",
                ShapeType::Operation => "operation",
                ShapeType::Resource => "resource",
                ShapeType::Member => "member",
            }
        )
    }
}

impl FromStr for ShapeType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(ShapeType::All),
            "number" => Ok(ShapeType::Number),
            "simpleType" => Ok(ShapeType::SimpleType),
            "collection" => Ok(ShapeType::Collection),
            "blob" => Ok(ShapeType::Blob),
            "boolean" => Ok(ShapeType::Boolean),
            "document" => Ok(ShapeType::Document),
            "string" => Ok(ShapeType::String),
            "integer" => Ok(ShapeType::Integer),
            "byte" => Ok(ShapeType::Byte),
            "short" => Ok(ShapeType::Short),
            "long" => Ok(ShapeType::Long),
            "float" => Ok(ShapeType::Float),
            "double" => Ok(ShapeType::Double),
            "bigDecimal" => Ok(ShapeType::BigDecimal),
            "bigInteger" => Ok(ShapeType::BigInteger),
            "timestamp" => Ok(ShapeType::Timestamp),
            "list" => Ok(ShapeType::List),
            "set" => Ok(ShapeType::Set),
            "map" => Ok(ShapeType::Map),
            "structure" => Ok(ShapeType::Structure),
            "union" => Ok(ShapeType::Union),
            "service" => Ok(ShapeType::Service),
            "operation" => Ok(ShapeType::Operation),
            "resource" => Ok(ShapeType::Resource),
            "member" => Ok(ShapeType::Member),
            _ => Err(InvalidSelector.into()),
        }
    }
}

impl From<ShapeType> for SelectorExpression {
    fn from(v: ShapeType) -> Self {
        SelectorExpression::ShapeType(v)
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Value::Text(v) => format!("{:?}", v),
                Value::Number(v) => v.to_string(),
                Value::RootShapeIdentifier(v) => v.to_string(),
                Value::AbsoluteRootShapeIdentifier(v) => v.to_string(),
            }
        )
    }
}

impl From<String> for Value {
    fn from(v: String) -> Self {
        Value::Text(v)
    }
}

impl From<&str> for Value {
    fn from(v: &str) -> Self {
        Value::Text(v.to_string())
    }
}

impl From<Number> for Value {
    fn from(v: Number) -> Self {
        Value::Number(v)
    }
}

impl From<i64> for Value {
    fn from(v: i64) -> Self {
        Value::Number(v.into())
    }
}

impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Value::Number(v.into())
    }
}

impl From<Identifier> for Value {
    fn from(v: Identifier) -> Self {
        Value::RootShapeIdentifier(v)
    }
}

impl From<ShapeID> for Value {
    fn from(v: ShapeID) -> Self {
        Value::AbsoluteRootShapeIdentifier(v)
    }
}

impl Value {
    is_as! { text, Text, String }

    is_as! { number, Number, Number }

    is_as! { root_shape_id, RootShapeIdentifier, Identifier }

    is_as! {
        absolute_root_shape_id, AbsoluteRootShapeIdentifier, ShapeID
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for KeyPathSegment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                KeyPathSegment::Value(v) => v.to_string(),
                KeyPathSegment::FunctionProperty(v) => format!("({})", v),
            }
        )
    }
}

impl From<Value> for KeyPathSegment {
    fn from(v: Value) -> Self {
        Self::Value(v)
    }
}

impl From<Identifier> for KeyPathSegment {
    fn from(i: Identifier) -> Self {
        Self::FunctionProperty(i)
    }
}

impl KeyPathSegment {
    is_as! { value, Value, Value }

    is_as! { function_property, FunctionProperty, Identifier }
}

// ------------------------------------------------------------------------------------------------

impl Display for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.identifier,
            if self.path.is_empty() {
                String::new()
            } else {
                format!(
                    "|{}",
                    self.path()
                        .map(KeyPathSegment::to_string)
                        .collect::<Vec<String>>()
                        .join("|")
                )
            },
        )
    }
}

impl From<Identifier> for Key {
    fn from(v: Identifier) -> Self {
        Self {
            identifier: v,
            path: Default::default(),
        }
    }
}

impl Key {
    /// Create a new key with only an identifier, no path.
    pub fn new(identifier: Identifier) -> Self {
        Self {
            identifier,
            path: Default::default(),
        }
    }

    /// Create a new key with both an identifier and path.
    pub fn with_path(identifier: Identifier, path: &[KeyPathSegment]) -> Self {
        Self {
            identifier,
            path: path.to_vec(),
        }
    }

    required_member! { identifier, Identifier }

    array_member! { path, path_segment, KeyPathSegment }
}

// ------------------------------------------------------------------------------------------------

impl Display for Comparator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Comparator::StringEqual => "=",
                Comparator::StringNotEqual => "!=",
                Comparator::StringStartsWith => "^=",
                Comparator::StringEndsWith => "$=",
                Comparator::StringContains => "*=",
                Comparator::StringExists => "?=",
                Comparator::NumberGreaterThan => ">",
                Comparator::NumberGreaterOrEqual => ">=",
                Comparator::NumberLessThan => "<",
                Comparator::NumberLessOrEqual => "<=",
                Comparator::ProjectionEqual => "{=}",
                Comparator::ProjectionNotEqual => "{!=}",
                Comparator::ProjectionSubset => "{<}",
                Comparator::ProjectionProperSubset => "{<<}",
            }
        )
    }
}

impl FromStr for Comparator {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "=" => Ok(Comparator::StringEqual),
            "!=" => Ok(Comparator::StringNotEqual),
            "^=" => Ok(Comparator::StringStartsWith),
            "$=" => Ok(Comparator::StringEndsWith),
            "*=" => Ok(Comparator::StringContains),
            "?=" => Ok(Comparator::StringExists),
            ">" => Ok(Comparator::NumberGreaterThan),
            ">=" => Ok(Comparator::NumberGreaterOrEqual),
            "<" => Ok(Comparator::NumberLessThan),
            "<=" => Ok(Comparator::NumberLessOrEqual),
            "{=}" => Ok(Comparator::ProjectionEqual),
            "{!=}" => Ok(Comparator::ProjectionNotEqual),
            "{<}" => Ok(Comparator::ProjectionSubset),
            "{<<}" => Ok(Comparator::ProjectionProperSubset),
            _ => Err(InvalidSelector.into()),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for AttributeComparison {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            " {} {}{}",
            self.comparator,
            self.rhs_values()
                .map(Value::to_string)
                .collect::<Vec<String>>()
                .join(", "),
            if self.case_insensitive {
                " i".to_string()
            } else {
                String::new()
            }
        )
    }
}

impl AttributeComparison {
    /// Construct a new comparison with the provided comparator and right-hand side values.
    pub fn new(comparator: Comparator, values: &[Value]) -> Self {
        Self {
            comparator,
            rhs_values: values.to_vec(),
            case_insensitive: false,
        }
    }

    /// Construct a new case-insensitive comparison with the provided comparator and right-hand side values.
    pub fn new_case_insensitive(comparator: Comparator, values: &[Value]) -> Self {
        Self {
            comparator,
            rhs_values: values.to_vec(),
            case_insensitive: true,
        }
    }

    /// Create a new comparison between the `AttributeSelector` and `rhs` value with the `=` operator.
    pub fn string_equal(rhs: Value) -> Self {
        Self::new(Comparator::StringEqual, &[rhs])
    }

    /// Create a new comparison between the `AttributeSelector` and `rhs` value with the `!=` operator.
    pub fn string_not_equal(rhs: Value) -> Self {
        Self::new(Comparator::StringNotEqual, &[rhs])
    }

    /// Create a new comparison between the `AttributeSelector` and `rhs` value with the `^=` operator.
    pub fn string_starts_with(rhs: Value) -> Self {
        Self::new(
            Comparator::StringStartsWith,
            &[Value::Text(rhs.to_string())],
        )
    }

    /// Create a new comparison between the `AttributeSelector` and `rhs` value with the `$=` operator.
    pub fn string_ends_with(rhs: Value) -> Self {
        Self::new(Comparator::StringEndsWith, &[rhs])
    }

    /// Create a new comparison between the `AttributeSelector` and `rhs` value with the `*=` operator.
    pub fn string_contains(rhs: Value) -> Self {
        Self::new(Comparator::StringContains, &[rhs])
    }

    /// Create a new comparison between the `AttributeSelector` and `rhs` value with the `?=` operator.
    pub fn string_exists(rhs: bool) -> Self {
        Self::new(Comparator::StringExists, &[Value::Text(rhs.to_string())])
    }

    /// Create a new comparison between the `AttributeSelector` and `rhs` value with the `>` operator.
    pub fn number_greater(rhs: Number) -> Self {
        Self::new(
            Comparator::NumberGreaterThan,
            &[Value::Text(rhs.to_string())],
        )
    }

    /// Create a new comparison between the `AttributeSelector` and `rhs` value with the `>=` operator.
    pub fn number_greater_or_equal(rhs: Number) -> Self {
        Self::new(
            Comparator::NumberGreaterOrEqual,
            &[Value::Text(rhs.to_string())],
        )
    }

    /// Create a new comparison between the `AttributeSelector` and `rhs` value with the `<` operator.
    pub fn number_less(rhs: Number) -> Self {
        Self::new(Comparator::NumberLessThan, &[Value::Text(rhs.to_string())])
    }

    /// Create a new comparison between the `AttributeSelector` and `rhs` value with the `<=` operator.
    pub fn number_less_or_equal(rhs: Number) -> Self {
        Self::new(
            Comparator::NumberLessOrEqual,
            &[Value::Text(rhs.to_string())],
        )
    }

    // --------------------------------------------------------------------------------------------

    required_member! { comparator, Comparator }

    array_member! { rhs_values, value, Value }

    boolean_member! { case_insensitive }
}

// ------------------------------------------------------------------------------------------------

impl Display for AttributeSelector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}{}]",
            self.key,
            match &self.comparison {
                None => String::new(),
                Some(v) => v.to_string(),
            }
        )
    }
}

impl From<Key> for AttributeSelector {
    fn from(key: Key) -> Self {
        Self {
            key,
            comparison: None,
        }
    }
}

impl From<AttributeSelector> for SelectorExpression {
    fn from(v: AttributeSelector) -> Self {
        SelectorExpression::AttributeSelector(v)
    }
}

impl AttributeSelector {
    /// Create a new selector with only a key.
    pub fn new(key: Key) -> Self {
        Self {
            key,
            comparison: None,
        }
    }

    /// Create a new selector with both a key and comparison.
    pub fn with_comparison(key: Key, comparison: AttributeComparison) -> Self {
        Self {
            key,
            comparison: Some(comparison),
        }
    }

    required_member! { key, Key }

    optional_member! {
        comparison, AttributeComparison
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for SelectorExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SelectorExpression::ShapeType(v) => v.to_string(),
                SelectorExpression::AttributeSelector(v) => v.to_string(),
                SelectorExpression::ScopedAttributeSelector(v) => v.to_string(),
                SelectorExpression::NeighborSelector(v) => v.to_string(),
                SelectorExpression::Function(v) => v.to_string(),
                SelectorExpression::VariableDefinition(v) => v.to_string(),
                SelectorExpression::VariableReference(v) => v.to_string(),
            }
        )
    }
}

impl SelectorExpression {
    is_as! { shape_type, ShapeType, ShapeType }

    is_as! { attribute_selector, AttributeSelector, AttributeSelector }

    is_as! {
        scoped_attribute_selector, ScopedAttributeSelector, ScopedAttributeSelector
    }

    is_as! { neighbor_selector, NeighborSelector, NeighborSelector }

    is_as! { function, Function, Function }

    is_as! {
        variable_definition, VariableDefinition, VariableDefinition
    }

    is_as! { variable_reference, VariableReference, VariableReference }
}

// ------------------------------------------------------------------------------------------------

impl Default for Selector {
    fn default() -> Self {
        Self {
            expressions: Default::default(),
        }
    }
}

impl Display for Selector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.expressions()
                .map(SelectorExpression::to_string)
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl From<SelectorExpression> for Selector {
    fn from(v: SelectorExpression) -> Self {
        Self {
            expressions: vec![v],
        }
    }
}

impl From<Vec<SelectorExpression>> for Selector {
    fn from(v: Vec<SelectorExpression>) -> Self {
        Self { expressions: v }
    }
}

impl Selector {
    array_member! {
        expressions, expression, SelectorExpression
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for ScopedAttributeSelector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[@{}: {}]",
            match &self.key {
                None => String::new(),
                Some(key) => key.to_string(),
            },
            self.assertions
                .iter()
                .map(ScopedAttributeAssertion::to_string)
                .collect::<Vec<String>>()
                .join(" && ")
        )
    }
}

impl From<ScopedAttributeSelector> for SelectorExpression {
    fn from(v: ScopedAttributeSelector) -> Self {
        SelectorExpression::ScopedAttributeSelector(v)
    }
}

impl ScopedAttributeSelector {
    /// Create a new selector with the provided assertions, without a key.
    pub fn new(assertions: &[ScopedAttributeAssertion]) -> Self {
        assert!(!assertions.is_empty());
        Self {
            key: None,
            assertions: assertions.to_vec(),
        }
    }

    /// Create a new selector with the provided key and assertions.
    pub fn with_key(key: Key, assertions: &[ScopedAttributeAssertion]) -> Self {
        assert!(!assertions.is_empty());
        Self {
            key: Some(key),
            assertions: assertions.to_vec(),
        }
    }

    optional_member! { key, Key }

    array_member! {
        assertions, assertion, ScopedAttributeAssertion

    }
}

// ------------------------------------------------------------------------------------------------

impl Display for ScopedValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ScopedValue::Value(v) => v.to_string(),
                ScopedValue::ContextValue(v) => format!(
                    "@{{{}}}",
                    v.iter()
                        .map(KeyPathSegment::to_string)
                        .collect::<Vec<String>>()
                        .join("|")
                ),
            }
        )
    }
}

impl From<String> for ScopedValue {
    fn from(v: String) -> Self {
        Value::Text(v).into()
    }
}

impl From<&str> for ScopedValue {
    fn from(v: &str) -> Self {
        Value::Text(v.to_string()).into()
    }
}

impl From<Number> for ScopedValue {
    fn from(v: Number) -> Self {
        Value::Number(v).into()
    }
}

impl From<i64> for ScopedValue {
    fn from(v: i64) -> Self {
        Value::Number(v.into()).into()
    }
}

impl From<f64> for ScopedValue {
    fn from(v: f64) -> Self {
        Value::Number(v.into()).into()
    }
}

impl From<Identifier> for ScopedValue {
    fn from(v: Identifier) -> Self {
        Value::RootShapeIdentifier(v).into()
    }
}

impl From<ShapeID> for ScopedValue {
    fn from(v: ShapeID) -> Self {
        Value::AbsoluteRootShapeIdentifier(v).into()
    }
}

impl From<Value> for ScopedValue {
    fn from(v: Value) -> Self {
        ScopedValue::Value(v)
    }
}

impl From<Vec<KeyPathSegment>> for ScopedValue {
    fn from(v: Vec<KeyPathSegment>) -> Self {
        ScopedValue::ContextValue(v)
    }
}

impl From<&[KeyPathSegment]> for ScopedValue {
    fn from(v: &[KeyPathSegment]) -> Self {
        ScopedValue::ContextValue(v.to_vec())
    }
}

impl ScopedValue {
    is_as! { value, Value, Value }

    is_as_array! { context_value, ContextValue, KeyPathSegment }
}

// ------------------------------------------------------------------------------------------------

impl Display for ScopedAttributeAssertion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}{}",
            self.lhs_value,
            self.comparator,
            self.rhs_values()
                .map(ScopedValue::to_string)
                .collect::<Vec<String>>()
                .join(", "),
            if self.case_insensitive {
                " i".to_string()
            } else {
                String::new()
            }
        )
    }
}
impl ScopedAttributeAssertion {
    /// Create a new assertion between `lhs_value` and `rhs_values` using the `comparator` operation.
    pub fn new(lhs_value: ScopedValue, comparator: Comparator, rhs_values: &[ScopedValue]) -> Self {
        Self {
            lhs_value,
            comparator,
            rhs_values: rhs_values.to_vec(),
            case_insensitive: false,
        }
    }

    /// Create a new case insensitive assertion between `lhs_value` and `rhs_values` using the
    /// `comparator` operation.
    pub fn new_case_insensitive(
        lhs_value: ScopedValue,
        comparator: Comparator,
        rhs_values: &[ScopedValue],
    ) -> Self {
        Self {
            lhs_value,
            comparator,
            rhs_values: rhs_values.to_vec(),
            case_insensitive: true,
        }
    }

    /// Create a new assertion between `lhs` and `rhs` with the `=` operator.
    pub fn string_equal(lhs: ScopedValue, rhs: ScopedValue) -> Self {
        Self::new(lhs, Comparator::StringEqual, &[rhs])
    }

    /// Create a new assertion between `lhs` and `rhs` with the `!=` operator.
    pub fn string_not_equal(lhs: ScopedValue, rhs: ScopedValue) -> Self {
        Self::new(lhs, Comparator::StringNotEqual, &[rhs])
    }

    /// Create a new assertion between `lhs` and `rhs` with the `^=` operator.
    pub fn string_starts_with(lhs: ScopedValue, rhs: ScopedValue) -> Self {
        Self::new(
            lhs,
            Comparator::StringStartsWith,
            &[Value::Text(rhs.to_string()).into()],
        )
    }

    /// Create a new assertion between `lhs` and `rhs` with the `$=` operator.
    pub fn string_ends_with(lhs: ScopedValue, rhs: ScopedValue) -> Self {
        Self::new(lhs, Comparator::StringEndsWith, &[rhs])
    }

    /// Create a new assertion between `lhs` and `rhs` with the `*=` operator.
    pub fn string_contains(lhs: ScopedValue, rhs: ScopedValue) -> Self {
        Self::new(lhs, Comparator::StringContains, &[rhs])
    }

    /// Create a new assertion between `lhs` and `rhs` with the `?=` operator.
    pub fn string_exists(lhs: ScopedValue, rhs: bool) -> Self {
        Self::new(
            lhs,
            Comparator::StringExists,
            &[Value::Text(rhs.to_string()).into()],
        )
    }

    /// Create a new assertion between `lhs` and `rhs` with the `>` operator.
    pub fn number_greater(lhs: ScopedValue, rhs: Number) -> Self {
        Self::new(
            lhs,
            Comparator::NumberGreaterThan,
            &[Value::Text(rhs.to_string()).into()],
        )
    }

    /// Create a new assertion between `lhs` and `rhs` with the `>=` operator.
    pub fn number_greater_or_equal(lhs: ScopedValue, rhs: Number) -> Self {
        Self::new(
            lhs,
            Comparator::NumberGreaterOrEqual,
            &[Value::Text(rhs.to_string()).into()],
        )
    }

    /// Create a new assertion between `lhs` and `rhs` with the `<` operator.
    pub fn number_less(lhs: ScopedValue, rhs: Number) -> Self {
        Self::new(
            lhs,
            Comparator::NumberLessThan,
            &[Value::Text(rhs.to_string()).into()],
        )
    }

    /// Create a new assertion between `lhs` and `rhs` with the `<=` operator.
    pub fn number_less_or_equal(lhs: ScopedValue, rhs: Number) -> Self {
        Self::new(
            lhs,
            Comparator::NumberLessOrEqual,
            &[Value::Text(rhs.to_string()).into()],
        )
    }

    required_member! { lhs_value, ScopedValue }

    required_member! { comparator, Comparator }

    array_member! { rhs_values, value, ScopedValue }

    boolean_member! { case_insensitive }
}

// ------------------------------------------------------------------------------------------------

impl Display for NeighborSelector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                NeighborSelector::ForwardUndirected => ">".to_string(),
                NeighborSelector::ReverseUndirected => "<".to_string(),
                NeighborSelector::ForwardDirected(vs) => format!(
                    "-[{}]->",
                    vs.iter()
                        .map(Identifier::to_string)
                        .collect::<Vec<String>>()
                        .join(", ")
                ),
                NeighborSelector::ReverseDirected(vs) => format!(
                    "<-[{}]-",
                    vs.iter()
                        .map(Identifier::to_string)
                        .collect::<Vec<String>>()
                        .join(", ")
                ),
                NeighborSelector::ForwardRecursiveDirected => "~>".to_string(),
            }
        )
    }
}

impl From<NeighborSelector> for SelectorExpression {
    fn from(v: NeighborSelector) -> Self {
        SelectorExpression::NeighborSelector(v)
    }
}

impl NeighborSelector {
    is_only! { forward_undirected, ForwardUndirected }

    is_only! { reverse_undirected, ReverseUndirected }

    is_only! { forward_recursive_directed, ForwardRecursiveDirected }

    is_as_array! { forward_directed, ForwardDirected, Identifier  }

    is_as_array! { reverse_directed, ReverseDirected, Identifier }
}

// ------------------------------------------------------------------------------------------------

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            ":{}({})",
            self.name,
            self.arguments
                .iter()
                .map(SelectorExpression::to_string)
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl From<Function> for SelectorExpression {
    fn from(v: Function) -> Self {
        SelectorExpression::Function(v)
    }
}

impl Function {
    /// Construct a new function named `name` and corresponding `arguments` expressions.
    pub fn new(name: Identifier, arguments: &[SelectorExpression]) -> Self {
        assert!(!arguments.is_empty());
        Self {
            name,
            arguments: arguments.to_vec(),
        }
    }

    required_member! { name, Identifier }

    array_member! {
        arguments, argument, SelectorExpression

    }
}

// ------------------------------------------------------------------------------------------------

impl Display for VariableDefinition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "${}({})",
            self.name,
            self.expressions
                .iter()
                .map(SelectorExpression::to_string)
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl From<VariableDefinition> for SelectorExpression {
    fn from(v: VariableDefinition) -> Self {
        SelectorExpression::VariableDefinition(v)
    }
}

impl VariableDefinition {
    /// Construct a new variable named `name` and defined by `expressions`.
    pub fn new(name: Identifier, expressions: &[SelectorExpression]) -> Self {
        assert!(!expressions.is_empty());
        Self {
            name,
            expressions: expressions.to_vec(),
        }
    }

    required_member! { name, Identifier }

    array_member! {
        expressions, expression, SelectorExpression
    }

    /// Create a new reference to this definition.
    pub fn new_reference(&self) -> VariableReference {
        VariableReference::new(self.name.clone())
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for VariableReference {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "${{{}}}", self.name)
    }
}

impl From<VariableReference> for SelectorExpression {
    fn from(v: VariableReference) -> Self {
        SelectorExpression::VariableReference(v)
    }
}

impl From<Identifier> for VariableReference {
    fn from(v: Identifier) -> Self {
        VariableReference::new(v)
    }
}

impl VariableReference {
    /// Create a new reference to the variable named `name`.
    pub fn new(name: Identifier) -> Self {
        Self { name }
    }

    required_member! { name, Identifier }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

//pub mod evaluate;
