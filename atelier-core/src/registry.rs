/*!
Provides a run-time registry for resolving namespaces to models.

*/

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
        self.resolve(&Namespace::from_str(crate::prelude::PRELUDE_NAMESPACE).unwrap())
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
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for SimpleModelRegistry {
    fn default() -> Self {
        let mut initial = Self {
            known: Default::default(),
        };
        initial.register(crate::prelude::prelude_model(Version::current()));
        initial
    }
}

impl ModelRegistry for SimpleModelRegistry {
    fn register(&mut self, model: Model) {
        let _ = self.known.insert(model.namespace().clone(), model);
    }

    fn contains_namespace(&self, namespace: &Namespace) -> bool {
        self.known.contains_key(namespace)
    }

    fn resolve(&self, namespace: &Namespace) -> Option<&Model> {
        self.known.get(namespace)
    }
}
