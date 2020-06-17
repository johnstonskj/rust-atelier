/*!
Provides a run-time registry for resolving namespaces to models.

*/

use crate::model::builder::{shapes, traits, ModelBuilder};
use crate::model::{Model, Namespace};
use crate::Version;
use std::collections::HashMap;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The core trait for a registry, this does not describe whether this is persistent or dynamic.
///
pub trait ModelRegistry: Default {
    ///
    /// Add the model to the registry.
    ///
    fn register(&mut self, model: Model);
    ///
    /// Does the registry contain a model with the given `namespace`?
    ///
    fn contains_namespace(&self, namespace: &Namespace) -> bool;
    ///
    /// Resolve the `namespace` to a `Model`, if known.
    ///
    fn resolve(&self, namespace: &Namespace) -> Option<&Model>;
    ///
    /// Returns the prelude's `Model`, if known.
    ///
    fn prelude(&self) -> Option<&Model> {
        self.resolve(&Namespace::from_str(PRELUDE_NAMESPACE).unwrap())
    }
}

///
/// Simple implementation of `ModelRegistry` which only includes the prelude or any model explicitly
/// added by the client. It does no discovery and is not persistent.
///
#[derive(Clone, Debug)]
pub struct SimpleModelRegistry {
    known: HashMap<Namespace, Model>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// The namespace for the Smith prelude model.
///
pub const PRELUDE_NAMESPACE: &str = "smithy.api";

///
/// This returns a new model representing the standard prelude as defined by `version` of the
/// Smithy specification.
///
pub fn prelude_model(version: Version) -> Model {
    ModelBuilder::new(PRELUDE_NAMESPACE)
        .version(version)
        .shape(shapes::string("String").build())
        .shape(shapes::blob("Blob").build())
        .shape(shapes::big_integer("BigInteger").build())
        .shape(shapes::big_decimal("BigDecimal").build())
        .shape(shapes::timestamp("Timestamp").build())
        .shape(shapes::document("Document").build())
        .shape(
            shapes::boolean("Boolean")
                .add_trait(traits::is_boxed())
                .build(),
        )
        .shape(shapes::boolean("PrimitiveBoolean").build())
        .shape(shapes::byte("Byte").add_trait(traits::is_boxed()).build())
        .shape(shapes::byte("PrimitiveByte").build())
        .shape(shapes::short("Short").add_trait(traits::is_boxed()).build())
        .shape(shapes::short("PrimitiveShort").build())
        .shape(
            shapes::integer("Integer")
                .add_trait(traits::is_boxed())
                .build(),
        )
        .shape(shapes::integer("PrimitiveInteger").build())
        .shape(shapes::long("Long").add_trait(traits::is_boxed()).build())
        .shape(shapes::long("PrimitiveLong").build())
        .shape(shapes::float("Float").add_trait(traits::is_boxed()).build())
        .shape(shapes::float("PrimitiveFloat").build())
        .shape(
            shapes::double("Double")
                .add_trait(traits::is_boxed())
                .build(),
        )
        .shape(shapes::double("PrimitiveDouble").build())
        .build()
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for SimpleModelRegistry {
    fn default() -> Self {
        let mut initial = Self {
            known: Default::default(),
        };
        initial.register(prelude_model(Version::current()));
        initial
    }
}

impl ModelRegistry for SimpleModelRegistry {
    fn register(&mut self, model: Model) {
        self.known.insert(model.namespace().clone(), model);
    }

    fn contains_namespace(&self, namespace: &Namespace) -> bool {
        self.known.contains_key(namespace)
    }

    fn resolve(&self, namespace: &Namespace) -> Option<&Model> {
        self.known.get(namespace)
    }
}
