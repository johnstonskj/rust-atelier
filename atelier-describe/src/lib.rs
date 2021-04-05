/*!
* This crate provides two mechanisms for generating human-readable documentation for a Smithy model
* using the crate [somedoc](https://crates.io/crates/somedoc).
*
* Firstly, the [`DocumentationWriter`](struct.DocumentationWriter.html) structure implements the
* `ModelWriter` trait and so may be used in the same manner as other model writers. The
* `ModelWriter::new` function takes an argument that will denote the format to produce, but provides
* little other control over the generation. Internally this writer implementation calls the following
* function.
*
* The function [`describe_model`](fn.describe_model.html) will produce a
* [`Document`](https://docs.rs/somedoc/0.2.3/somedoc/model/document/struct.Document.html) instance
* from a `Model`. This instance may then be rendered according to the writers provided by somedoc.
* This provides complete control over the actual formatting step and the same generated Document may
* be written multiple times if required.
*
* # Examples
*
* The following demonstrates how to use the `describe_model` function.
*
* ```rust
* use atelier_core::model::Model;
* # use atelier_core::Version;
* use atelier_describe::describe_model;
* use somedoc::write::{write_document_to_string, OutputFormat};
* # fn make_model() -> Model { Model::new(Version::default()) }
*
* let model = make_model();
* let documentation = describe_model(&model).unwrap();
*
* let doc_string = write_document_to_string(&documentation, OutputFormat::Html).unwrap();
* ```
*
* The following example demonstrates the `ModelWriter` trait and outputs the documentation to
* stdout.
*
* ```rust
* use atelier_core::model::Model;
* use atelier_core::io::ModelWriter;
* # use atelier_core::Version;
* use atelier_describe::{describe_model, DocumentationWriter};
* use somedoc::write::{write_document_to_string, OutputFormat};
* use std::io::stdout;
* # fn make_model() -> Model { Model::new(Version::default()) }
*
* let model = make_model();
* let mut writer = DocumentationWriter::new(OutputFormat::Html);
* let documentation = writer.write(&mut stdout(), &model).unwrap();
* ```
*
*/

#![warn(
// ---------- Stylistic
future_incompatible,
nonstandard_style,
rust_2018_idioms,
trivial_casts,
trivial_numeric_casts,
// ---------- Public
missing_debug_implementations,
missing_docs,
unreachable_pub,
// ---------- Unsafe
unsafe_code,
// ---------- Unused
unused_extern_crates,
unused_import_braces,
unused_qualifications,
unused_results,
)]

use atelier_core::error::Result as ModelResult;
use atelier_core::io::ModelWriter;
use atelier_core::model::shapes::{
    HasTraits, ListOrSet, Map, Operation, Resource, Service, ShapeKind, StructureOrUnion,
    TopLevelShape,
};
use atelier_core::model::values::Value;
use atelier_core::model::{HasIdentity, Model, NamespaceID, ShapeID};
use atelier_core::prelude::{
    PRELUDE_NAMESPACE, TRAIT_DOCUMENTATION, TRAIT_EXTERNALDOCUMENTATION, TRAIT_TAGS,
};
use atelier_core::syntax::{
    SHAPE_ID_MEMBER_SEPARATOR, SHAPE_LIST, SHAPE_MAP, SHAPE_OPERATION, SHAPE_RESOURCE,
    SHAPE_SERVICE, SHAPE_SET, SHAPE_STRUCTURE, SHAPE_UNION,
};
use somedoc::model::block::{
    Caption, Cell, Column, HasBlockContent, HasLabel, Heading, Item, Label, List, Paragraph, Row,
    Table,
};
use somedoc::model::inline::HyperLink;
use somedoc::model::inline::{HasInlineContent, InlineContent, Span};
use somedoc::model::Document;
use somedoc::write::{write_document, OutputFormat};
use std::io::Write;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A `ModelWriter` for creating documentation of a model instance.
///
#[derive(Debug)]
pub struct DocumentationWriter {
    output_format: OutputFormat,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Create a `Document` instance describing the `Model` provided. This can then be rendered using
/// the `somedoc` `write_document` or `write_document_to_string` functions.
///
pub fn describe_model(model: &Model) -> ModelResult<Document> {
    let mut document = Document::default();
    describe_this_model(model, &mut document)?;
    Ok(document)
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for DocumentationWriter {
    fn default() -> Self {
        Self {
            output_format: Default::default(),
        }
    }
}

impl ModelWriter for DocumentationWriter {
    fn write(&mut self, w: &mut impl Write, model: &Model) -> ModelResult<()> {
        let document = describe_model(model)?;
        match write_document(&document, self.output_format.clone(), w) {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string().into()),
        }
    }
}

impl DocumentationWriter {
    ///
    /// Construct a new writer with the given output format. The output formats
    /// are part of the `somedoc` crate and allow different documentation technology
    /// to render the content.
    ///
    /// # Example
    ///
    /// ```rust
    /// use atelier_describe::DocumentationWriter;
    /// use somedoc::write::OutputFormat;
    ///
    /// let writer = DocumentationWriter::new(OutputFormat::Html);
    /// ```
    ///
    pub fn new(output_format: OutputFormat) -> Self {
        Self { output_format }
    }
}
// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn describe_this_model(model: &Model, doc: &mut Document) -> ModelResult<()> {
    let doc = doc.set_title("Smithy Model");
    let doc = doc.add_paragraph(format!("Smith Version: {}", model.smithy_version()).into());

