use std::io::Write;

use atelier_core::error::Result;
use atelier_core::io::ModelWriter;
use atelier_core::model::shapes::{HasTraits, MemberShape, ShapeKind};
use atelier_core::model::values::Value;
use atelier_core::model::{HasIdentity, Model, NamespaceID, ShapeID};
use atelier_core::syntax::{
    MEMBER_COLLECTION_OPERATIONS, MEMBER_CREATE, MEMBER_DELETE, MEMBER_ERRORS, MEMBER_IDENTIFIERS,
    MEMBER_INPUT, MEMBER_KEY, MEMBER_LIST, MEMBER_MEMBER, MEMBER_OPERATIONS, MEMBER_OUTPUT,
    MEMBER_PUT, MEMBER_READ, MEMBER_RESOURCES, MEMBER_UPDATE, MEMBER_VALUE, MEMBER_VERSION,
    SHAPE_APPLY, SHAPE_LIST, SHAPE_MAP, SHAPE_OPERATION, SHAPE_RESOURCE, SHAPE_SERVICE, SHAPE_SET,
    SHAPE_STRUCTURE, SHAPE_UNION,
};

use crate::syntax::{
    CONTROL_DATA_PREFIX, STATEMENT_METADATA, STATEMENT_NAMESPACE, STATEMENT_USE, TRAIT_PREFIX,
};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This struct implements the `ModelWriter` trait to write a [Model](../atelier_core/model/struct.Model.html)
/// in the [Smithy IDL](https://awslabs.github.io/smithy/1.0/spec/core/idl.html) representation.
///
/// Currently the Smithy writer takes only one parameter which is the namespace to filter the semantic
/// model.
///
///```rust
/// use atelier_core::model::NamespaceID;
/// use atelier_smithy::SmithyWriter;
/// use std::str::FromStr;
///
/// let writer = SmithyWriter::new(NamespaceID::from_str("org.example").unwrap());
///
#[derive(Debug)]
pub struct SmithyWriter {
    namespace: NamespaceID,
    prelude_namespace: NamespaceID,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl ModelWriter for SmithyWriter {
    fn write(&mut self, w: &mut impl Write, model: &Model) -> Result<()> {
        self.write_control_section(w, model)?;
        self.write_metadata_section(w, model)?;
        self.write_shape_section(w, model)
    }
}

impl SmithyWriter {
    pub fn new(namespace: NamespaceID) -> Self {
        Self {
            namespace,
            prelude_namespace: NamespaceID::new_unchecked(atelier_core::prelude::PRELUDE_NAMESPACE),
        }
    }

    fn write_control_section(&mut self, w: &mut impl Write, model: &Model) -> Result<()> {
        writeln!(
            w,
            "{}{}: \"{}\"",
            CONTROL_DATA_PREFIX,
            MEMBER_VERSION,
            model.smithy_version().to_string()
        )?;
        writeln!(w)?;
        Ok(())
    }

    fn write_metadata_section(&mut self, w: &mut impl Write, model: &Model) -> Result<()> {
        if model.has_metadata() {
            for (key, value) in model.metadata() {
                write!(w, "{} \"{}\" = ", STATEMENT_METADATA, key)?;
                self.write_value(w, value, true)?;
                writeln!(w)?;
            }
            writeln!(w)?;
        }
        Ok(())
    }

    fn write_shape_section(&mut self, w: &mut impl Write, model: &Model) -> Result<()> {
        self.write_namespace_statement(w, model)?;
        self.write_use_section(w, model)?;
        self.write_shape_statements(w, model)
    }
    fn write_namespace_statement(&mut self, w: &mut impl Write, _: &Model) -> Result<()> {
        writeln!(w, "{} {}", STATEMENT_NAMESPACE, self.namespace)?;
        writeln!(w)?;
        Ok(())
    }

    fn write_use_section(&mut self, w: &mut impl Write, model: &Model) -> Result<()> {
        for use_shape in model
            .shapes()
            .filter(|shape| shape.is_unresolved() && !shape.has_traits())
        {
            writeln!(w, "{} {}", STATEMENT_USE, use_shape.id())?;
        }
        writeln!(w)?;
        Ok(())
    }

