use crate::error::{invalid_value_variant, ErrorKind, Result};
use crate::model::shapes::{HasMembers, Member, Valued};
use crate::model::values::{Key, NodeValue};
use crate::model::{Identifier, Named, ShapeID};
use crate::syntax::{
    MEMBER_COLLECTION_OPERATIONS, MEMBER_CREATE, MEMBER_DELETE, MEMBER_ERRORS, MEMBER_IDENTIFIERS,
    MEMBER_INPUT, MEMBER_LIST, MEMBER_OPERATIONS, MEMBER_OUTPUT, MEMBER_PUT, MEMBER_READ,
    MEMBER_RESOURCES, MEMBER_UPDATE, MEMBER_VERSION,
};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Corresponds to the "service" shape.
///
#[derive(Clone, Debug)]
pub struct Service {
    version: Member,    // **required** Value::String
    operations: Member, // Value::Array(Value::ShapeID)
    resources: Member,  // Value::Array(Value::ShapeID)
}

///
/// Corresponds to the "operation" shape.
///
#[derive(Clone, Debug)]
pub struct Operation {
    input: Member,  // Value::ShapeID
    output: Member, // Value::ShapeID
    errors: Member, // Value::Array(Value::ShapeID)
}

///
/// Corresponds to the "resource" shape.
///
#[derive(Clone, Debug)]
pub struct Resource {
    identifiers: Member,           // Value::Object(Identifier, Value::ShapeID)
    create: Member,                // Value::ShapeID
    put: Member,                   // Value::ShapeID
    read: Member,                  // Value::ShapeID
    update: Member,                // Value::ShapeID
    delete: Member,                // Value::ShapeID
    list: Member,                  // Value::ShapeID
    operations: Member,            // Value::List(Value::ShapeID)
    collection_operations: Member, // Value::List(Value::ShapeID)
    resources: Member,             // Value::List(Value::ShapeID)
}

// ------------------------------------------------------------------------------------------------
// Macros
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
macro_rules! optional_member {
    ($has:ident, $member:ident, $setter:ident, $unsetter:ident) => {
        /// Returns `true` if this shape has a value for this member, else `false`.
        pub fn $has(&self) -> bool {
            self.$member.value().is_some()
        }

        /// Return the current value of this member.
        pub fn $member(&self) -> Option<&ShapeID> {
            match &self.$member.value().as_ref() {
                None => None,
                Some(NodeValue::ShapeID(id)) => Some(id),
                _ => invalid_value_variant("ShapeID"),
            }
        }

        /// Set the current value of this member.
        pub fn $setter(&mut self, $member: ShapeID) {
            self.$member.set_value(NodeValue::ShapeID($member))
        }

        /// Set the current value of this member to `None`.
        pub fn $unsetter(&mut self) {
            self.$member.unset_value();
        }
    };
}

