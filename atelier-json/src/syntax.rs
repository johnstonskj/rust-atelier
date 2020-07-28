// ------------------------------------------------------------------------------------------------
// Macros
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
macro_rules! string_const {
    ($name:ident, $value:expr) => {
        pub(crate) const $name: &str = $value;
    };
}

// ------------------------------------------------------------------------------------------------
// JSON Key Names
// ------------------------------------------------------------------------------------------------

string_const!(K_COLLECTION_OPERATIONS, "collectionOperations");
string_const!(K_CREATE, "create");
string_const!(K_DELETE, "delete");
string_const!(K_ERRORS, "errors");
string_const!(K_IDENTIFIERS, "identifiers");
string_const!(K_INPUT, "input");
string_const!(K_KEY, "key");
string_const!(K_LIST, "list");
string_const!(K_MEMBER, "member");
string_const!(K_METADATA, "metadata");
string_const!(K_MEMBERS, "members");
string_const!(K_OPERATIONS, "operations");
string_const!(K_OUTPUT, "output");
string_const!(K_PUT, "put");
string_const!(K_READ, "read");
string_const!(K_RESOURCES, "resources");
string_const!(K_SHAPES, "shapes");
string_const!(K_SMITHY, "smithy");
string_const!(K_TARGET, "target");
string_const!(K_TRAITS, "traits");
string_const!(K_TYPE, "type");
string_const!(K_UPDATE, "update");
string_const!(K_VALUE, "value");
string_const!(K_VERSION, "version");

// ------------------------------------------------------------------------------------------------
// JSON Value Strings
// ------------------------------------------------------------------------------------------------

string_const!(V_APPLY, "apply");
string_const!(V_LIST, "list");
string_const!(V_SET, "set");
string_const!(V_MAP, "map");
string_const!(V_STRUCTURE, "structure");
string_const!(V_UNION, "union");
string_const!(V_SERVICE, "service");
string_const!(V_OPERATION, "operation");
string_const!(V_RESOURCE, "resource");
