/*!
One-line description.

More detailed description, with

# Example

*/

use crate::model::values::Value;
use crate::model::ShapeID;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

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

pub enum IdAttributes {
    Namespace,
    Name,
    Member,
    Length,
}

pub enum ServiceAttributes {
    Id,
    Version,
}

pub enum NodeAttributes {
    Keys,
    Values,
    Length,
    Id(ShapeID),
}

pub enum Attribute {
    Id(IdAttributes),
    Trait(NodeAttributes),
    Node(NodeAttributes),
    Service(ServiceAttributes),
    Var,
}

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
pub struct AttributeMatch {
    attribute: Attribute,
    operation: Operation,
    rhs: Vec<Value>,
}

#[allow(dead_code)]
pub struct QueryResult {
    values: Option<Vec<Value>>,
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
