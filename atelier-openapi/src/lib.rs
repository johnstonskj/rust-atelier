use std::cell::RefCell;
use std::io::Write;
use std::str::FromStr;

use atelier_core::error::Error as ModelError;
use atelier_core::model::shapes::{AppliedTraits, Service};
use atelier_core::model::values::Value;
use atelier_core::model::visitor::walk_model;
use atelier_core::model::{Model, ShapeID};
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

impl OpenApiModelVisitor {
    fn add_info_object(&self, title: String, version: String) {
        let mut spec = self.spec.borrow_mut();

        spec.info = openapi3::Info {
            version,
            title,
            ..openapi3::Info::default()
        };
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
        let title_trait_id = &ShapeID::from_str("smithy.api#title").unwrap();

        let title = if traits.contains_key(title_trait_id) {
            expect_string_trait_value(traits, title_trait_id)
        } else {
            format!("{}", id.shape_name())
        };

        let version = shape.version().clone();

        self.add_info_object(title, version);

        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn expect_string_trait_value(traits: &AppliedTraits, trait_id: &ShapeID) -> String {
    if let Some(Some(trait_value)) = traits.get(trait_id) {
        match trait_value {
            Value::String(s) => s.clone(),
            v => panic!("Expected trait {} to be a string but was: {}", trait_id, v),
        }
    } else {
        panic!("Expected trait {} not found", trait_id)
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
