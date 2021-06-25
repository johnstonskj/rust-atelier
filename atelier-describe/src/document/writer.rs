/*!
This module provides two mechanisms for generating human-readable documentation for a Smithy model
using the crate [somedoc](https://crates.io/crates/somedoc).

The function [`describe_model`](fn.describe_model.html) will produce a
[`Document`](https://docs.rs/somedoc/0.2.3/somedoc/model/document/struct.Document.html) instance
from a `Model`. This instance may then be rendered according to the writers provided by somedoc.
This provides complete control over the actual formatting step and the same generated Document may
be written multiple times if required.

# Examples

The following demonstrates how to use the `describe_model` function.

```rust
use atelier_core::model::Model;
# use atelier_core::Version;
use atelier_describe::document::writer::describe_model;
use somedoc::write::{write_document_to_string, OutputFormat};
# fn make_model() -> Model { Model::new(Version::default()) }

let model = make_model();
let documentation = describe_model(&model).unwrap();

let doc_string = write_document_to_string(&documentation, OutputFormat::Html).unwrap();
```

The [`DocumentationWriter`](struct.DocumentationWriter.html) structure implements the
`ModelWriter` trait and so may be used in the same manner as other model writers.

# Example

The following example demonstrates the `ModelWriter` trait and outputs the documentation, in
[CommonMark](https://spec.commonmark.org/) format, to stdout.

```rust
use atelier_core::model::Model;
use atelier_core::io::ModelWriter;
# use atelier_core::Version;
use atelier_describe::document::writer::DocumentationWriter;
use std::io::stdout;
# fn make_model() -> Model { Model::new(Version::default()) }

let model = make_model();
let mut writer = DocumentationWriter::default();
writer.write(&mut stdout(), &model).expect("Error writing model documentation");
```

*/