    if model.has_metadata() {
        describe_model_metadata(model, doc.add_heading(Heading::section("Metadata")))?;
    }

    if model.has_shapes() {
        for namespace in model.namespaces() {
            describe_model_shapes(model, namespace, doc)?;
        }
    }
    Ok(())
}

fn describe_model_shapes(
    model: &Model,
    namespace: &NamespaceID,
    doc: &mut Document,
) -> ModelResult<()> {
    let mut shape_names: Vec<&ShapeID> = model
        .shape_names()
        .filter(|id| id.namespace() == namespace)
        .collect();
    if !shape_names.is_empty() {
        let doc = doc.add_heading(Heading::section(&format!("Namespace {}", namespace)));
        shape_names.sort_by(|l, r| l.to_string().cmp(&r.to_string()));
        for shape in shape_names {
            let _ = describe_shape(model.shape(shape).unwrap(), doc)?;
        }
    }
    Ok(())
}

fn describe_shape(shape: &TopLevelShape, doc: &mut Document) -> ModelResult<()> {
    let kind_str = match shape.body() {
        ShapeKind::Simple(s) => s.to_string(),
        ShapeKind::List(_) => SHAPE_LIST.to_string(),
        ShapeKind::Set(_) => SHAPE_SET.to_string(),
        ShapeKind::Map(_) => SHAPE_MAP.to_string(),
        ShapeKind::Structure(_) => SHAPE_STRUCTURE.to_string(),
        ShapeKind::Union(_) => SHAPE_UNION.to_string(),
        ShapeKind::Service(_) => SHAPE_SERVICE.to_string(),
        ShapeKind::Operation(_) => SHAPE_OPERATION.to_string(),
        ShapeKind::Resource(_) => SHAPE_RESOURCE.to_string(),
        ShapeKind::Unresolved => "?".to_string(),
    };
    let doc = doc.add_heading(
        Heading::sub_section(&format!("{} ({})", shape.id().shape_name(), kind_str))
            .set_label(shape_id_to_label(shape.id()))
            .clone(),
    );

    describe_documentation(shape, doc)?;

    match shape.body() {
        ShapeKind::List(v) => describe_list_or_set(shape.id(), v, doc),
        ShapeKind::Set(v) => describe_list_or_set(shape.id(), v, doc),
        ShapeKind::Map(v) => describe_map(shape.id(), v, doc),
        ShapeKind::Structure(v) => describe_structure_or_union(shape.id(), v, doc),
        ShapeKind::Union(v) => describe_structure_or_union(shape.id(), v, doc),
        ShapeKind::Service(v) => describe_service(shape.id(), v, doc),
        ShapeKind::Operation(v) => describe_operation(shape.id(), v, doc),
        ShapeKind::Resource(v) => describe_resource(shape.id(), v, doc),
        _ => Ok(()),
    }
}

