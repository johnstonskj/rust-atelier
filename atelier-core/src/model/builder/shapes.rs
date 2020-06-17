/*!
One-line description.

More detailed description, with

# Example

*/

use crate::model::shapes::{Shape, ShapeInner, SimpleType, Trait};
use crate::model::{Annotated, Documented, Identifier};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub struct ShapeBuilder {
    shape: Shape,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn blob(id: &str) -> ShapeBuilder {
    ShapeBuilder::new(
        Identifier::from_str(id).unwrap(),
        ShapeInner::SimpleType(SimpleType::Blob),
    )
}

pub fn boolean(id: &str) -> ShapeBuilder {
    ShapeBuilder::new(
        Identifier::from_str(id).unwrap(),
        ShapeInner::SimpleType(SimpleType::Boolean),
    )
}

pub fn document(id: &str) -> ShapeBuilder {
    ShapeBuilder::new(
        Identifier::from_str(id).unwrap(),
        ShapeInner::SimpleType(SimpleType::Document),
    )
}

pub fn string(id: &str) -> ShapeBuilder {
    ShapeBuilder::new(
        Identifier::from_str(id).unwrap(),
        ShapeInner::SimpleType(SimpleType::String),
    )
}

pub fn byte(id: &str) -> ShapeBuilder {
    ShapeBuilder::new(
        Identifier::from_str(id).unwrap(),
        ShapeInner::SimpleType(SimpleType::Byte),
    )
}

pub fn short(id: &str) -> ShapeBuilder {
    ShapeBuilder::new(
        Identifier::from_str(id).unwrap(),
        ShapeInner::SimpleType(SimpleType::Short),
    )
}

pub fn integer(id: &str) -> ShapeBuilder {
    ShapeBuilder::new(
        Identifier::from_str(id).unwrap(),
        ShapeInner::SimpleType(SimpleType::Integer),
    )
}

pub fn long(id: &str) -> ShapeBuilder {
    ShapeBuilder::new(
        Identifier::from_str(id).unwrap(),
        ShapeInner::SimpleType(SimpleType::Long),
    )
}

pub fn float(id: &str) -> ShapeBuilder {
    ShapeBuilder::new(
        Identifier::from_str(id).unwrap(),
        ShapeInner::SimpleType(SimpleType::Float),
    )
}

pub fn double(id: &str) -> ShapeBuilder {
    ShapeBuilder::new(
        Identifier::from_str(id).unwrap(),
        ShapeInner::SimpleType(SimpleType::Double),
    )
}

pub fn big_integer(id: &str) -> ShapeBuilder {
    ShapeBuilder::new(
        Identifier::from_str(id).unwrap(),
        ShapeInner::SimpleType(SimpleType::BigInteger),
    )
}

pub fn big_decimal(id: &str) -> ShapeBuilder {
    ShapeBuilder::new(
        Identifier::from_str(id).unwrap(),
        ShapeInner::SimpleType(SimpleType::BigDecimal),
    )
}

pub fn timestamp(id: &str) -> ShapeBuilder {
    ShapeBuilder::new(
        Identifier::from_str(id).unwrap(),
        ShapeInner::SimpleType(SimpleType::Timestamp),
    )
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl ShapeBuilder {
    fn new(id: Identifier, shape_type: ShapeInner) -> Self {
        Self {
            shape: Shape::new(id, shape_type),
        }
    }

    pub fn documentation(&mut self, documentation: &str) -> &mut Self {
        self.shape.set_documentation(documentation);
        self
    }

    pub fn add_trait(&mut self, a_trait: Trait) -> &mut Self {
        self.shape.add_trait(a_trait);
        self
    }

    pub fn build(&self) -> Shape {
        self.shape.clone()
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