use atelier_core::error::Result as ModelResult;
use atelier_core::io::ModelWriter;
use atelier_core::model::shapes::{
    HasTraits, ListOrSet, Map, Operation, Resource, Service, ShapeKind, StructureOrUnion,
    TopLevelShape,
};
use atelier_core::model::values::Value;
use atelier_core::model::{HasIdentity, Model, NamespaceID, ShapeID};
use atelier_core::prelude::{
    prelude_namespace_id, PRELUDE_NAMESPACE, TRAIT_DOCUMENTATION, TRAIT_EXTERNALDOCUMENTATION,
};
use atelier_core::syntax::{
    SHAPE_ID_MEMBER_SEPARATOR, SHAPE_LIST, SHAPE_MAP, SHAPE_OPERATION, SHAPE_RESOURCE,
    SHAPE_SERVICE, SHAPE_SET, SHAPE_STRUCTURE, SHAPE_UNION,
};
use somedoc::model::block::{
    Caption, Cell, Column, HasBlockContent, HasLabel, Heading, Item, Label, List, Paragraph, Quote,
    Row, Table,
};
use somedoc::model::inline::HyperLink;
use somedoc::model::inline::{HasInlineContent, InlineContent, Span};
use somedoc::model::Document;
use somedoc::write::markdown::MarkdownFlavor;
use somedoc::write::{write_document, OutputFormat};
use std::io::Write;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A `ModelWriter` for creating documentation of a model instance. This will always generate
/// [CommonMark](https://spec.commonmark.org/) output as this is the format that Smithy expects in
/// documentation traits and comments.
///
#[derive(Debug)]
pub struct DocumentationWriter {}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Create a `Document` instance describing the `Model` provided. This can then be rendered using
/// the `somedoc` `write_document` or `write_document_to_string` functions.
///
pub fn describe_model(model: &Model) -> ModelResult<Document> {
    let mut document = Document::default();
    describe_this_model(model, &mut document);
    Ok(document)
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for DocumentationWriter {
    fn default() -> Self {
        Self {}
    }
}

impl ModelWriter for DocumentationWriter {
    fn write(&mut self, w: &mut impl Write, model: &Model) -> ModelResult<()> {
        let document = describe_model(model)?;
        match write_document(
            &document,
            OutputFormat::Markdown(MarkdownFlavor::CommonMark),
            w,
        ) {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string().into()),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn describe_this_model(model: &Model, doc: &mut Document) {
    let doc = doc.set_title("Smithy Model");
    let doc = doc.add_paragraph(format!("Smithy Version: {}", model.smithy_version()).into());

    if model.has_metadata() {
        describe_model_metadata(model, doc.add_heading(Heading::section("Metadata")));
    }

    if model.has_shapes() {
        for namespace in model.namespaces() {
            describe_model_shapes(model, namespace, doc);
        }
    }
}

fn describe_model_shapes(model: &Model, namespace: &NamespaceID, doc: &mut Document) {
    let mut shape_names: Vec<&ShapeID> = model
        .shape_names()
        .filter(|id| id.namespace() == namespace)
        .collect();
    if !shape_names.is_empty() {
        let doc = doc.add_heading(Heading::section(&format!("Namespace {}", namespace)));
        shape_names.sort_by_key(|l| l.to_string());
        for shape in shape_names {
            let _ = describe_shape(model.shape(shape).unwrap(), doc);
        }
    }
}

fn describe_shape(shape: &TopLevelShape, doc: &mut Document) {
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

    describe_documentation(shape, doc);

    match shape.body() {
        ShapeKind::List(v) => describe_list_or_set(shape.id(), v, doc),
        ShapeKind::Set(v) => describe_list_or_set(shape.id(), v, doc),
        ShapeKind::Map(v) => describe_map(shape.id(), v, doc),
        ShapeKind::Structure(v) => describe_structure_or_union(shape.id(), v, doc),
        ShapeKind::Union(v) => describe_structure_or_union(shape.id(), v, doc),
        ShapeKind::Service(v) => describe_service(shape.id(), v, doc),
        ShapeKind::Operation(v) => describe_operation(shape.id(), v, doc),
        ShapeKind::Resource(v) => describe_resource(shape.id(), v, doc),
        _ => {}
    }
}

fn trait_value(
    id: String,
    prefix: Option<String>,
    value: &Value,
) -> Vec<(String, Option<String>, Option<String>)> {
    let mut result: Vec<(String, Option<String>, Option<String>)> = Default::default();
    match value {
        Value::Boolean(v) => result.push((id, prefix, Some(v.to_string()))),
        Value::Number(v) => result.push((id, prefix, Some(v.to_string()))),
        Value::String(v) => result.push((id, prefix, Some(v.to_string()))),
        Value::Array(vs) => {
            if vs.is_empty() {
                result.push((id, None, None))
            } else {
                for (i, v) in vs.iter().enumerate() {
                    result.extend(trait_value(id.clone(), Some(format!("[{}]", i)), v));
                }
            }
        }
        Value::Object(vs) => {
            if vs.is_empty() {
                result.push((id, None, None))
            } else {
                let mut obj_keys: Vec<&String> = vs.keys().collect();
                obj_keys.sort();
                for k in obj_keys {
                    let v = vs.get(k).unwrap();
                    result.extend(trait_value(id.clone(), Some(format!(".{}", k)), v));
                }
            }
        }
        _ => result.push((id, None, None)),
    }
    result
}

fn describe_traits(shape: &impl HasTraits, doc: &mut Document) {
    let prelude_namespace = prelude_namespace_id();
    let doc_trait = ShapeID::new_unchecked(PRELUDE_NAMESPACE, TRAIT_DOCUMENTATION, None);
    let ext_doc_trait =
        ShapeID::new_unchecked(PRELUDE_NAMESPACE, TRAIT_EXTERNALDOCUMENTATION, None);
    let mut traits: Vec<(String, Option<String>, Option<String>)> = Default::default();
    let mut trait_ids: Vec<&ShapeID> = shape.traits().iter().map(|(id, _)| id).collect();
    trait_ids.sort();
    for id in trait_ids {
        let value = shape.trait_named(id).unwrap();
        if id != &doc_trait && id != &ext_doc_trait {
            let id = if id.namespace() == prelude_namespace {
                id.shape_name().to_string()
            } else {
                id.to_string()
            };
            if let Some(value) = value {
                traits.extend(trait_value(id.to_string(), None, value));
            } else {
                traits.push((id, None, None));
            }
        }
    }

    if !traits.is_empty() {
        let mut trait_table = Table::new(&[
            Column::new("Trait ID"),
            Column::new("Path"),
            Column::new("Value"),
        ]);
        for applied in traits {
            trait_table.add_row(Row::new(&[
                Cell::plain_str(&applied.0),
                if let Some(v) = &applied.1 {
                    Cell::code_str(v)
                } else {
                    Cell::skip()
                },
                if let Some(v) = &applied.2 {
                    Cell::code_str(v)
                } else {
                    Cell::skip()
                },
            ]));
        }
        let _ = doc.add_table(trait_table);
    }
}

fn describe_documentation(shape: &impl HasTraits, doc: &mut Document) {
    let trait_id = ShapeID::new_unchecked(PRELUDE_NAMESPACE, TRAIT_DOCUMENTATION, None);
    if let Some(Some(doc_value)) = shape.trait_named(&trait_id) {
        let _ = doc.add_paragraph(Paragraph::plain_str(doc_value.as_string().unwrap()));
    }

    let trait_id = ShapeID::new_unchecked(PRELUDE_NAMESPACE, TRAIT_EXTERNALDOCUMENTATION, None);
    if let Some(Some(Value::Object(value_map))) = shape.trait_named(&trait_id) {
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

    describe_traits(shape, doc);
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

fn describe_list_or_set(shape_id: &ShapeID, shape: &ListOrSet, doc: &mut Document) {
    let _ = doc.add_paragraph(Paragraph::from(vec![
        Span::bold_str("Member type: ").into(),
        shape_link(shape_id, shape.member().target()),
    ]));
    describe_documentation(shape.member(), doc);
}

fn describe_map(shape_id: &ShapeID, shape: &Map, doc: &mut Document) {
    let _ = doc.add_paragraph(Paragraph::from(vec![
        Span::bold_str("Key type: ").into(),
        shape_link(shape_id, shape.key().target()),
    ]));
    describe_documentation(shape.key(), doc);
    let _ = doc.add_paragraph(Paragraph::from(vec![
        Span::bold_str(", value type: ").into(),
        shape_link(shape_id, shape.value().target()),
    ]));
    describe_documentation(shape.value(), doc);
}

fn describe_structure_or_union(shape_id: &ShapeID, shape: &StructureOrUnion, doc: &mut Document) {
    if shape.has_members() {
        let _ = doc.add_heading(Heading::sub_sub_section("Members"));
        let mut indent = Quote::default();
        for member in shape.members() {
            let _ = indent.add_paragraph(Paragraph::from(vec![
                Span::code_str(&member.id().to_string()).into(),
                Span::plain_str(": ").into(),
                shape_link(shape_id, member.target()),
            ]));
            describe_documentation(member, doc);
        }
        let _ = doc.add_block_quote(indent);
    }
}

fn describe_service(shape_id: &ShapeID, shape: &Service, doc: &mut Document) {
    let _ = doc.add_paragraph(Paragraph::from(vec![
        Span::bold_str("Service version").into(),
        Span::plain_str(": ").into(),
        Span::plain_str(shape.version()).into(),
    ]));
    if shape.has_operations() {
        let _ = doc.add_heading(Heading::sub_sub_section("Operations"));
        let mut indent = Quote::default();
        let mut list = List::unordered();
        for member_id in shape.operations() {
            let _ = list.add_item(Item::from(shape_link(shape_id, member_id)));
        }
        let _ = indent.add_list(list);
        let _ = doc.add_block_quote(indent);
    }
    if shape.has_resources() {
        let _ = doc.add_heading(Heading::sub_sub_section("Resources"));
        let mut indent = Quote::default();
        let mut list = List::unordered();
        for member_id in shape.resources() {
            let _ = list.add_item(Item::from(shape_link(shape_id, member_id)));
        }
        let _ = indent.add_list(list);
        let _ = doc.add_block_quote(indent);
    }
    if shape.has_renames() {
        let _ = doc.add_heading(Heading::sub_sub_section("Renames"));
        let mut indent = Quote::default();
        let mut list = List::unordered();
        for (shape_id, local_name) in shape.renames() {
            let _ = list.add_item(Item::from(InlineContent::Span(Span::from(vec![
                Span::code_str(&shape_id.to_string()).into(),
                Span::plain_str(" renamed to ").into(),
                Span::code_str(&local_name.to_string()).into(),
            ]))));
        }
        let _ = indent.add_list(list);
        let _ = doc.add_block_quote(indent);
    }
}

fn describe_operation(shape_id: &ShapeID, shape: &Operation, doc: &mut Document) {
    if let Some(member) = shape.input() {
        let mut indent = Quote::default();
        let _ = indent.add_paragraph(Paragraph::from(vec![
            Span::bold_str("Input type").into(),
            Span::plain_str(": ").into(),
            shape_link(shape_id, member),
        ]));
        let _ = doc.add_block_quote(indent);
    }
    if let Some(member) = shape.output() {
        let mut indent = Quote::default();
        let _ = indent.add_paragraph(Paragraph::from(vec![
            Span::bold_str("Output type").into(),
            Span::plain_str(": ").into(),
            shape_link(shape_id, member),
        ]));
        let _ = doc.add_block_quote(indent);
    }
    if shape.has_errors() {
        let mut indent = Quote::default();
        let _ = indent.add_paragraph(Paragraph::from(vec![
            Span::bold_str("Errors").into(),
            Span::plain_str(":").into(),
        ]));
        let mut list = List::unordered();
        for error in shape.errors() {
            let _ = list.add_item(shape_link(shape_id, error).into());
        }
        let _ = indent.add_list(list);
        let _ = doc.add_block_quote(indent);
    }
}

fn describe_resource(shape_id: &ShapeID, shape: &Resource, doc: &mut Document) {
    if shape.has_any_resource_operation() {
        let _ = doc.add_heading(Heading::sub_sub_section("Resource Operations"));
        let mut indent = Quote::default();
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
        let _ = indent.add_list(list);
        let _ = doc.add_block_quote(indent);
    }
    if shape.has_operations() {
        let _ = doc.add_heading(Heading::sub_sub_section("Operations"));
        let mut indent = Quote::default();
        let mut list = List::unordered();
        for member_id in shape.operations() {
            let _ = list.add_item(Item::from(shape_link(shape_id, member_id)));
        }
        let _ = indent.add_list(list);
        let _ = doc.add_block_quote(indent);
    }
    if shape.has_collection_operations() {
        let _ = doc.add_heading(Heading::sub_sub_section("Collection Operations"));
        let mut indent = Quote::default();
        let mut list = List::unordered();
        for member_id in shape.collection_operations() {
            let _ = list.add_item(Item::from(shape_link(shape_id, member_id)));
        }
        let _ = indent.add_list(list);
        let _ = doc.add_block_quote(indent);
    }
    if shape.has_resources() {
        let _ = doc.add_heading(Heading::sub_sub_section("Resources"));
        let mut indent = Quote::default();
        let mut list = List::unordered();
        for member_id in shape.resources() {
            let _ = list.add_item(Item::from(shape_link(shape_id, member_id)));
        }
        let _ = indent.add_list(list);
        let _ = doc.add_block_quote(indent);
    }
}

fn describe_model_metadata(model: &Model, doc: &mut Document) {
    let mut metadata = Table::new(&[Column::new("Key"), Column::new("Value")]);
    for (key, value) in model.metadata() {
        metadata.add_row(Row::new(&[
            Cell::plain_str(key),
            Cell::code_str(&value.to_string()),
        ]));
    }
    let _ = doc.add_table(metadata);
}