fn describe_documentation(shape: &impl HasTraits, doc: &mut Document) -> ModelResult<()> {
    let doc_trait_id = ShapeID::new_unchecked(PRELUDE_NAMESPACE, TRAIT_DOCUMENTATION, None);
    for doc_value in shape.traits().iter().filter(|t| t.id() == &doc_trait_id) {
        if let Some(doc_value) = doc_value.value() {
            let _ = doc.add_paragraph(Paragraph::plain_str(doc_value.as_string().unwrap()));
        }
    }

    let doc_trait_id = ShapeID::new_unchecked(PRELUDE_NAMESPACE, TRAIT_EXTERNALDOCUMENTATION, None);
    for doc_value in shape.traits().iter().filter(|t| t.id() == &doc_trait_id) {
        if let Some(Value::Object(value_map)) = doc_value.value() {
            let mut links: Vec<InlineContent> = value_map
                .iter()
                .map(|(k, v)| {
                    InlineContent::HyperLink(HyperLink::external_with_caption(
                        &v.to_string(),
                        Caption::from(k.to_string()),
                    ))
                })
                .collect();
            if !links.is_empty() {
                links.insert(0, Span::plain_str("See also: ").into());
                let _ = doc.add_paragraph(links.into());
            }
        }
    }

    let mut traits = Table::new(&[Column::new("Trait"), Column::new("Value")]);

    if shape.is_boxed() {
        traits.add_row(Row::new(&[
            Cell::plain_str("Is Boxed"),
            Cell::code_str("true"),
        ]));
    }
    if shape.is_deprecated() {
        traits.add_row(Row::new(&[
            Cell::plain_str("Is Deprecated"),
            Cell::code_str("true"),
        ]));
    }
    if shape.is_error() {
        traits.add_row(Row::new(&[
            Cell::plain_str("Is Error"),
            // TODO: value: string
            Cell::code_str("true"),
        ]));
    }
    if shape.is_idempotent() {
        traits.add_row(Row::new(&[
            Cell::plain_str("Is Idempotent"),
            Cell::code_str("true"),
        ]));
    }
    if shape.has_length() {
        traits.add_row(Row::new(&[
            Cell::plain_str("Has Length"),
            // TODO: value: object
            Cell::code_str("true"),
        ]));
    }
    if shape.is_no_replace() {
        traits.add_row(Row::new(&[
            Cell::plain_str("No Replace"),
            Cell::code_str("true"),
        ]));
    }
    if shape.is_paginated() {
        traits.add_row(Row::new(&[
            Cell::plain_str("Paginated"),
            // TODO: value: object
            Cell::code_str("true"),
        ]));
    }
    if shape.has_pattern() {
        traits.add_row(Row::new(&[
            Cell::plain_str("Has Pattern"),
            // TODO: value: string
            Cell::code_str("true"),
        ]));
    }
    if shape.is_private() {
        traits.add_row(Row::new(&[
            Cell::plain_str("Is Private"),
            Cell::code_str("true"),
        ]));
    }
    if shape.is_readonly() {
        traits.add_row(Row::new(&[
            Cell::plain_str("Is Read-only"),
            Cell::code_str("true"),
        ]));
    }
    // TODO: references
    if shape.is_required() {
        traits.add_row(Row::new(&[
            Cell::plain_str("Is Required"),
            Cell::code_str("true"),
        ]));
    }
    // TODO: requires length
    if shape.is_sensitive() {
        traits.add_row(Row::new(&[
            Cell::plain_str("Is Sensitive"),
            Cell::code_str("true"),
        ]));
    }
    if shape.is_streaming() {
        traits.add_row(Row::new(&[
            Cell::plain_str("Is Streaming"),
            Cell::code_str("true"),
        ]));
    }
    // TODO: since, value: String
    if shape.is_tagged() {
        let mut tags: Vec<String> = Vec::new();
        let doc_trait_id = ShapeID::new_unchecked(PRELUDE_NAMESPACE, TRAIT_TAGS, None);
        for doc_value in shape.traits().iter().filter(|t| t.id() == &doc_trait_id) {
            match doc_value.value() {
                Some(Value::Array(values)) => tags.extend(values.iter().map(|v| v.to_string())),
                Some(Value::String(value)) => tags.push(value.clone()),
                _ => {}
            }
        }
        traits.add_row(Row::new(&[
            Cell::plain_str("Has Tags"),
            Cell::code_str(&tags.join(", ")),
        ]));
    }
    // if shape.is_titled() {
    //     traits.add_row(Row::new(&[
    //         Cell::plain_str("Title"),
    //         match shape.ti,
    //     ]));
    // }
    // TODO: title: string
    if shape.is_trait() {
        traits.add_row(Row::new(&[
            Cell::plain_str("Is Trait"),
            Cell::code_str("true"),
        ]));
    }
    if shape.has_unique_items() {
        traits.add_row(Row::new(&[
            Cell::plain_str("Has Unique Items"),
            Cell::code_str("true"),
        ]));
    }
    if shape.is_unstable() {
        traits.add_row(Row::new(&[
            Cell::plain_str("Is Unstable"),
            Cell::code_str("true"),
        ]));
    }

    // TODO: non-prelude traits!

    if traits.has_rows() {
        let _ = doc.add_table(traits);
    }

    Ok(())
}

