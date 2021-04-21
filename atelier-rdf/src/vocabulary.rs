/*!
This module provides the RDF vocabulary used to describe Smithy semantic models in RDF. Wherever
possible existing RDF semantics, predicates, and idioms are used.
*/

#![allow(missing_docs)]

// ------------------------------------------------------------------------------------------------
// Public Values
// ------------------------------------------------------------------------------------------------

namespace! {
    "smithy",
    "https://awslabs.github.io/smithy/rdf-1.0#",
    {
        // The model root
        model, "Model",
        smithy_version, "smithy_version",
        metadata, "metadata",
        key, "key",
        value, "value",

        // Simple Shapes
        blob_shape, "Blob",
        boolean_shape, "Boolean",
        document_shape, "Document",
        string_shape, "String",
        byte_shape, "Byte",
        short_shape, "Short",
        integer_shape, "Integer",
        long_shape, "Long",
        float_shape, "Float",
        double_shape, "Double",
        big_integer_shape, "BigInteger",
        big_decimal_shape, "BigDecimal",
        timestamp_shape, "Timestamp",

        // Members
        member_shape, "Member",
        target, "target",

        // Lists and Sets
        list_shape, "List",
        set_shape, "Set",
        member_target, "member_target",

        // Maps
        map_shape, "Map",
        key_target, "key_target",
        value_target, "value_target",

        // Structures and Unions
        structure_shape, "Structure",
        union_shape, "Union",
        member, "member",

        // Services
        service_shape, "Service",
        version, "version",
        operation, "operation",
        resource, "resource",
        rename, "rename",
        shape, "shape",
        name, "name",

        // Operations
        operation_shape, "Operation",
        input, "input",
        output, "output",
        error, "error",

        // Resources
        resource_shape, "Resource",
        identifiers, "identifiers",
        create, "create",
        put, "put",
        read, "read",
        update, "update",
        delete, "delete",
        list, "list",
        // + operation
        collection_operation, "collectionOperation",
        // + resource

        // Traits
        apply, "apply",
        trait_shape, "trait",

        // Values
        null, "null"
    }
}
