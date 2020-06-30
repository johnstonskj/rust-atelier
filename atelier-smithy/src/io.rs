/*!
One-line description.

More detailed description, with

# Example

*/

use crate::parser;
use atelier_core::error::Result;
use atelier_core::io::{ModelReader, ModelWriter};
use atelier_core::model::shapes::{Member, ShapeBody, Trait, Valued};
use atelier_core::model::values::NodeValue;
use atelier_core::model::{Annotated, Model, Named};
use atelier_core::syntax::{
    MEMBER_COLLECTION_OPERATIONS, MEMBER_CREATE, MEMBER_DELETE, MEMBER_ERRORS, MEMBER_IDENTIFIERS,
    MEMBER_INPUT, MEMBER_KEY, MEMBER_LIST, MEMBER_MEMBER, MEMBER_OPERATIONS, MEMBER_OUTPUT,
    MEMBER_PUT, MEMBER_READ, MEMBER_RESOURCES, MEMBER_UPDATE, MEMBER_VALUE, MEMBER_VERSION,
    SHAPE_APPLY, SHAPE_LIST, SHAPE_MAP, SHAPE_OPERATION, SHAPE_RESOURCE, SHAPE_SERVICE, SHAPE_SET,
    SHAPE_STRUCTURE, SHAPE_UNION,
};
use std::io::{Read, Write};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Read a [Model](../atelier_core/model/struct.Model.html) from the Smithy native representation.
///
#[derive(Debug)]
pub struct SmithyReader;

///
/// Write a [Model](../atelier_core/model/struct.Model.html) in the Smithy native representation.
///
#[derive(Debug)]
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
    const REPRESENTATION: &'static str = "Smithy";

    fn read(&mut self, r: &mut impl Read) -> Result<Model> {
        let mut content: String = String::new();
        let _ = r.read_to_string(&mut content)?;
        parser::parse(&content)
    }
}

// ------------------------------------------------------------------------------------------------

const CONTROL_DATA_PREFIX: &str = "$";
const TRAIT_PREFIX: &str = "@";

const STATEMENT_NAMESPACE: &str = "namespace";
const STATEMENT_USE: &str = "use";
const STATEMENT_METADATA: &str = "metadata";

impl<'a> Default for SmithyWriter {
    fn default() -> Self {
        Self {}
    }
}

impl<'a> ModelWriter<'a> for SmithyWriter {
    const REPRESENTATION: &'static str = "Smithy";

    fn write(&mut self, w: &mut impl Write, model: &'a Model) -> Result<()> {
        self.write_header(w, model)?;
        self.write_shapes(w, model)?;
        self.write_footer(w, model)
    }
}

