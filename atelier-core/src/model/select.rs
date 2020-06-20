/*!
Model structures for selector expressions.

*/

use crate::model::values::NodeValue;
use crate::model::ShapeID;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub enum SelectorToken {
    All,
    Number,
    SimpleType,
    Collection,
    Blob,
    Boolean,
    Document,
    String,
    Byte,
    Short,
    Integer,
    Long,
    Float,
    Double,
    BigInteger,
    BigDecimal,
    Timestamp,
    List,
    Set,
    Map,
    Structure,
    Union,
    Service,
    Operation,
    Resource,
    Member,
}

#[derive(Debug)]
pub enum IdAttributes {
    Namespace,
    Name,
    Member,
    Length,
}

#[derive(Debug)]
pub enum ServiceAttributes {
    Id,
    Version,
}

#[derive(Debug)]
pub enum NodeAttributes {
    Keys,
    Values,
    Length,
    Id(ShapeID),
}

#[derive(Debug)]
pub enum Attribute {
    Id(IdAttributes),
    Trait(NodeAttributes),
    Node(NodeAttributes),
    Service(ServiceAttributes),
    Var,
}

#[derive(Debug)]
pub enum Operation {
    // Strings... (i = insensitive)
    Equal,
    NotEqual,
    StartsWith,
    EndsWith,
    Contains,
    Exists,
    // Numeric
    GreaterThan,
    GreaterOrEqual,
    LessThan,
    LessOrEqual,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct AttributeMatch {
    attribute: Attribute,
    operation: Operation,
    rhs: Vec<NodeValue>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct QueryResult {
    values: Option<Vec<NodeValue>>,
}

pub trait Queryable {
    fn query(&self) -> QueryResult;
}

pub trait Matchable {
    fn matches(&self) -> bool;
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

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
