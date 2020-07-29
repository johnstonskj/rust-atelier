/*!
*/

// ------------------------------------------------------------------------------------------------
// Public Values
// ------------------------------------------------------------------------------------------------

namespace! {
    "smithy",
    "https://awslabs.github.io/smithy/vocab/1.0#",
    {
        // The model root
        model, "Model",
        shapes, "shapes",
        traits, "traits",

        // Simple Types
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

        // Aggregate types
        list_shape, "List",
        set_shape, "Set",
        map_shape, "Map",
        structure_shape, "Structure",
        union_shape, "Union",

        // Service types
        service_shape, "Service",
        operation_shape, "Operation",
        resource_shape, "Resource",

        // Type members
        member, "member",
        key, "key",
        value, "value",
        identifier, "identifier",
        operation, "operation",
        collection_operation, "collectionOperation",
        resource, "resource",
        input, "input",
        output, "output",
        error, "error",
        create, "create",
        put, "put",
        update, "update",
        delete, "delete",
        read, "read",
        list, "list",
        version, "version",

        // Type members
        trait_name, "trait"
    }
}
