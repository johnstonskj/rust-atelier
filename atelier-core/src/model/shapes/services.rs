/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::invalid_value_variant;
use crate::model::shapes::{Member, Valued};
use crate::model::values::Value;
use crate::model::{Identifier, ShapeID};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Service {
    version: Member,    // **required** Value::String
    operations: Member, // Value::Set(Value::Ref)
    resources: Member,  // Value::Set(Value::Ref)
}

#[derive(Clone, Debug)]
pub struct Operation {
    input: Member,  // Value::Ref
    output: Member, // Value::Ref
    errors: Member, // Value::Set(Value::Ref)
}

#[derive(Clone, Debug)]
pub struct Resource {
    identifiers: Member,           // Value::Map(Identifier, Value::Ref)
    create: Member,                // Value::Ref
    put: Member,                   // Value::Ref
    read: Member,                  // Value::Ref
    update: Member,                // Value::Ref
    delete: Member,                // Value::Ref
    list: Member,                  // Value::Ref
    operations: Member,            // Value::List(Value::Ref)
    collection_operations: Member, // Value::List(Value::Ref)
    resources: Member,             // Value::List(Value::Ref)
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

impl Default for Service {
    fn default() -> Self {
        Self {
            version: Member::new(Identifier::from_str("version").unwrap()),
            operations: Member::with_value(
                Identifier::from_str("operations").unwrap(),
                Value::Set(Default::default()),
            ),
            resources: Member::with_value(
                Identifier::from_str("resources").unwrap(),
                Value::Set(Default::default()),
            ),
        }
    }
}

impl Service {
    pub fn version(&self) -> &String {
        match &self.version.value().as_ref().unwrap() {
            Value::String(v) => v,
            _ => invalid_value_variant("String"),
        }
    }
    pub fn set_version(&mut self, version: &str) {
        self.version.set_value(Value::String(version.to_string()))
    }

    pub fn operations(&self) -> impl Iterator<Item = &ShapeID> {
        match self.operations.value() {
            Some(v) => match v {
                Value::Set(vs) => vs.iter().map(|v| {
                    if let Value::Ref(id) = v {
                        id
                    } else {
                        invalid_value_variant("Ref")
                    }
                }),
                _ => invalid_value_variant("Set"),
            },
            _ => invalid_value_variant("Set"),
        }
    }
    pub fn add_operation(&mut self, operation: ShapeID) {
        match self.operations.value_mut() {
            Some(v) => match v {
                Value::Set(vs) => vs.push(Value::Ref(operation)),
                _ => invalid_value_variant("Set"),
            },
            _ => invalid_value_variant("Set"),
        }
    }
    pub fn append_operations(&mut self, operations: &[ShapeID]) {
        match self.operations.value_mut() {
            Some(v) => match v {
                Value::Set(vs) => {
                    vs.append(&mut operations.iter().cloned().map(Value::Ref).collect())
                }
                _ => invalid_value_variant("Set"),
            },
            _ => invalid_value_variant("Set"),
        }
    }
    pub fn remove_operation(&mut self, operation: &ShapeID) {
        match self.operations.value_mut() {
            Some(v) => match v {
                Value::Set(vs) => {
                    let id_value = Value::Ref(operation.clone());
                    vs.retain(|v| v == &id_value);
                }
                _ => invalid_value_variant("Set"),
            },
            _ => invalid_value_variant("Set"),
        }
    }

    pub fn resources(&self) -> &Member {
        &self.resources
    }
    pub fn add_resource(&mut self, resource: ShapeID) {
        match &mut self.resources.value_mut() {
            Some(v) => match v {
                Value::Set(vs) => vs.push(Value::Ref(resource)),
                _ => invalid_value_variant("Set"),
            },
            _ => invalid_value_variant("Set"),
        }
    }
    pub fn append_resources(&mut self, resources: &[ShapeID]) {
        match &mut self.resources.value_mut() {
            Some(v) => match v {
                Value::Set(vs) => {
                    vs.append(&mut resources.iter().cloned().map(Value::Ref).collect());
                }
                _ => invalid_value_variant("Set"),
            },
            _ => invalid_value_variant("Set"),
        }
    }
    pub fn remove_resource(&mut self, resource: &ShapeID) {
        match self.resources.value_mut() {
            Some(v) => match v {
                Value::Set(vs) => {
                    let id_value = Value::Ref(resource.clone());
                    vs.retain(|v| v == &id_value);
                }
                _ => invalid_value_variant("Set"),
            },
            _ => invalid_value_variant("Set"),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for Operation {
    fn default() -> Self {
        Self {
            input: Member::new(Identifier::from_str("input").unwrap()),
            output: Member::new(Identifier::from_str("output").unwrap()),
            errors: Member::with_value(
                Identifier::from_str("errors").unwrap(),
                Value::Set(Default::default()),
            ),
        }
    }
}

impl Operation {
    pub fn input(&self) -> &ShapeID {
        match &self.input.value().as_ref().unwrap() {
            Value::Ref(id) => id,
            _ => invalid_value_variant("Set"),
        }
    }
    pub fn set_input(&mut self, input: ShapeID) {
        self.input.set_value(Value::Ref(input))
    }
    pub fn unset_input(&mut self) {
        self.input.unset_value();
    }

    pub fn output(&self) -> &ShapeID {
        match &self.output.value().as_ref().unwrap() {
            Value::Ref(id) => id,
            _ => invalid_value_variant("Set"),
        }
    }
    pub fn set_output(&mut self, output: ShapeID) {
        self.output.set_value(Value::Ref(output))
    }
    pub fn unset_output(&mut self) {
        self.output.unset_value();
    }

    pub fn errors(&self) -> &Member {
        &self.errors
    }
    pub fn add_error(&mut self, error: ShapeID) {
        match &mut self.errors.value_mut() {
            Some(v) => match v {
                Value::List(vs) => vs.push(Value::Ref(error)),
                _ => invalid_value_variant("List"),
            },
            _ => invalid_value_variant("List"),
        }
    }
    pub fn append_errors(&mut self, errors: &[ShapeID]) {
        match &mut self.errors.value_mut() {
            Some(v) => match v {
                Value::List(vs) => {
                    vs.append(&mut errors.iter().cloned().map(Value::Ref).collect());
                }
                _ => invalid_value_variant("List"),
            },
            _ => invalid_value_variant("List"),
        }
    }
    pub fn remove_error(&mut self, error: &ShapeID) {
        match self.errors.value_mut() {
            Some(v) => match v {
                Value::List(vs) => {
                    let id_value = Value::Ref(error.clone());
                    vs.retain(|v| v == &id_value);
                }
                _ => invalid_value_variant("List"),
            },
            _ => invalid_value_variant("List"),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for Resource {
    fn default() -> Self {
        Self {
            identifiers: Member::new(Identifier::from_str("identifiers").unwrap()),
            create: Member::new(Identifier::from_str("create").unwrap()),
            put: Member::new(Identifier::from_str("put").unwrap()),
            read: Member::new(Identifier::from_str("read").unwrap()),
            update: Member::new(Identifier::from_str("update").unwrap()),
            delete: Member::new(Identifier::from_str("delete").unwrap()),
            list: Member::new(Identifier::from_str("list").unwrap()),
            operations: Member::new(Identifier::from_str("operations").unwrap()),
            collection_operations: Member::new(
                Identifier::from_str("collectionOperations").unwrap(),
            ),
            resources: Member::new(Identifier::from_str("resources").unwrap()),
        }
    }
}

impl Resource {}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
