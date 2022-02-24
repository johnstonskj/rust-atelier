use std::borrow::Borrow;
use std::cell::RefCell;

use atelier_core::error::{Error as ModelError, Result as ModelResult};
use atelier_core::model::visitor::walk_model;
use atelier_core::{io::ModelWriter, model::visitor::ModelVisitor};
use okapi::openapi3;
use serde_json::{to_writer, to_writer_pretty};

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
    fn write(
        &mut self,
        w: &mut impl std::io::Write,
        model: &atelier_core::model::Model,
    ) -> atelier_core::error::Result<()> {
        let visitor = OpenApiModelVisitor {
            spec: RefCell::new(openapi3::OpenApi::default()),
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
        _id: &atelier_core::model::ShapeID,
        _traits: &atelier_core::model::shapes::AppliedTraits,
        shape: &atelier_core::model::shapes::Service,
    ) -> Result<(), Self::Error> {
        let mut spec = self.spec.borrow_mut();

        // let mut info = openapi3::Info::default();
        // info.version = shape.version().clone();
        spec.info = openapi3::Info {
            version: shape.version().clone(),
            ..openapi3::Info::default()
        };

        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
