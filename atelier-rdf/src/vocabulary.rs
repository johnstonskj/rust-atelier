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
        blob, "Blob",
        boolean, "Boolean",
        document, "Document",
        string, "String",
        byte, "Byte",
        short, "Short",
        integer, "Integer",
        long, "Long",
        float, "Float",
        double, "Double",
        big_integer, "BigInteger",
        big_decimal, "BigDecimal",
        timestamp, "Timestamp",

        // Aggregate types
        list_type, "List",
        set, "Set",
        map, "Map",
        structure, "Structure",
        union, "Union",

        // Service types
        service, "Service",
        operation, "Operation",
        resource, "Resource",

        // Type members
        member, "member",
        key, "key",
        value, "value",
        identifier, "identifier",
        operations, "operations",
        collection_operations, "collectionOperations",
        resources, "resources",
        input, "input",
        output, "output",
        errors, "errors",
        create, "create",
        put, "put",
        update, "update",
        delete, "delete",
        read, "read",
        list_op, "list",

        // Type members
        trait_name, "trait"
    }
}
