use std::cell::RefCell;
use std::io::Write;
use std::str::FromStr;

use atelier_core::error::{Error as ModelError, Result as ModelResult};
use atelier_core::model::shapes::{AppliedTraits, Service};
use atelier_core::model::values::Value;
use atelier_core::model::visitor::walk_model;
use atelier_core::model::{Identifier, Model, ShapeID};
use atelier_core::{io::ModelWriter, model::visitor::ModelVisitor};
use okapi::openapi3;
use serde_json::to_writer_pretty;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Writes out a model in the [OpenAPI](https://swagger.io/specification/) format.
///
#[derive(Debug, Default)]
pub struct OpenApiWriter {}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

struct OpenApiModelVisitor {
    spec: RefCell<openapi3::OpenApi>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl ModelWriter for OpenApiWriter {
    fn write(&mut self, w: &mut impl Write, model: &Model) -> atelier_core::error::Result<()> {
        let visitor = OpenApiModelVisitor {
            spec: RefCell::new(openapi3::OpenApi {
                openapi: "3.0.2".to_string(),
                ..openapi3::OpenApi::default()
            }),
        };
        walk_model(model, &visitor)?;

        to_writer_pretty(w, &visitor.spec.into_inner());

        Ok(())
    }
}

impl ModelVisitor for OpenApiModelVisitor {
    type Error = ModelError;

    fn service(
        &self,
        id: &ShapeID,
        traits: &AppliedTraits,
        shape: &Service,
    ) -> Result<(), Self::Error> {
        let mut spec = self.spec.borrow_mut();

        let title_trait_id = &ShapeID::from_str("smithy.api#title").unwrap();

        let title = expect_string_trait_value(traits, title_trait_id)
            .unwrap_or(format!("{}", id.shape_name()));

        // let mut info = openapi3::Info::default();
        // info.version = shape.version().clone();
        spec.info = openapi3::Info {
            version: shape.version().clone(),
            title,
            ..openapi3::Info::default()
        };

        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// TODO: tighten up types, should error if trait doesn't have string value
// TODO: also needs better name
fn expect_string_trait_value(traits: &AppliedTraits, trait_id: &ShapeID) -> Option<String> {
    if let Some(Some(trait_value)) = traits.get(trait_id) {
        match trait_value {
            Value::String(s) => Some(s.clone()),
            _ => None,
        }
    } else {
        None
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
