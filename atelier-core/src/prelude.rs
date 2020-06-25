/*!
Provides an implementation of the prelude model described in the Smithy specification.

*/

use crate::model::builder::{shapes::SimpleShapeBuilder, ModelBuilder};
use crate::model::{Model, ShapeID};
use crate::Version;
use std::collections::HashMap;

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

lazy_static! {
    static ref PRELUDES: HashMap<Version, Model> = make_prelude_models();
}

///
/// Return a model representing the standard prelude for `version` of the Smithy specification.
///
pub fn prelude_model(version: Version) -> &'static Model {
    PRELUDES.get(&version).unwrap()
}

///
/// Return a list of shape IDs defined in the standard prelude for `version` of the Smithy
/// specification.
///
pub fn prelude_model_shapes(version: Version) -> Vec<ShapeID> {
    PRELUDES.get(&version).unwrap().defined_shapes()
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn make_prelude_models() -> HashMap<Version, Model> {
    let mut map: HashMap<Version, Model> = Default::default();
    let _ = map.insert(Version::V10, prelude_model_v10());
    map
}

fn prelude_model_v10() -> Model {
    ModelBuilder::new(PRELUDE_NAMESPACE, Some(Version::V10))
        // Simple Shapes/Types
        .shape(SimpleShapeBuilder::string(SHAPE_STRING).into())
        .shape(SimpleShapeBuilder::blob(SHAPE_BLOB).into())
        .shape(SimpleShapeBuilder::big_integer(SHAPE_BIGINTEGER).into())
        .shape(SimpleShapeBuilder::big_decimal(SHAPE_BIGDECIMAL).into())
        .shape(SimpleShapeBuilder::timestamp(SHAPE_TIMESTAMP).into())
        .shape(SimpleShapeBuilder::document(SHAPE_DOCUMENT).into())
        .shape(SimpleShapeBuilder::boolean(SHAPE_BOOLEAN).boxed().into())
        .shape(SimpleShapeBuilder::boolean(SHAPE_PRIMITIVEBOOLEAN).into())
        .shape(SimpleShapeBuilder::byte(SHAPE_BYTE).boxed().into())
        .shape(SimpleShapeBuilder::byte(SHAPE_PRIMITIVEBYTE).into())
        .shape(SimpleShapeBuilder::short(SHAPE_SHORT).boxed().into())
        .shape(SimpleShapeBuilder::short(SHAPE_PRIMITIVESHORT).into())
        .shape(SimpleShapeBuilder::integer(SHAPE_INTEGER).boxed().into())
        .shape(SimpleShapeBuilder::integer(SHAPE_PRIMITIVEINTEGER).into())
        .shape(SimpleShapeBuilder::long(SHAPE_LONG).boxed().into())
        .shape(SimpleShapeBuilder::long(SHAPE_PRIMITIVELONG).into())
        .shape(SimpleShapeBuilder::float(SHAPE_FLOAT).boxed().into())
        .shape(SimpleShapeBuilder::float(SHAPE_PRIMITIVEFLOAT).into())
        .shape(SimpleShapeBuilder::double(SHAPE_DOUBLE).boxed().into())
        .shape(SimpleShapeBuilder::double(SHAPE_PRIMITIVEDOUBLE).into())
        // Traits
        .into()
}
