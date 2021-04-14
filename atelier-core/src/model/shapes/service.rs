use crate::model::shapes::TopLevelShape;
use crate::model::values::{Value, ValueMap};
use crate::model::{HasIdentity, ShapeID};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Corresponds to the "service" shape.
///
#[derive(Clone, Debug, PartialEq)]
pub struct Service {
    version: String,
    operations: Vec<ShapeID>,
    resources: Vec<ShapeID>,
}

///
/// Corresponds to the "operation" shape.
///
#[derive(Clone, Debug, PartialEq)]
pub struct Operation {
    input: Option<ShapeID>,
    output: Option<ShapeID>,
    errors: Vec<ShapeID>,
}

///
/// Corresponds to the "resource" shape.
///
#[derive(Clone, Debug, PartialEq)]
pub struct Resource {
    identifiers: ValueMap,
    create: Option<ShapeID>,
    put: Option<ShapeID>,
    read: Option<ShapeID>,
    update: Option<ShapeID>,
    delete: Option<ShapeID>,
    list: Option<ShapeID>,
    operations: Vec<ShapeID>,
    collection_operations: Vec<ShapeID>,
    resources: Vec<ShapeID>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Service {
    pub fn new(version: &str) -> Self {
        assert!(!version.is_empty());
        Self {
            version: version.to_string(),
            operations: Default::default(),
            resources: Default::default(),
        }
    }

    /// Returns the service's version identifier.
    pub fn version(&self) -> &String {
        &self.version
    }

    /// Set this service's version identifier. This **must not** be an empty value.
    pub fn set_version(&mut self, version: &str) {
        assert!(!version.is_empty());
        self.version = version.to_string()
    }

    array_member! { operations, operation, ShapeID }

    /// Add an element to this member's collection.
    pub fn add_operation_shape(&mut self, shape: &TopLevelShape) {
        if shape.is_operation() {
            self.add_operation(shape.id().clone())
        }
    }

    array_member! { resources, resource, ShapeID }

    /// Add an element to this member's collection.
    pub fn add_resource_shape(&mut self, shape: &TopLevelShape) {
        if shape.is_resource() {
            self.add_resource(shape.id().clone())
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for Operation {
    fn default() -> Self {
        Self {
            input: Default::default(),
            output: Default::default(),
            errors: Default::default(),
        }
    }
}

impl Operation {
    optional_member! { input, ShapeID }

    /// Set the current value of this member.
    pub fn set_input_shape(&mut self, shape: &TopLevelShape) {
        if !(shape.is_operation()
            || shape.is_resource()
            || shape.is_service()
            || shape.is_unresolved())
        {
            self.set_input(shape.id().clone())
        }
    }

    optional_member! { output, ShapeID }

    /// Set the current value of this member.
    pub fn set_output_shape(&mut self, shape: &TopLevelShape) {
        if !(shape.is_operation()
            || shape.is_resource()
            || shape.is_service()
            || shape.is_unresolved())
        {
            self.set_output(shape.id().clone())
        }
    }

    array_member! { errors, error, ShapeID }

    /// Add an element to this member's collection.
    pub fn add_error_shape(&mut self, shape: &TopLevelShape) {
        if !(shape.is_operation()
            || shape.is_resource()
            || shape.is_service()
            || shape.is_unresolved())
        {
            self.add_error(shape.id().clone())
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for Resource {
    fn default() -> Self {
        Self {
            identifiers: Default::default(),
            create: Default::default(),
            put: Default::default(),
            read: Default::default(),
            update: Default::default(),
            delete: Default::default(),
            list: Default::default(),
            operations: Default::default(),
            collection_operations: Default::default(),
            resources: Default::default(),
        }
    }
}

impl Resource {
    /// Returns `true` if this resource has _any_ identifiers, else `false`.
    pub fn has_identifiers(&self) -> bool {
        !self.identifiers.is_empty()
    }

    /// Returns `true` if this resource has an identifier with the provided name, else `false`.
    pub fn has_identifier(&self, name: &str) -> bool {
        !self.identifiers.contains_key(name)
    }

    /// Returns the identifier's value, which should always be `Value::String`, representing the
    /// shape ID of the target shape.
    pub fn identifier(&self, name: &str) -> Option<&Value> {
        self.identifiers.get(name)
    }

    /// Remove the identifier from this resource returning any previous value for the identifier name.
    pub fn remove_identifier(&mut self, name: &str) -> Option<Value> {
        self.identifiers.remove(name)
    }

    /// Add an identifier, providing a name and the target shape ID.
    pub fn add_identifier(&mut self, name: &str, target: &str) -> Option<Value> {
        self.identifiers
            .insert(name.to_string(), Value::String(target.to_string()))
    }

    /// Add an identifier, providing a name and the target shape ID. This method will panic if the
    /// target is not a `Value::String`.
    pub fn add_identifier_value(&mut self, name: &str, target: Value) -> Option<Value> {
        assert!(target.is_string());
        self.identifiers.insert(name.to_string(), target)
    }

    /// Return an iterator over all the identifier name and target pairs for this resource.
    pub fn identifiers(&self) -> impl Iterator<Item = (&String, &Value)> {
        self.identifiers.iter()
    }

    /// Returns `true` if this resource has **any** resource operations (create, put, read, update,
    /// delete, or list), else `false`.
    pub fn has_any_resource_operation(&self) -> bool {
        self.has_create()
            || self.has_put()
            || self.has_read()
            || self.has_update()
            || self.has_delete()
            || self.has_list()
    }

    optional_member! { create, ShapeID }

    /// Set the current value of this member.
    pub fn set_create_operation_shape(&mut self, shape: &TopLevelShape) {
        if shape.is_operation() {
            self.set_create(shape.id().clone())
        }
    }

    optional_member! { put, ShapeID }

    /// Set the current value of this member.
    pub fn set_put_operation_shape(&mut self, shape: &TopLevelShape) {
        if shape.is_operation() {
            self.set_put(shape.id().clone())
        }
    }

    optional_member! { read, ShapeID }

    /// Set the current value of this member.
    pub fn set_read_operation_shape(&mut self, shape: &TopLevelShape) {
        if shape.is_operation() {
            self.set_read(shape.id().clone())
        }
    }

    optional_member! { update, ShapeID }

    /// Set the current value of this member.
    pub fn set_update_operation_shape(&mut self, shape: &TopLevelShape) {
        if shape.is_operation() {
            self.set_update(shape.id().clone())
        }
    }

    optional_member! { delete, ShapeID }

    /// Set the current value of this member.
    pub fn set_delete_operation_shape(&mut self, shape: &TopLevelShape) {
        if shape.is_operation() {
            self.set_delete(shape.id().clone())
        }
    }

    optional_member! { list, ShapeID }

    /// Set the current value of this member.
    pub fn set_list_operation_shape(&mut self, shape: &TopLevelShape) {
        if shape.is_operation() {
            self.set_list(shape.id().clone())
        }
    }

    array_member! { operations, operation, ShapeID }

    /// Add an element to this member's collection.
    pub fn add_operation_shape(&mut self, shape: &TopLevelShape) {
        if shape.is_operation() {
            self.add_operation(shape.id().clone())
        }
    }

    array_member! { collection_operations, collection_operation, ShapeID }

    /// Add an element to this member's collection.
    pub fn add_collection_operation_shape(&mut self, shape: &TopLevelShape) {
        if shape.is_operation() {
            self.add_collection_operation(shape.id().clone())
        }
    }

    array_member! { resources, resource, ShapeID }

    /// Add an element to this member's collection.
    pub fn add_resource_shape(&mut self, shape: &TopLevelShape) {
        if shape.is_resource() {
            self.add_resource(shape.id().clone())
        }
    }
}
