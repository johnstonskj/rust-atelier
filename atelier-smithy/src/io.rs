/*!
One-line description.

More detailed description, with

# Example

*/

use atelier_core::error::Result;
use atelier_core::io::{ModelReader, ModelWriter};
use atelier_core::model::shapes::{Member, ShapeInner, Trait, Valued};
use atelier_core::model::values::NodeValue;
use atelier_core::model::{Annotated, Documented, Model, Named};
use std::io::{Read, Write};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub struct SmithyReader;

pub struct SmithyWriter;

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for SmithyReader {
    fn default() -> Self {
        Self {}
    }
}

impl ModelReader for SmithyReader {
    fn read(&mut self, _r: &mut impl Read) -> Result<Model> {
        unimplemented!()
    }
}

// ------------------------------------------------------------------------------------------------

impl<'a> Default for SmithyWriter {
    fn default() -> Self {
        Self {}
    }
}

impl<'a> ModelWriter<'a> for SmithyWriter {
    fn write(&mut self, w: &mut impl Write, model: &'a Model) -> Result<()> {
        self.write_header(w, model)?;
        self.write_shapes(w, model)?;
        self.write_footer(w, model)
    }
}

impl<'a> SmithyWriter {
    fn write_header(&mut self, w: &mut impl Write, model: &'a Model) -> Result<()> {
        writeln!(
            w,
            "$version: \"{}\"",
            match model.version() {
                None => atelier_core::Version::default().to_string(),
                Some(v) => v.to_string(),
            }
        )?;
        writeln!(w)?;
        writeln!(w, "namespace {}", model.namespace())?;
        writeln!(w)?;
        for use_shape in model.references() {
            writeln!(w, "use {}", use_shape)?;
        }
        writeln!(w)?;
        Ok(())
    }

    fn write_shapes(&mut self, w: &mut impl Write, model: &'a Model) -> Result<()> {
        for shape in model.shapes() {
            if let Some(doc) = shape.documentation() {
                writeln!(w, "/// {}", doc)?;
            }
            for a_trait in shape.traits() {
                self.write_trait(w, a_trait, "")?;
            }
            match shape.inner() {
                ShapeInner::SimpleType(st) => {
                    writeln!(w, "{} {}", st, shape.id())?;
                }
                ShapeInner::List(list) => {
                    writeln!(w, "list {} {{", shape.id())?;
                    writeln!(w, "    member: {}", list.member())?;
                    writeln!(w, "}}")?;
                }
                ShapeInner::Set(set) => {
                    writeln!(w, "set {} {{", shape.id())?;
                    writeln!(w, "    member: {}", set.member())?;
                    writeln!(w, "}}")?;
                }
                ShapeInner::Map(map) => {
                    writeln!(w, "map {} {{", shape.id())?;
                    writeln!(w, "    key: {}", map.key())?;
                    writeln!(w, "    value: {}", map.value())?;
                    writeln!(w, "}}")?;
                }
                ShapeInner::Structure(structured) => {
                    writeln!(w, "structure {} {{", shape.id())?;
                    self.write_members(w, structured.members(), "    ")?;
                    writeln!(w, "}}")?;
                }
                ShapeInner::Union(structured) => {
                    writeln!(w, "union {} {{", shape.id())?;
                    self.write_members(w, structured.members(), "    ")?;
                    writeln!(w, "}}")?;
                }
                ShapeInner::Service(service) => {
                    writeln!(w, "service {} {{", shape.id())?;
                    writeln!(w, "    version: {}", service.version())?;
                    if service.has_operations() {
                        writeln!(
                            w,
                            "    operations: [{}]",
                            service
                                .operations()
                                .map(|s| s.to_string())
                                .collect::<Vec<String>>()
                                .join(", ")
                        )?;
                    }
                    if service.has_resources() {
                        writeln!(
                            w,
                            "    resources: [{}]",
                            service
                                .resources()
                                .map(|s| s.to_string())
                                .collect::<Vec<String>>()
                                .join(", ")
                        )?;
                    }
                    writeln!(w, "}}")?;
                }
                ShapeInner::Operation(operation) => {
                    writeln!(w, "operation {} {{", shape.id())?;
                    if let Some(id) = operation.input() {
                        writeln!(w, "    input: {}", id)?;
                    }
                    if let Some(id) = operation.output() {
                        writeln!(w, "    output: {}", id)?;
                    }
                    if operation.has_errors() {
                        writeln!(
                            w,
                            "    errors: [{}]",
                            operation
                                .errors()
                                .map(|s| s.to_string())
                                .collect::<Vec<String>>()
                                .join(", ")
                        )?;
                    }
                    writeln!(w, "}}")?;
                }
                ShapeInner::Resource(resource) => {
                    writeln!(w, "resource {} {{", shape.id())?;
                    // TODO: identifiers
                    if let Some(id) = resource.create() {
                        writeln!(w, "    create: {}", id)?;
                    }
                    if let Some(id) = resource.put() {
                        writeln!(w, "    put: {}", id)?;
                    }
                    if let Some(id) = resource.read() {
                        writeln!(w, "    read: {}", id)?;
                    }
                    if let Some(id) = resource.update() {
                        writeln!(w, "    update: {}", id)?;
                    }
                    if let Some(id) = resource.delete() {
                        writeln!(w, "    delete: {}", id)?;
                    }
                    if let Some(id) = resource.list() {
                        writeln!(w, "    list: {}", id)?;
                    }
                    if resource.has_operations() {
                        writeln!(
                            w,
                            "    operations: [{}]",
                            resource
                                .operations()
                                .map(|s| s.to_string())
                                .collect::<Vec<String>>()
                                .join(", ")
                        )?;
                    }
                    if resource.has_collection_operations() {
                        writeln!(
                            w,
                            "    collectionOperations: [{}]",
                            resource
                                .collection_operations()
                                .map(|s| s.to_string())
                                .collect::<Vec<String>>()
                                .join(", ")
                        )?;
                    }
                    if resource.has_resources() {
                        writeln!(
                            w,
                            "    resources: [{}]",
                            resource
                                .resources()
                                .map(|s| s.to_string())
                                .collect::<Vec<String>>()
                                .join(", ")
                        )?;
                    }
                    writeln!(w, "}}")?;
                }
            }
            writeln!(w)?;
        }
        Ok(())
    }

    fn write_trait(&mut self, w: &mut impl Write, a_trait: &'a Trait, prefix: &str) -> Result<()> {
        write!(w, "{}@{}", prefix, a_trait.id())?;
        match a_trait.value() {
            None => writeln!(w)?,
            Some(NodeValue::Object(map)) => writeln!(
                w,
                "({})",
                map.iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect::<Vec<String>>()
                    .join(", ")
            )?,
            Some(value) => writeln!(w, "({})", value.to_string())?,
        }
        Ok(())
    }

    fn write_members(
        &mut self,
        w: &mut impl Write,
        members: impl Iterator<Item = &'a Member>,
        prefix: &str,
    ) -> Result<()> {
        for member in members {
            self.write_member(w, member, prefix)?;
        }
        Ok(())
    }

    fn write_member(&mut self, w: &mut impl Write, member: &'a Member, prefix: &str) -> Result<()> {
        if let Some(doc) = member.documentation() {
            writeln!(w, "{}/// {}", prefix, doc)?;
        }
        for a_trait in member.traits() {
            self.write_trait(w, a_trait, prefix)?;
        }
        writeln!(
            w,
            "{}{}: {}",
            prefix,
            member.id(),
            member.value().as_ref().unwrap()
        )?;
        Ok(())
    }

    fn write_footer(&mut self, _w: &mut impl Write, _model: &'a Model) -> Result<()> {
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