    fn write_shape_statements(&mut self, w: &mut impl Write, model: &Model) -> Result<()> {
        let from_namespace = self.namespace.clone();
        for shape in model
            .shapes()
            .filter(|shape| shape.id().namespace() == &from_namespace)
        {
            if !shape.body().is_unresolved() {
                for (id, value) in shape.traits() {
                    self.write_trait(w, id, value, "")?;
                }
            }
            match shape.body() {
                ShapeKind::Simple(st) => {
                    writeln!(w, "{} {}", st, shape.id().shape_name())?;
                }
                ShapeKind::List(list) => {
                    writeln!(w, "{} {} {{", SHAPE_LIST, shape.id().shape_name())?;
                    writeln!(w, "    {}: {}", MEMBER_MEMBER, list.member().target())?;
                    writeln!(w, "}}")?;
                }
                ShapeKind::Set(set) => {
                    writeln!(w, "{} {} {{", SHAPE_SET, shape.id().shape_name())?;
                    writeln!(w, "    {}: {}", MEMBER_MEMBER, set.member().target())?;
                    writeln!(w, "}}")?;
                }
                ShapeKind::Map(map) => {
                    writeln!(w, "{} {} {{", SHAPE_MAP, shape.id().shape_name())?;
                    writeln!(w, "    {}: {}", MEMBER_KEY, map.key().target().shape_name())?;
                    writeln!(
                        w,
                        "    {}: {}",
                        MEMBER_VALUE,
                        map.value().target().shape_name()
                    )?;
                    writeln!(w, "}}")?;
                }
                ShapeKind::Structure(structured) => {
                    writeln!(w, "{} {} {{", SHAPE_STRUCTURE, shape.id().shape_name())?;
                    self.write_members(w, structured.members(), "    ")?;
                    writeln!(w, "}}")?;
                }
                ShapeKind::Union(structured) => {
                    writeln!(w, "{} {} {{", SHAPE_UNION, shape.id().shape_name())?;
                    self.write_members(w, structured.members(), "    ")?;
                    writeln!(w, "}}")?;
                }
                ShapeKind::Service(service) => {
                    writeln!(w, "{} {} {{", SHAPE_SERVICE, shape.id().shape_name())?;
                    writeln!(w, "    {}: {:?}", MEMBER_VERSION, service.version())?;
                    if service.has_operations() {
                        writeln!(
                            w,
                            "    {}: [{}]",
                            MEMBER_OPERATIONS,
                            service
                                .operations()
                                .map(|s| s.shape_name().to_string())
                                .collect::<Vec<String>>()
                                .join(", ")
                        )?;
                    }
                    if service.has_resources() {
                        writeln!(
                            w,
                            "    {}: [{}]",
                            MEMBER_RESOURCES,
                            service
                                .resources()
                                .map(|s| s.shape_name().to_string())
                                .collect::<Vec<String>>()
                                .join(", ")
                        )?;
                    }
                    writeln!(w, "}}")?;
                }
                ShapeKind::Operation(operation) => {
                    writeln!(w, "{} {} {{", SHAPE_OPERATION, shape.id().shape_name())?;
                    if let Some(id) = operation.input() {
                        writeln!(w, "    {}: {}", MEMBER_INPUT, id.shape_name())?;
                    }
                    if let Some(id) = operation.output() {
                        writeln!(w, "    {}: {}", MEMBER_OUTPUT, id.shape_name())?;
                    }
                    if operation.has_errors() {
                        writeln!(
                            w,
                            "    {}: [{}]",
                            MEMBER_ERRORS,
                            operation
                                .errors()
                                .map(|s| s.shape_name().to_string())
                                .collect::<Vec<String>>()
                                .join(", ")
                        )?;
                    }
                    writeln!(w, "}}")?;
                }
                ShapeKind::Resource(resource) => {
                    writeln!(w, "{} {} {{", SHAPE_RESOURCE, shape.id().shape_name())?;
                    if resource.has_identifiers() {
                        writeln!(w, "    {}: {{", MEMBER_IDENTIFIERS)?;
                        for (id, target) in resource.identifiers() {
                            writeln!(w, "        {}: {}", id, target.shape_name())?;
                        }
                        writeln!(w, "    }}")?;
                    }
                    if let Some(id) = resource.create() {
                        writeln!(w, "    {}: {}", MEMBER_CREATE, id.shape_name())?;
                    }
                    if let Some(id) = resource.put() {
                        writeln!(w, "    {}: {}", MEMBER_PUT, id.shape_name())?;
                    }
                    if let Some(id) = resource.read() {
                        writeln!(w, "    {}: {}", MEMBER_READ, id.shape_name())?;
                    }
                    if let Some(id) = resource.update() {
                        writeln!(w, "    {}: {}", MEMBER_UPDATE, id.shape_name())?;
                    }
                    if let Some(id) = resource.delete() {
                        writeln!(w, "    {}: {}", MEMBER_DELETE, id.shape_name())?;
                    }
                    if let Some(id) = resource.list() {
                        writeln!(w, "    {}: {}", MEMBER_LIST, id.shape_name())?;
                    }
                    if resource.has_operations() {
                        writeln!(
                            w,
                            "    {}: [{}]",
                            MEMBER_OPERATIONS,
                            resource
                                .operations()
                                .map(|s| s.shape_name().to_string())
                                .collect::<Vec<String>>()
                                .join(", ")
                        )?;
                    }
                    if resource.has_collection_operations() {
                        writeln!(
                            w,
                            "    {}: [{}]",
                            MEMBER_COLLECTION_OPERATIONS,
                            resource
                                .collection_operations()
                                .map(|s| s.shape_name().to_string())
                                .collect::<Vec<String>>()
                                .join(", ")
                        )?;
                    }
                    if resource.has_resources() {
                        writeln!(
                            w,
                            "    {}: [{}]",
                            MEMBER_RESOURCES,
                            resource
                                .resources()
                                .map(|s| s.shape_name().to_string())
                                .collect::<Vec<String>>()
                                .join(", ")
                        )?;
                    }
                    writeln!(w, "}}")?;
                }
                ShapeKind::Unresolved => {
                    if shape.has_traits() {
                        for (id, value) in shape.traits() {
                            write!(w, "{} {} ", SHAPE_APPLY, shape.id())?;
                            self.write_trait(w, id, value, "")?;
                        }
                    }
                }
            }
            writeln!(w)?;
        }
        Ok(())
    }

