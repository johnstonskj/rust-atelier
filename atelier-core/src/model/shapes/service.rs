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

    array_member! { resources, resource, ShapeID, has_resources, add_resource, append_resources, remove_resources }
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

    optional_member! { output, ShapeID, has_output, set_output, unset_output }

    array_member! { errors, error, ShapeID, has_errors, add_error, append_errors, remove_errors }
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

    optional_member! { put, ShapeID, has_put, set_put, unset_put }

    optional_member! { read, ShapeID, has_read, set_read, unset_read }

    optional_member! { update, ShapeID, has_update, set_update, unset_update }

    optional_member! { delete, ShapeID, has_delete, set_delete, unset_delete }

    optional_member! { list, ShapeID, has_list, set_list, unset_list }

    array_member! { operations, operation, ShapeID, has_operations, add_operation, append_operations, remove_operations }

    array_member! { collection_operations, collection_operation, ShapeID, has_collection_operations, add_collection_operation, append_collection_operations, remove_collection_operations }

    array_member! { resources, resource, ShapeID, has_resources, add_resource, append_resources, remove_resources }
}