fn shape_id_to_label(source: &ShapeID) -> Label {
    if let Some(member) = source.member_name() {
        Label::safe_from(
            &format!("{}_{}", source.shape_name(), member),
            Some("member"),
        )
    } else {
        Label::safe_from(&format!("{}", source.shape_name()), Some("shape"))
    }
}

fn shape_link(source: &ShapeID, target: &ShapeID) -> InlineContent {
    if source.namespace() == target.namespace() {
        let caption = if let Some(member) = source.member_name() {
            Caption::from(format!(
                "{}{}{}",
                target.shape_name(),
                SHAPE_ID_MEMBER_SEPARATOR,
                member
            ))
        } else {
            Caption::from(format!("{}", target.shape_name()))
        };
        HyperLink::internal_with_caption(shape_id_to_label(target), caption).into()
    } else {
        Span::code_str(&target.to_string()).into()
    }
}

fn describe_list_or_set(
    shape_id: &ShapeID,
    shape: &ListOrSet,
    doc: &mut Document,
) -> ModelResult<()> {
    let _ = doc.add_paragraph(Paragraph::from(vec![
        Span::bold_str("Member type: ").into(),
        shape_link(shape_id, shape.member().target()),
    ]));
    Ok(())
}

fn describe_map(shape_id: &ShapeID, shape: &Map, doc: &mut Document) -> ModelResult<()> {
    let _ = doc.add_paragraph(Paragraph::from(vec![
        Span::bold_str("Key type: ").into(),
        shape_link(shape_id, shape.key().target()),
        Span::bold_str(", value type: ").into(),
        shape_link(shape_id, shape.value().target()),
    ]));
    Ok(())
}

fn describe_structure_or_union(
    shape_id: &ShapeID,
    shape: &StructureOrUnion,
    doc: &mut Document,
) -> ModelResult<()> {
    if shape.has_members() {
        let _ = doc.add_heading(Heading::sub_sub_section("Members"));
        let mut list = List::unordered();
        for member in shape.members() {
            let _ = list.add_item(Item::from(vec![
                Span::code_str(&member.id().member_name().as_ref().unwrap().to_string()).into(),
                Span::plain_str(": ").into(),
                shape_link(shape_id, member.target()),
            ]));
        }
        let _ = doc.add_list(list);
    }
    Ok(())
}

fn describe_service(shape_id: &ShapeID, shape: &Service, doc: &mut Document) -> ModelResult<()> {
    let _ = doc.add_paragraph(Paragraph::from(vec![
        Span::bold_str("Service version").into(),
        Span::plain_str(": ").into(),
        Span::plain_str(shape.version()).into(),
    ]));
    if shape.has_operations() {
        let _ = doc.add_heading(Heading::sub_sub_section("Operations"));
        let mut list = List::unordered();
        for member_id in shape.operations() {
            let _ = list.add_item(Item::from(shape_link(shape_id, member_id)));
        }
        let _ = doc.add_list(list);
    }
    if shape.has_resources() {
        let _ = doc.add_heading(Heading::sub_sub_section("Resources"));
        let mut list = List::unordered();
        for member_id in shape.resources() {
            let _ = list.add_item(Item::from(shape_link(shape_id, member_id)));
        }
        let _ = doc.add_list(list);
    }
    Ok(())
}

