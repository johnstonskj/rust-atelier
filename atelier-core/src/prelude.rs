/*!
Provides an implementation of the prelude model described in the Smithy specification.

*/

use crate::model::builder::{shapes::Builder, shapes::SimpleShapeBuilder, ModelBuilder};
use crate::model::Model;
use crate::Version;

// ------------------------------------------------------------------------------------------------
// Macros
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
macro_rules! string_const {
    ($name:ident, $value:expr, $comment:expr) => {
        #[doc = $comment]
        pub const $name: &str = $value;
    };
}

// ------------------------------------------------------------------------------------------------
// Public Names
// ------------------------------------------------------------------------------------------------

///
/// The namespace for the Smith prelude model.
///
pub const PRELUDE_NAMESPACE: &str = "smithy.api";

string_const!(
    SHAPE_STRING,
    "String",
    "The identifier for the simple shape `String`"
);
string_const!(
    SHAPE_BLOB,
    "Blob",
    "The identifier for the simple shape `Blob`"
);
string_const!(
    SHAPE_BIGINTEGER,
    "BigInteger",
    "The identifier for the simple shape `BigInteger`"
);
string_const!(
    SHAPE_BIGDECIMAL,
    "BigDecimal",
    "The identifier for the simple shape `BigDecimal`"
);
string_const!(
    SHAPE_TIMESTAMP,
    "Timestamp",
    "The identifier for the simple shape `Timestamp`"
);
string_const!(
    SHAPE_DOCUMENT,
    "Document",
    "The identifier for the simple shape `Document`"
);
string_const!(
    SHAPE_BOOLEAN,
    "Boolean",
    "The identifier for the simple shape `Boolean`"
);
string_const!(
    SHAPE_PRIMITIVEBOOLEAN,
    "PrimitiveBoolean",
    "The identifier for the simple shape `PrimitiveBoolean`"
);
string_const!(
    SHAPE_BYTE,
    "Byte",
    "The identifier for the simple shape `Byte`"
);
string_const!(
    SHAPE_PRIMITIVEBYTE,
    "PrimitiveByte",
    "The identifier for the simple shape `PrimitiveByte`"
);
string_const!(
    SHAPE_SHORT,
    "Short",
    "The identifier for the simple shape `Short`"
);
string_const!(
    SHAPE_PRIMITIVESHORT,
    "PrimitiveShort",
    "The identifier for the simple shape `PrimitiveShort`"
);
string_const!(
    SHAPE_INTEGER,
    "Integer",
    "The identifier for the simple shape `Integer`"
);
string_const!(
    SHAPE_PRIMITIVEINTEGER,
    "PrimitiveInteger",
    "The identifier for the simple shape `PrimitiveInteger`"
);
string_const!(
    SHAPE_LONG,
    "Long",
    "The identifier for the identifier for the simple shape `Long`"
);
string_const!(
    SHAPE_PRIMITIVELONG,
    "PrimitiveLong",
    "The identifier for the identifier for the simple shape `PrimitiveLong`"
);
string_const!(
    SHAPE_FLOAT,
    "Float",
    "The identifier for the simple shape `Float`"
);
string_const!(
    SHAPE_PRIMITIVEFLOAT,
    "PrimitiveFloat",
    "The identifier for the simple shape `PrimitiveFloat`"
);
string_const!(
    SHAPE_DOUBLE,
    "Double",
    "The identifier for the simple shape `Double`"
);
string_const!(
    SHAPE_PRIMITIVEDOUBLE,
    "PrimitiveDouble",
    "The identifier for the simple shape `PrimitiveDouble`"
);

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// This returns a new model representing the standard prelude as defined by `version` of the
/// Smithy specification.
///
pub fn prelude_model(version: Version) -> Model {
    ModelBuilder::new(PRELUDE_NAMESPACE)
        // Smithy specification version
        .version(version)
        // Simple Shapes/Types
        .shape(SimpleShapeBuilder::string(SHAPE_STRING).build())
        .shape(SimpleShapeBuilder::blob(SHAPE_BLOB).build())
        .shape(SimpleShapeBuilder::big_integer(SHAPE_BIGINTEGER).build())
        .shape(SimpleShapeBuilder::big_decimal(SHAPE_BIGDECIMAL).build())
        .shape(SimpleShapeBuilder::timestamp(SHAPE_TIMESTAMP).build())
        .shape(SimpleShapeBuilder::document(SHAPE_DOCUMENT).build())
        .shape(SimpleShapeBuilder::boolean(SHAPE_BOOLEAN).boxed().build())
        .shape(SimpleShapeBuilder::boolean(SHAPE_PRIMITIVEBOOLEAN).build())
        .shape(SimpleShapeBuilder::byte(SHAPE_BYTE).boxed().build())
        .shape(SimpleShapeBuilder::byte(SHAPE_PRIMITIVEBYTE).build())
        .shape(SimpleShapeBuilder::short(SHAPE_SHORT).boxed().build())
        .shape(SimpleShapeBuilder::short(SHAPE_PRIMITIVESHORT).build())
        .shape(SimpleShapeBuilder::integer(SHAPE_INTEGER).boxed().build())
        .shape(SimpleShapeBuilder::integer(SHAPE_PRIMITIVEINTEGER).build())
        .shape(SimpleShapeBuilder::long(SHAPE_LONG).boxed().build())
        .shape(SimpleShapeBuilder::long(SHAPE_PRIMITIVELONG).build())
        .shape(SimpleShapeBuilder::float(SHAPE_FLOAT).boxed().build())
        .shape(SimpleShapeBuilder::float(SHAPE_PRIMITIVEFLOAT).build())
        .shape(SimpleShapeBuilder::double(SHAPE_DOUBLE).boxed().build())
        .shape(SimpleShapeBuilder::double(SHAPE_PRIMITIVEDOUBLE).build())
        // Traits
        .build()
}