#[doc(hidden)]
macro_rules! array_member {
    ($collection:ident, $member:ident, $has_fn:ident, $add_fn:ident, $append_fn:ident, $remove_fn:ident) => {
        /// Returns `true` if this member's collection has _any_ elements, else `false`.
        pub fn $has_fn(&self) -> bool {
            match self.$collection.value() {
                Some(v) => match v {
                    NodeValue::Array(vs) => !vs.is_empty(),
                    _ => invalid_value_variant("Array"),
                },
                _ => invalid_value_variant("Array"),
            }
        }

        /// Return an iterator over all elements in this member's collection.
        pub fn $collection(&self) -> impl Iterator<Item = &ShapeID> {
            match self.$collection.value() {
                Some(v) => match v {
                    NodeValue::Array(vs) => vs.iter().map(|v| {
                        if let NodeValue::ShapeID(id) = v {
                            id
                        } else {
                            invalid_value_variant("ShapeID")
                        }
                    }),
                    _ => invalid_value_variant("Array"),
                },
                _ => invalid_value_variant("Array"),
            }
        }

        /// Add an element to this member's collection.
        pub fn $add_fn(&mut self, $member: ShapeID) {
            match self.$collection.value_mut() {
                Some(v) => match v {
                    NodeValue::Array(vs) => vs.push(NodeValue::ShapeID($member)),
                    _ => invalid_value_variant("Array"),
                },
                _ => invalid_value_variant("Array"),
            }
        }

        /// Add all these elements to this member's collection.
        pub fn $append_fn(&mut self, $collection: &[ShapeID]) {
            match self.$collection.value_mut() {
                Some(v) => match v {
                    NodeValue::Array(vs) => vs.append(
                        &mut $collection
                            .iter()
                            .cloned()
                            .map(NodeValue::ShapeID)
                            .collect(),
                    ),
                    _ => invalid_value_variant("Array"),
                },
                _ => invalid_value_variant("Array"),
            }
        }

        /// Remove an element, with the given identifier, to this member's collection.
        pub fn $remove_fn(&mut self, $member: &ShapeID) {
            match self.$collection.value_mut() {
                Some(v) => match v {
                    NodeValue::Array(vs) => {
                        let id_value = NodeValue::ShapeID($member.clone());
                        vs.retain(|v| v == &id_value);
                    }
                    _ => invalid_value_variant("Array"),
                },
                _ => invalid_value_variant("Array"),
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for Service {
    fn default() -> Self {
        Self {
            version: Member::new(Identifier::from_str(MEMBER_VERSION).unwrap()),
            operations: Member::with_value(
                Identifier::from_str("operations").unwrap(),
                NodeValue::Array(Default::default()),
            ),
            resources: Member::with_value(
                Identifier::from_str("resources").unwrap(),
                NodeValue::Array(Default::default()),
            ),
        }
    }
}

impl HasMembers for Service {
    fn has_member_named(&self, member_name: &Identifier) -> bool {
        [MEMBER_VERSION, MEMBER_OPERATIONS, MEMBER_RESOURCES]
            .contains(&member_name.to_string().as_str())
    }

    fn get_member_named(&self, member_name: &Identifier) -> Option<&Member> {
        if member_name.to_string() == MEMBER_VERSION {
            Some(&self.version)
        } else if member_name.to_string() == MEMBER_OPERATIONS {
            Some(&self.operations)
        } else if member_name.to_string() == MEMBER_RESOURCES {
            Some(&self.resources)
        } else {
            None
        }
    }

    fn set_member(&mut self, member: Member) -> Result<()> {
        if member.id().to_string() == MEMBER_VERSION {
            if let Some(NodeValue::String(_)) = member.value() {
                self.version = member;
                Ok(())
            } else {
                Err(ErrorKind::InvalidValueVariant("String".to_string()).into())
            }
        } else if member.id().to_string() == MEMBER_OPERATIONS {
            if let Some(NodeValue::Array(vs)) = member.value() {
                if vs.iter().all(|v| v.is_shape_id()) {
                    self.operations = member;
                    Ok(())
                } else {
                    Err(ErrorKind::InvalidValueVariant("Array > ShapeID".to_string()).into())
                }
            } else {
                Err(ErrorKind::InvalidValueVariant("Array > ShapeID".to_string()).into())
            }
        } else if member.id().to_string() == MEMBER_RESOURCES {
            if let Some(NodeValue::Array(vs)) = member.value() {
                if vs.iter().all(|v| v.is_shape_id()) {
                    self.resources = member;
                    Ok(())
                } else {
                    Err(ErrorKind::InvalidValueVariant("Array > ShapeID".to_string()).into())
                }
            } else {
                Err(ErrorKind::InvalidValueVariant("Array > ShapeID".to_string()).into())
            }
        } else {
            Err(ErrorKind::UnknownMember(member.id().to_string()).into())
        }
    }
}

impl Service {
    pub fn version(&self) -> &String {
        match &self.version.value().as_ref().unwrap() {
            NodeValue::String(v) => v,
            _ => invalid_value_variant("String"),
        }
    }
    pub fn set_version(&mut self, version: &str) {
        self.version
            .set_value(NodeValue::String(version.to_string()))
    }

    array_member! { operations, operation, has_operations, add_operation, append_operations, remove_operation }

    array_member! { resources, resource, has_resources, add_resource, append_resources, remove_resource }
}

// ------------------------------------------------------------------------------------------------

impl Default for Operation {
    fn default() -> Self {
        Self {
            input: Member::new(Identifier::from_str(MEMBER_INPUT).unwrap()),
            output: Member::new(Identifier::from_str(MEMBER_OUTPUT).unwrap()),
            errors: Member::with_value(
                Identifier::from_str(MEMBER_ERRORS).unwrap(),
                NodeValue::Array(Default::default()),
            ),
        }
    }
}

impl HasMembers for Operation {
    fn has_member_named(&self, member_name: &Identifier) -> bool {
        [MEMBER_INPUT, MEMBER_OUTPUT, MEMBER_ERRORS].contains(&member_name.to_string().as_str())
    }

    fn get_member_named(&self, member_name: &Identifier) -> Option<&Member> {
        if member_name.to_string() == MEMBER_INPUT {
            Some(&self.input)
        } else if member_name.to_string() == MEMBER_OUTPUT {
            Some(&self.output)
        } else if member_name.to_string() == MEMBER_ERRORS {
            Some(&self.errors)
        } else {
            None
        }
    }

    fn set_member(&mut self, member: Member) -> Result<()> {
        if member.id().to_string() == MEMBER_INPUT {
            if let Some(NodeValue::ShapeID(_)) = member.value() {
                self.input = member;
                Ok(())
            } else {
                Err(ErrorKind::InvalidValueVariant("ShapeID".to_string()).into())
            }
        } else if member.id().to_string() == MEMBER_OUTPUT {
            if let Some(NodeValue::ShapeID(_)) = member.value() {
                self.output = member;
                Ok(())
            } else {
                Err(ErrorKind::InvalidValueVariant("ShapeID".to_string()).into())
            }
        } else if member.id().to_string() == MEMBER_ERRORS {
            if let Some(NodeValue::Array(vs)) = member.value() {
                if vs.iter().all(|v| v.is_shape_id()) {
                    self.errors = member;
                    Ok(())
                } else {
                    Err(ErrorKind::InvalidValueVariant("ShapeID".to_string()).into())
                }
            } else {
                Err(ErrorKind::InvalidValueVariant("Array > ShapeID".to_string()).into())
            }
        } else {
            Err(ErrorKind::UnknownMember(member.id().to_string()).into())
        }
    }
}

impl Operation {
    optional_member! { has_input, input, set_input, unset_input }

    optional_member! { has_output, output, set_output, unset_output }

    array_member! { errors, error, has_errors, add_error, append_errors, remove_error }
}

// ------------------------------------------------------------------------------------------------

impl Default for Resource {
    fn default() -> Self {
        Self {
            identifiers: Member::with_value(
                Identifier::from_str(MEMBER_IDENTIFIERS).unwrap(),
                NodeValue::Object(Default::default()),
            ),
            create: Member::new(Identifier::from_str(MEMBER_CREATE).unwrap()),
            put: Member::new(Identifier::from_str(MEMBER_PUT).unwrap()),
            read: Member::new(Identifier::from_str(MEMBER_READ).unwrap()),
            update: Member::new(Identifier::from_str(MEMBER_UPDATE).unwrap()),
            delete: Member::new(Identifier::from_str(MEMBER_DELETE).unwrap()),
            list: Member::new(Identifier::from_str(MEMBER_LIST).unwrap()),
            operations: Member::with_value(
                Identifier::from_str(MEMBER_OPERATIONS).unwrap(),
                NodeValue::Array(Default::default()),
            ),
            collection_operations: Member::with_value(
                Identifier::from_str(MEMBER_COLLECTION_OPERATIONS).unwrap(),
                NodeValue::Array(Default::default()),
            ),
            resources: Member::with_value(
                Identifier::from_str(MEMBER_RESOURCES).unwrap(),
                NodeValue::Array(Default::default()),
            ),
        }
    }
}

impl HasMembers for Resource {
    fn has_member_named(&self, member_name: &Identifier) -> bool {
        [
            MEMBER_IDENTIFIERS,
            MEMBER_CREATE,
            MEMBER_PUT,
            MEMBER_READ,
            MEMBER_UPDATE,
            MEMBER_DELETE,
            MEMBER_LIST,
            MEMBER_OPERATIONS,
            MEMBER_COLLECTION_OPERATIONS,
            MEMBER_RESOURCES,
        ]
        .contains(&member_name.to_string().as_str())
    }

    fn get_member_named(&self, member_name: &Identifier) -> Option<&Member> {
        if member_name.to_string() == MEMBER_IDENTIFIERS {
            Some(&self.identifiers)
        } else if member_name.to_string() == MEMBER_CREATE {
            Some(&self.create)
        } else if member_name.to_string() == MEMBER_PUT {
            Some(&self.put)
        } else if member_name.to_string() == MEMBER_READ {
            Some(&self.read)
        } else if member_name.to_string() == MEMBER_UPDATE {
            Some(&self.update)
        } else if member_name.to_string() == MEMBER_DELETE {
            Some(&self.delete)
        } else if member_name.to_string() == MEMBER_LIST {
            Some(&self.list)
        } else if member_name.to_string() == MEMBER_OPERATIONS {
            Some(&self.operations)
        } else if member_name.to_string() == MEMBER_COLLECTION_OPERATIONS {
            Some(&self.collection_operations)
        } else if member_name.to_string() == "resources" {
            Some(&self.resources)
        } else {
            None
        }
    }

    fn set_member(&mut self, member: Member) -> Result<()> {
        if member.id().to_string() == MEMBER_IDENTIFIERS {
            if let Some(NodeValue::Object(vs)) = member.value() {
                if vs.iter().all(|(k, v)| k.is_identifier() && v.is_shape_id()) {
                    self.identifiers = member;
                    Ok(())
                } else {
                    Err(ErrorKind::InvalidValueVariant("ShapeID".to_string()).into())
                }
            } else {
                Err(ErrorKind::InvalidValueVariant("ShapeID".to_string()).into())
            }
        } else if member.id().to_string() == MEMBER_CREATE {
            if let Some(NodeValue::ShapeID(_)) = member.value() {
                self.create = member;
                Ok(())
            } else {
                Err(ErrorKind::InvalidValueVariant("ShapeID".to_string()).into())
            }
        } else if member.id().to_string() == MEMBER_PUT {
            if let Some(NodeValue::ShapeID(_)) = member.value() {
                self.put = member;
                Ok(())
            } else {
                Err(ErrorKind::InvalidValueVariant("ShapeID".to_string()).into())
            }
        } else if member.id().to_string() == MEMBER_READ {
            if let Some(NodeValue::ShapeID(_)) = member.value() {
                self.read = member;
                Ok(())
            } else {
                Err(ErrorKind::InvalidValueVariant("ShapeID".to_string()).into())
            }
        } else if member.id().to_string() == MEMBER_UPDATE {
            if let Some(NodeValue::ShapeID(_)) = member.value() {
                self.update = member;
                Ok(())
            } else {
                Err(ErrorKind::InvalidValueVariant("ShapeID".to_string()).into())
            }
        } else if member.id().to_string() == MEMBER_DELETE {
            if let Some(NodeValue::ShapeID(_)) = member.value() {
                self.delete = member;
                Ok(())
            } else {
                Err(ErrorKind::InvalidValueVariant("ShapeID".to_string()).into())
            }
        } else if member.id().to_string() == MEMBER_LIST {
            if let Some(NodeValue::ShapeID(_)) = member.value() {
                self.list = member;
                Ok(())
            } else {
                Err(ErrorKind::InvalidValueVariant("ShapeID".to_string()).into())
            }
        } else if member.id().to_string() == MEMBER_OPERATIONS {
            if let Some(NodeValue::Array(vs)) = member.value() {
                if vs.iter().all(|v| v.is_shape_id()) {
                    self.operations = member;
                }
                Ok(())
            } else {
                Err(ErrorKind::InvalidValueVariant("Array > ShapeID".to_string()).into())
            }
        } else if member.id().to_string() == MEMBER_COLLECTION_OPERATIONS {
            if let Some(NodeValue::Array(vs)) = member.value() {
                if vs.iter().all(|v| v.is_shape_id()) {
                    self.collection_operations = member;
                    Ok(())
                } else {
                    Err(ErrorKind::InvalidValueVariant("Array > ShapeID".to_string()).into())
                }
            } else {
                Err(ErrorKind::InvalidValueVariant("Array > ShapeID".to_string()).into())
            }
        } else if member.id().to_string() == MEMBER_RESOURCES {
            if let Some(NodeValue::Array(vs)) = member.value() {
                if vs.iter().all(|v| v.is_shape_id()) {
                    self.resources = member;
                    Ok(())
                } else {
                    Err(ErrorKind::InvalidValueVariant("Array > ShapeID".to_string()).into())
                }
            } else {
                Err(ErrorKind::InvalidValueVariant("Array > ShapeID".to_string()).into())
            }
        } else {
            Err(ErrorKind::UnknownMember(member.id().to_string()).into())
        }
    }
}

impl Resource {
    pub fn has_identifiers(&self) -> bool {
        match self.identifiers.value() {
            Some(v) => match v {
                NodeValue::Object(vs) => !vs.is_empty(),
                _ => invalid_value_variant("Object"),
            },
            _ => invalid_value_variant("Object"),
        }
    }

    pub fn identifiers(&self) -> impl Iterator<Item = (&Identifier, &ShapeID)> {
        match self.identifiers.value() {
            Some(v) => match v {
                NodeValue::Object(vs) => vs
                    .iter()
                    .map(|(k, v)| (k.as_identifier().unwrap(), v.as_reference().unwrap())),
                _ => invalid_value_variant("Object"),
            },
            _ => invalid_value_variant("Object"),
        }
    }

    pub fn add_identifier(&mut self, id: Identifier, shape: ShapeID) {
        match self.identifiers.value_mut() {
            Some(v) => match v {
                NodeValue::Object(vs) => {
                    let _ = vs.insert(id.into(), shape.into());
                }
                _ => invalid_value_variant("Object"),
            },
            _ => invalid_value_variant("Object"),
        }
    }

    pub fn remove_identifier(&mut self, id: &Identifier) {
        match self.identifiers.value_mut() {
            Some(v) => match v {
                NodeValue::Object(vs) => {
                    let key: Key = id.clone().into();
                    vs.retain(|k, _| k == &key);
                }
                _ => invalid_value_variant("Object"),
            },
            _ => invalid_value_variant("Object"),
        }
    }

    optional_member! { has_create, create, set_create, unset_create }

    optional_member! { has_put, put, set_put, unset_put }

    optional_member! { has_read, read, set_read, unset_read }

    optional_member! { has_update, update, set_update, unset_update }

    optional_member! { has_delete, delete, set_delete, unset_delete }

    optional_member! { has_list, list, set_list, unset_list }

    array_member! { operations, operation, has_operations, add_operation, append_operations, remove_operation }

    array_member! { collection_operations, collection_operation, has_collection_operations, add_collection_operation, append_collection_operations, remove_collection_operation }

    array_member! { resources, resource, has_resources, add_resource, append_resources, remove_resource }
}
