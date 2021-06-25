/*!
Provides an [`rdf_to_model`](fn.rdf_to_model.html) function and [`RdfReader`](struct.RdfReader.html)
type that will read from an RDF source and construct a model.

# Example - rdf_to_model

Currently unimplemented.

# Example - RdfReader

Currently unimplemented.

*/

use crate::{vocabulary as smithy, REPRESENTATION_NAME};
use atelier_core::error::{ErrorKind, Result as ModelResult};
use atelier_core::io::ModelReader;
use atelier_core::model::shapes::TopLevelShape;
use atelier_core::model::Model;
use atelier_core::Version;
use rdftk_core::model::graph::GraphRef;
use rdftk_core::model::literal::DataType;
use rdftk_core::model::statement::{ObjectNodeRef, SubjectNodeRef};
use rdftk_iri::IRIRef;
use rdftk_names::rdf;
use std::collections::HashSet;
use std::io::Read;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Simple implementation of the `ModelReader` trait that reads the RDF representation of a model.
///
#[derive(Debug)]
pub struct RdfReader {}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Convert an RDF graph into a Smithy semantic model.
///
pub fn rdf_to_model(graph: &GraphRef, model_iri: Option<IRIRef>) -> ModelResult<Model> {
    let model_subject = rdf_model_subject(graph, model_iri)?;

    let version_string = rdf_literal_string(
        graph,
        &model_subject,
        smithy::smithy_version(),
        "version",
        None,
    )?;
    let mut model = Model::new(Version::from_str(&version_string)?);

    for shape in graph.borrow().objects_for(&model_subject, smithy::shape()) {
        let top_level_shape = rdf_to_shape(graph, shape)?;
        let _ = model.add_shape(top_level_shape);
    }
    Ok(model)
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl ModelReader for RdfReader {
    fn read(&mut self, _r: &mut impl Read) -> atelier_core::error::Result<Model> {
        unimplemented!()
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn rdf_literal_string(
    graph: &GraphRef,
    subject: &SubjectNodeRef,
    predicate: &IRIRef,
    name: &str,
    data_type: Option<DataType>,
) -> ModelResult<String> {
    let graph = graph.borrow();
    let object_nodes: HashSet<&ObjectNodeRef> = graph.objects_for(&subject, predicate);
    if object_nodes.len() == 1 {
        if let Some(literal) = object_nodes.iter().next().unwrap().as_literal() {
            if data_type.is_some() && literal.data_type() != data_type.as_ref() {
                return Err(ErrorKind::Deserialization(
                    REPRESENTATION_NAME.to_string(),
                    name.to_string(),
                    Some("data_type".to_string()),
                )
                .into());
            }
            Ok(literal.lexical_form().clone())
        } else {
            Err(ErrorKind::Deserialization(
                REPRESENTATION_NAME.to_string(),
                name.to_string(),
                Some("as_literal".to_string()),
            )
            .into())
        }
    } else {
        Err(ErrorKind::Deserialization(
            REPRESENTATION_NAME.to_string(),
            name.to_string(),
            Some("len".to_string()),
        )
        .into())
    }
}

fn rdf_model_subject(graph: &GraphRef, model_iri: Option<IRIRef>) -> ModelResult<SubjectNodeRef> {
    let ref_graph = graph.borrow();
    let model_object = ref_graph
        .statement_factory()
        .named_object(smithy::model().clone());
    if let Some(model_iri) = model_iri {
        let subject = ref_graph.statement_factory().named_subject(model_iri);
        if ref_graph.contains_all(&subject, rdf::a_type(), &model_object) {
            Ok(subject)
        } else {
            Err(ErrorKind::Deserialization(
                REPRESENTATION_NAME.to_string(),
                "model".to_string(),
                None,
            )
            .into())
        }
    } else {
        let subjects: HashSet<&SubjectNodeRef> = ref_graph
            .matches(None, Some(rdf::a_type()), Some(&model_object))
            .iter()
            .map(|st| st.subject())
            .collect();
        if subjects.len() == 1 {
            let subject = *subjects.iter().next().unwrap();
            Ok(subject.clone())
        } else {
            Err(ErrorKind::Deserialization(
                REPRESENTATION_NAME.to_string(),
                "model".to_string(),
                None,
            )
            .into())
        }
    }
}

fn rdf_to_shape(
    _graph: &GraphRef,
    _subject_as_object: &ObjectNodeRef,
) -> ModelResult<TopLevelShape> {
    // if let Some(iri) = subject_as_object.as_iri() && is_shape_iri(iri) {
    //     // Find type
    //     // Find members
    //     // Apply traits
    unimplemented!()
    // } else {
    //     Err(
    //         ErrorKind::Deserialization(REPRESENTATION_NAME.to_string(), "shape".to_string(), None)
    //             .into(),
    //     )
    // }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