fn describe_operation(
    shape_id: &ShapeID,
    shape: &Operation,
    doc: &mut Document,
) -> ModelResult<()> {
    if let Some(member) = shape.input() {
        let _ = doc.add_paragraph(Paragraph::from(vec![
            Span::bold_str("Input type").into(),
            Span::plain_str(": ").into(),
            shape_link(shape_id, member),
        ]));
    }
    if let Some(member) = shape.output() {
        let _ = doc.add_paragraph(Paragraph::from(vec![
            Span::bold_str("Output type").into(),
            Span::plain_str(": ").into(),
            shape_link(shape_id, member),
        ]));
    }
    if shape.has_errors() {
        let _ = doc.add_paragraph(Paragraph::from(vec![
            Span::bold_str("Errors").into(),
            Span::plain_str(":").into(),
        ]));
        let mut list = List::unordered();
        for error in shape.errors() {
            let _ = list.add_item(shape_link(shape_id, error).into());
        }
        let _ = doc.add_list(list);
    }
    Ok(())
}

fn describe_resource(shape_id: &ShapeID, shape: &Resource, doc: &mut Document) -> ModelResult<()> {
    if shape.has_any_resource_operation() {
        let _ = doc.add_heading(Heading::sub_sub_section("Resource Operations"));
        let mut list = List::unordered();
        if let Some(member) = shape.create() {
            let _ = list.add_item(Item::from(vec![
                Span::code_str("create").into(),
                Span::plain_str(": ").into(),
                shape_link(shape_id, member),
            ]));
        }
        if let Some(member) = shape.put() {
            let _ = list.add_item(Item::from(vec![
                Span::code_str("put").into(),
                Span::plain_str(": ").into(),
                shape_link(shape_id, member),
            ]));
        }
        if let Some(member) = shape.read() {
            let _ = list.add_item(Item::from(vec![
                Span::code_str("read").into(),
                Span::plain_str(": ").into(),
                shape_link(shape_id, member),
            ]));
        }
        if let Some(member) = shape.update() {
            let _ = list.add_item(Item::from(vec![
                Span::code_str("update").into(),
                Span::plain_str(": ").into(),
                shape_link(shape_id, member),
            ]));
        }
        if let Some(member) = shape.delete() {
            let _ = list.add_item(Item::from(vec![
                Span::code_str("delete").into(),
                Span::plain_str(": ").into(),
                shape_link(shape_id, member),
            ]));
        }
        if let Some(member) = shape.list() {
            let _ = list.add_item(Item::from(vec![
                Span::code_str("list").into(),
                Span::plain_str(": ").into(),
                shape_link(shape_id, member),
            ]));
        }
        let _ = doc.add_list(list);
    }
    if shape.has_operations() {
        let _ = doc.add_heading(Heading::sub_sub_section("Operations"));
        let mut list = List::unordered();
        for member_id in shape.operations() {
            let _ = list.add_item(Item::from(shape_link(shape_id, member_id)));
        }
        let _ = doc.add_list(list);
    }
    if shape.has_collection_operations() {
        let _ = doc.add_heading(Heading::sub_sub_section("Collection Operations"));
        let mut list = List::unordered();
        for member_id in shape.collection_operations() {
            let _ = list.add_item(Item::from(shape_link(shape_id, member_id)));
        }
        let _ = doc.add_list(list);
    }
    if shape.has_resources() {
        let _ = doc.add_heading(Heading::sub_sub_section("Resources"));
        let mut list = List::unordered();
        for member_id in shape.resources() {
            let _ = list.add_item(Item::from(shape_link(shape_id, member_id)));
        }
        let _ = doc.add_list(list);
    }
    Ok(())
}

fn describe_model_metadata(model: &Model, doc: &mut Document) -> ModelResult<()> {
    let mut metadata = Table::new(&[Column::new("Key"), Column::new("Value")]);
    for (key, value) in model.metadata() {
        metadata.add_row(Row::new(&[
            Cell::plain_str(key),
            Cell::code_str(&value.to_string()),
        ]));
    }
    let _ = doc.add_table(metadata);
    Ok(())
}