    fn write_trait(
        &mut self,
        w: &mut impl Write,
        id: &ShapeID,
        value: &Option<Value>,
        prefix: &str,
    ) -> Result<()> {
        let trait_namespace = id.namespace();
        let id = if *trait_namespace == self.namespace || *trait_namespace == self.prelude_namespace
        {
            id.shape_name().to_string()
        } else {
            id.to_string()
        };
        write!(w, "{}{}{}", prefix, TRAIT_PREFIX, id)?;
        match value {
            None => writeln!(w)?,
            Some(v) => {
                write!(w, "(")?;
                self.write_value(w, v, true)?;
                writeln!(w, ")")?;
            }
        }
        Ok(())
    }

    fn write_value(&mut self, w: &mut impl Write, value: &Value, top: bool) -> Result<()> {
        match value {
            Value::Array(vs) => {
                write!(w, "[")?;
                let last = if vs.is_empty() { 0 } else { vs.len() - 1 };
                for (i, v) in vs.iter().enumerate() {
                    self.write_value(w, v, false)?;
                    if i < last {
                        write!(w, ", ")?;
                    }
                }
                write!(w, "]")?;
            }
            Value::Object(vs) => {
                if !top {
                    write!(w, "{{")?;
                }
                let last = if vs.is_empty() { 0 } else { vs.len() - 1 };
                for (i, (k, v)) in vs.iter().enumerate() {
                    write!(w, "{}: ", k)?;
                    self.write_value(w, v, false)?;
                    if i < last {
                        write!(w, ", ")?;
                    }
                }
                if !top {
                    write!(w, "}}")?;
                }
            }
            Value::Number(v) => write!(w, "{}", v)?,
            Value::Boolean(v) => write!(w, "{}", v)?,
            Value::String(v) => write!(w, "{:?}", v)?,
            Value::None => {}
        }
        Ok(())
    }

    fn write_members<'a>(
        &'a mut self,
        w: &mut impl Write,
        members: impl Iterator<Item = &'a MemberShape>,
        prefix: &str,
    ) -> Result<()> {
        for member in members {
            self.write_member(w, member, prefix)?;
        }
        Ok(())
    }

    fn write_member(
        &mut self,
        w: &mut impl Write,
        member: &MemberShape,
        prefix: &str,
    ) -> Result<()> {
        for (id, value) in member.traits() {
            self.write_trait(w, id, value, prefix)?;
        }
        writeln!(
            w,
            "{}{}: {}",
            prefix,
            member.id(),
            member.target().shape_name()
        )?;
        Ok(())
    }
}
