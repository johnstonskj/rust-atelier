/*!
Model structures for values.
*/

use crate::model::{Identifier, ShapeID};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Blob(Vec<u8>),
    Boolean(bool),
    Document(String),
    String(String),
    Byte(i8),
    Short(i16),
    Integer(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    BigInteger(i128), // TODO: Need a better representation
    BigDecimal(i128), // TODO: Need a better representation
    Timestamp(i128),  // TODO: Need a better representation
    // ----------------------------------------------------
    Identifier(Identifier),
    Ref(ShapeID),
    // ----------------------------------------------------
    List(Vec<Value>),
    Set(Vec<Value>),
    Map(HashMap<Identifier, Value>),
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

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Boolean(v) => write!(f, "{}", v),
            Value::String(v) => write!(f, "\"{}\"", v),
            Value::Byte(v) => write!(f, "{}", v),
            Value::Short(v) => write!(f, "{}", v),
            Value::Integer(v) => write!(f, "{}", v),
            Value::Long(v) => write!(f, "{}", v),
            Value::Float(v) => write!(f, "{}", v),
            Value::Double(v) => write!(f, "{}", v),
            Value::BigInteger(v) => write!(f, "\"{}\"", v),
            Value::BigDecimal(v) => write!(f, "{}", v),
            Value::Timestamp(v) => write!(f, "{}", v),
            Value::Identifier(v) => write!(f, "{}", v),
            Value::List(vs) | Value::Set(vs) => writeln!(
                f,
                "[ {} ]",
                vs.iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Value::Map(vs) => writeln!(
                f,
                "{{ {} }}",
                vs.iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            _ => panic!("Cannot format a value of this type"),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