impl<'a> SmithyWriter {
    fn write_header(&mut self, w: &mut impl Write, model: &'a Model) -> Result<()> {
        writeln!(w, "{}: \"{}\"", MEMBER_VERSION, model.version().to_string())?;
        writeln!(w)?;
        if model.has_control_data() {
            for (key, value) in model.control_data() {
                writeln!(w, "{}{} = {}", CONTROL_DATA_PREFIX, key, value)?;
            }
            writeln!(w)?;
        }
        if model.has_metadata() {
            for (key, value) in model.metadata() {
                writeln!(w, "{} \"{}\" = {}", STATEMENT_METADATA, key, value)?;
            }
            writeln!(w)?;
        }
        writeln!(w, "{} {}", STATEMENT_NAMESPACE, model.namespace())?;
        writeln!(w)?;
        for use_shape in model.references() {
            writeln!(w, "{} {}", STATEMENT_USE, use_shape)?;
        }
        writeln!(w)?;
        Ok(())
    }

    fn write_shapes(&mut self, w: &mut impl Write, model: &'a Model) -> Result<()> {
        for shape in model.shapes() {
            if !shape.body().is_apply() {
                for a_trait in shape.traits() {
                    self.write_trait(w, a_trait, "")?;
                }
            }
            match shape.body() {
                ShapeBody::SimpleType(st) => {
                    writeln!(w, "{} {}", st, shape.id())?;
                }
                ShapeBody::List(list) => {
                    writeln!(w, "{} {} {{", SHAPE_LIST, shape.id())?;
                    writeln!(w, "    {}: {}", MEMBER_MEMBER, list.member())?;
                    writeln!(w, "}}")?;
                }
                ShapeBody::Set(set) => {
                    writeln!(w, "{} {} {{", SHAPE_SET, shape.id())?;
                    writeln!(w, "    {}: {}", MEMBER_MEMBER, set.member())?;
                    writeln!(w, "}}")?;
                }
                ShapeBody::Map(map) => {
                    writeln!(w, "{} {} {{", SHAPE_MAP, shape.id())?;
                    writeln!(w, "    {}: {}", MEMBER_KEY, map.key())?;
                    writeln!(w, "    {}: {}", MEMBER_VALUE, map.value())?;
                    writeln!(w, "}}")?;
                }
                ShapeBody::Structure(structured) => {
                    writeln!(w, "{} {} {{", SHAPE_STRUCTURE, shape.id())?;
                    self.write_members(w, structured.members(), "    ")?;
                    writeln!(w, "}}")?;
                }
                ShapeBody::Union(structured) => {
                    writeln!(w, "{} {} {{", SHAPE_UNION, shape.id())?;
                    self.write_members(w, structured.members(), "    ")?;
                    writeln!(w, "}}")?;
                }
                ShapeBody::Service(service) => {
                    writeln!(w, "{} {} {{", SHAPE_SERVICE, shape.id())?;
                    writeln!(w, "    {}: {}", MEMBER_VERSION, service.version())?;
                    if service.has_operations() {
                        writeln!(
                            w,
                            "    {}: [{}]",
                            MEMBER_OPERATIONS,
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
                            "    {}: [{}]",
                            MEMBER_RESOURCES,
                            service
                                .resources()
                                .map(|s| s.to_string())
                                .collect::<Vec<String>>()
                                .join(", ")
                        )?;
                    }
                    writeln!(w, "}}")?;
                }
                ShapeBody::Operation(operation) => {
                    writeln!(w, "{} {} {{", SHAPE_OPERATION, shape.id())?;
                    if let Some(id) = operation.input() {
                        writeln!(w, "    {}: {}", MEMBER_INPUT, id)?;
                    }
                    if let Some(id) = operation.output() {
                        writeln!(w, "    {}: {}", MEMBER_OUTPUT, id)?;
                    }
                    if operation.has_errors() {
                        writeln!(
                            w,
                            "    {}: [{}]",
                            MEMBER_ERRORS,
                            operation
                                .errors()
                                .map(|s| s.to_string())
                                .collect::<Vec<String>>()
                                .join(", ")
                        )?;
                    }
                    writeln!(w, "}}")?;
                }
                ShapeBody::Resource(resource) => {
                    writeln!(w, "{} {} {{", SHAPE_RESOURCE, shape.id())?;
                    if resource.has_identifiers() {
                        writeln!(w, "    {}: {{", MEMBER_IDENTIFIERS)?;
                        for (id, ref_id) in resource.identifiers() {
                            writeln!(w, "        {}: {}", id, ref_id)?;
                        }
                        writeln!(w, "    }}")?;
                    }
                    if let Some(id) = resource.create() {
                        writeln!(w, "    {}: {}", MEMBER_CREATE, id)?;
                    }
                    if let Some(id) = resource.put() {
                        writeln!(w, "    {}: {}", MEMBER_PUT, id)?;
                    }
                    if let Some(id) = resource.read() {
                        writeln!(w, "    {}: {}", MEMBER_READ, id)?;
                    }
                    if let Some(id) = resource.update() {
                        writeln!(w, "    {}: {}", MEMBER_UPDATE, id)?;
                    }
                    if let Some(id) = resource.delete() {
                        writeln!(w, "    {}: {}", MEMBER_DELETE, id)?;
                    }
                    if let Some(id) = resource.list() {
                        writeln!(w, "    {}: {}", MEMBER_LIST, id)?;
                    }
                    if resource.has_operations() {
                        writeln!(
                            w,
                            "    {}: [{}]",
                            MEMBER_OPERATIONS,
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
                            "    {}: [{}]",
                            MEMBER_COLLECTION_OPERATIONS,
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
                            "    {}: [{}]",
                            MEMBER_RESOURCES,
                            resource
                                .resources()
                                .map(|s| s.to_string())
                                .collect::<Vec<String>>()
                                .join(", ")
                        )?;
                    }
                    writeln!(w, "}}")?;
                }
                ShapeBody::Apply => {
                    for a_trait in shape.traits() {
                        write!(w, "{} {} ", SHAPE_APPLY, shape.id())?;
                        self.write_trait(w, a_trait, "")?;
                    }
                }
            }
            writeln!(w)?;
        }
        Ok(())
    }

    fn write_trait(&mut self, w: &mut impl Write, a_trait: &'a Trait, prefix: &str) -> Result<()> {
        write!(w, "{}{}{}", prefix, TRAIT_PREFIX, a_trait.id())?;
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

    fn write_footer(&mut self, _: &mut impl Write, _: &'a Model) -> Result<()> {
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
