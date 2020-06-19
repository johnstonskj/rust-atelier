/*!
Provides a run-time registry for resolving namespaces to models.

*/

use crate::model::builder::{shapes::Builder, shapes::SimpleShapeBuilder, ModelBuilder};
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
        // Smithy specification version
        .version(version)
        // Simple Shapes/Types
        .shape(SimpleShapeBuilder::string("String").build())
        .shape(SimpleShapeBuilder::blob("Blob").build())
        .shape(SimpleShapeBuilder::big_integer("BigInteger").build())
        .shape(SimpleShapeBuilder::big_decimal("BigDecimal").build())
        .shape(SimpleShapeBuilder::timestamp("Timestamp").build())
        .shape(SimpleShapeBuilder::document("Document").build())
        .shape(SimpleShapeBuilder::boolean("Boolean").boxed().build())
        .shape(SimpleShapeBuilder::boolean("PrimitiveBoolean").build())
        .shape(SimpleShapeBuilder::byte("Byte").boxed().build())
        .shape(SimpleShapeBuilder::byte("PrimitiveByte").build())
        .shape(SimpleShapeBuilder::short("Short").boxed().build())
        .shape(SimpleShapeBuilder::short("PrimitiveShort").build())
        .shape(SimpleShapeBuilder::integer("Integer").boxed().build())
        .shape(SimpleShapeBuilder::integer("PrimitiveInteger").build())
        .shape(SimpleShapeBuilder::long("Long").boxed().build())
        .shape(SimpleShapeBuilder::long("PrimitiveLong").build())
        .shape(SimpleShapeBuilder::float("Float").boxed().build())
        .shape(SimpleShapeBuilder::float("PrimitiveFloat").build())
        .shape(SimpleShapeBuilder::double("Double").boxed().build())
        .shape(SimpleShapeBuilder::double("PrimitiveDouble").build())
        // Traits
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
