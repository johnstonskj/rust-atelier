use crate::model::shapes::{Shape, TopLevelShape};
use crate::model::values::{Value, ValueMap};
use crate::model::ShapeID;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Corresponds to the "service" shape.
///
#[derive(Clone, Debug)]
pub struct Service {
    version: String,
    operations: Vec<ShapeID>,
    resources: Vec<ShapeID>,
}

///
/// Corresponds to the "operation" shape.
///
#[derive(Clone, Debug)]
pub struct Operation {
    input: Option<ShapeID>,
    output: Option<ShapeID>,
    errors: Vec<ShapeID>,
}

///
/// Corresponds to the "resource" shape.
///
#[derive(Clone, Debug)]
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

    array_member! { operations, operation, ShapeID, has_operations, add_operation, append_operations, remove_operations }

    pub fn add_operation_shape(&mut self, shape: &TopLevelShape) {
        if shape.is_operation() {
            self.add_operation(shape.id().clone())
        }
    }

    array_member! { resources, resource, ShapeID, has_resources, add_resource, append_resources, remove_resources }

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
    optional_member! { input, ShapeID, has_input, set_input, unset_input }

    pub fn set_input_shape(&mut self, shape: &TopLevelShape) {
        if !(shape.is_operation()
            || shape.is_resource()
            || shape.is_service()
            || shape.is_unresolved())
        {
            self.set_input(shape.id().clone())
        }
    }

    optional_member! { output, ShapeID, has_output, set_output, unset_output }

    pub fn set_output_shape(&mut self, shape: &TopLevelShape) {
        if !(shape.is_operation()
            || shape.is_resource()
            || shape.is_service()
            || shape.is_unresolved())
        {
            self.set_output(shape.id().clone())
        }
    }

    array_member! { errors, error, ShapeID, has_errors, add_error, append_errors, remove_errors }

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
    object_member! { identifiers, identifier, String => Value, has_identifiers, has_identifier, add_identifier, remove_identifier }

    optional_member! { create, ShapeID, has_create, set_create, unset_create }

    pub fn set_create_operation_shape(&mut self, shape: &TopLevelShape) {
        if shape.is_operation() {
            self.set_create(shape.id().clone())
        }
    }

    optional_member! { put, ShapeID, has_put, set_put, unset_put }

    pub fn set_put_operation_shape(&mut self, shape: &TopLevelShape) {
        if shape.is_operation() {
            self.set_put(shape.id().clone())
        }
    }

    optional_member! { read, ShapeID, has_read, set_read, unset_read }

    pub fn set_read_operation_shape(&mut self, shape: &TopLevelShape) {
        if shape.is_operation() {
            self.set_read(shape.id().clone())
        }
    }

    optional_member! { update, ShapeID, has_update, set_update, unset_update }

    pub fn set_update_operation_shape(&mut self, shape: &TopLevelShape) {
        if shape.is_operation() {
            self.set_update(shape.id().clone())
        }
    }

    optional_member! { delete, ShapeID, has_delete, set_delete, unset_delete }

    pub fn set_delete_operation_shape(&mut self, shape: &TopLevelShape) {
        if shape.is_operation() {
            self.set_delete(shape.id().clone())
        }
    }

    optional_member! { list, ShapeID, has_list, set_list, unset_list }

    pub fn set_list_operation_shape(&mut self, shape: &TopLevelShape) {
        if shape.is_operation() {
            self.set_list(shape.id().clone())
        }
    }

    array_member! { operations, operation, ShapeID, has_operations, add_operation, append_operations, remove_operations }

    pub fn add_operation_shape(&mut self, shape: &TopLevelShape) {
        if shape.is_operation() {
            self.add_operation(shape.id().clone())
        }
    }

    array_member! { collection_operations, collection_operation, ShapeID, has_collection_operations, add_collection_operation, append_collection_operations, remove_collection_operations }

    pub fn add_collection_operation_shape(&mut self, shape: &TopLevelShape) {
        if shape.is_operation() {
            self.add_collection_operation(shape.id().clone())
        }
    }

    array_member! { resources, resource, ShapeID, has_resources, add_resource, append_resources, remove_resources }

    pub fn add_resource_shape(&mut self, shape: &TopLevelShape) {
        if shape.is_resource() {
            self.add_resource(shape.id().clone())
        }
    }
}
