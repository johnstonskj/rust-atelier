/*!
One-line description.

More detailed description, with

# Example

*/

use crate::model::selector::{
    Function, Selector, SelectorExpression, ShapeType, VariableDefinition, VariableReference,
};
use crate::model::Identifier;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// Builder for `Selector` expressions.
#[derive(Clone, Debug)]
pub struct SelectorBuilder {
    selector: Selector,
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

impl Default for SelectorBuilder {
    fn default() -> Self {
        SelectorBuilder {
            selector: Default::default(),
        }
    }
}

impl From<SelectorBuilder> for Selector {
    fn from(v: SelectorBuilder) -> Self {
        v.selector
    }
}

impl SelectorBuilder {
    pub fn add_shape(&mut self, v: ShapeType) -> &mut Self {
        self.selector.add_expression(v.into());
        self
    }

    pub fn add_function(&mut self, v: Function) -> &mut Self {
        self.selector.add_expression(v.into());
        self
    }

    pub fn add_function_from(
        &mut self,
        name: Identifier,
        expressions: &[SelectorExpression],
    ) -> &mut Self {
        self.selector
            .add_expression(Function::new(name, expressions).into());
        self
    }

    pub fn add_variable_reference(&mut self, v: VariableReference) -> &mut Self {
        self.selector.add_expression(v.into());
        self
    }

    pub fn add_variable_reference_from(&mut self, name: Identifier) -> &mut Self {
        self.selector
            .add_expression(VariableReference::new(name).into());
        self
    }

    pub fn add_variable_definition(&mut self, v: VariableDefinition) -> &mut Self {
        self.selector.add_expression(v.into());
        self
    }

    pub fn add_variable_definition_from(
        &mut self,
        name: Identifier,
        expressions: &[SelectorExpression],
    ) -> &mut Self {
        self.selector
            .add_expression(VariableDefinition::new(name, expressions).into());
        self
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
