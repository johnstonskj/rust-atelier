/*!
One-line description.

More detailed description, with

# Example

*/

use crate::model::shapes::Shape;
use crate::model::{Model, Namespace, ShapeID};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub struct ModelBuilder {
    model: Model,
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

impl ModelBuilder {
    pub fn new(namespace: &str) -> Self {
        Self {
            model: Model {
                version: Default::default(),
                namespace: Namespace::from_str(namespace).unwrap(),
                uses: vec![],
                shapes: Default::default(),
                applies: vec![],
                metadata: vec![],
            },
        }
    }

    pub fn uses(&mut self, shape: &str) -> &mut Self {
        self.model.add_usage(ShapeID::from_str(shape).unwrap());
        self
    }

    pub fn add(&mut self, shape: Shape) -> &mut Self {
        self.model.add_shape(shape);
        self
    }

    pub fn build(&self) -> Model {
        self.model.clone()
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod shapes;

pub mod traits;

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder() {
        let model = ModelBuilder::new("example.weather");
        let model: Model = model.build();
        println!("{:?}", model);
    }
}
