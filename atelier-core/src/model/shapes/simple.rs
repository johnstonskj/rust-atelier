use crate::error::{Error, ErrorKind};
use crate::syntax::{
    SHAPE_BIG_DECIMAL, SHAPE_BIG_INTEGER, SHAPE_BLOB, SHAPE_BOOLEAN, SHAPE_BYTE, SHAPE_DOCUMENT,
    SHAPE_DOUBLE, SHAPE_FLOAT, SHAPE_INTEGER, SHAPE_LONG, SHAPE_SHORT, SHAPE_STRING,
    SHAPE_TIMESTAMP,
};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Corresponds to the simple shape within Smithy, these are atomic values.
///
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Simple {
    /// Corresponds to the `simple_type_name` production's "blob" terminal.
    Blob,
    /// Corresponds to the `simple_type_name` production's "boolean" terminal.
    Boolean,
    /// Corresponds to the `simple_type_name` production's "document" terminal.
    Document,
    /// Corresponds to the `simple_type_name` production's "string" terminal.
    String,
    /// Corresponds to the `simple_type_name` production's "byte" terminal.
    Byte,
    /// Corresponds to the `simple_type_name` production's "short" terminal.
    Short,
    /// Corresponds to the `simple_type_name` production's "integer" terminal.
    Integer,
    /// Corresponds to the `simple_type_name` production's "long" terminal.
    Long,
    /// Corresponds to the `simple_type_name` production's "float" terminal.
    Float,
    /// Corresponds to the `simple_type_name` production's "double" terminal.
    Double,
    /// Corresponds to the `simple_type_name` production's "bigInteger" terminal.
    BigInteger,
    /// Corresponds to the `simple_type_name` production's "bigDecimal" terminal.
    BigDecimal,
    /// Corresponds to the `simple_type_name` production's "timestamp" terminal.
    Timestamp,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for Simple {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Simple::Blob => SHAPE_BLOB,
                Simple::Boolean => SHAPE_BOOLEAN,
                Simple::Document => SHAPE_DOCUMENT,
                Simple::String => SHAPE_STRING,
                Simple::Byte => SHAPE_BYTE,
                Simple::Short => SHAPE_SHORT,
                Simple::Integer => SHAPE_INTEGER,
                Simple::Long => SHAPE_LONG,
                Simple::Float => SHAPE_FLOAT,
                Simple::Double => SHAPE_DOUBLE,
                Simple::BigInteger => SHAPE_BIG_INTEGER,
                Simple::BigDecimal => SHAPE_BIG_DECIMAL,
                Simple::Timestamp => SHAPE_TIMESTAMP,
            }
        )
    }
}

impl FromStr for Simple {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            SHAPE_BLOB => Ok(Simple::Blob),
            SHAPE_BOOLEAN => Ok(Simple::Boolean),
            SHAPE_DOCUMENT => Ok(Simple::Document),
            SHAPE_STRING => Ok(Simple::String),
            SHAPE_BYTE => Ok(Simple::Byte),
            SHAPE_SHORT => Ok(Simple::Short),
            SHAPE_INTEGER => Ok(Simple::Integer),
            SHAPE_LONG => Ok(Simple::Long),
            SHAPE_FLOAT => Ok(Simple::Float),
            SHAPE_DOUBLE => Ok(Simple::Double),
            SHAPE_BIG_INTEGER => Ok(Simple::BigInteger),
            SHAPE_BIG_DECIMAL => Ok(Simple::BigDecimal),
            SHAPE_TIMESTAMP => Ok(Simple::Timestamp),
            _ => Err(ErrorKind::UnknownType(s.to_string()).into()),
        }
    }
}
