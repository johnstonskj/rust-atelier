/*!
One-line description.

More detailed description, with

# Example

*/

use crate::model::builder::{shapes, traits, ModelBuilder};
use crate::model::Model;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn prelude_model() -> Model {
    ModelBuilder::new("smithy.api")
        .add(shapes::string("String").build())
        .add(shapes::blob("Blob").build())
        .add(shapes::big_integer("BigInteger").build())
        .add(shapes::big_decimal("BigDecimal").build())
        .add(shapes::timestamp("Timestamp").build())
        .add(shapes::document("Document").build())
        .add(
            shapes::boolean("Boolean")
                .add_trait(traits::is_boxed())
                .build(),
        )
        .add(shapes::boolean("PrimitiveBoolean").build())
        .add(shapes::byte("Byte").add_trait(traits::is_boxed()).build())
        .add(shapes::byte("PrimitiveByte").build())
        .add(shapes::short("Short").add_trait(traits::is_boxed()).build())
        .add(shapes::short("PrimitiveShort").build())
        .add(
            shapes::integer("Integer")
                .add_trait(traits::is_boxed())
                .build(),
        )
        .add(shapes::integer("PrimitiveInteger").build())
        .add(shapes::long("Long").add_trait(traits::is_boxed()).build())
        .add(shapes::long("PrimitiveLong").build())
        .add(shapes::float("Float").add_trait(traits::is_boxed()).build())
        .add(shapes::float("PrimitiveFloat").build())
        .add(
            shapes::double("Double")
                .add_trait(traits::is_boxed())
                .build(),
        )
        .add(shapes::double("PrimitiveDouble").build())
        .build()
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
