/*!
One-line description.

More detailed description, with

# Example

*/

use atelier_core::error::Result;
use atelier_core::io::{ModelReader, ModelWriter};
use atelier_core::model::shapes::ShapeInner;
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

impl Default for SmithyWriter {
    fn default() -> Self {
        Self {}
    }
}

impl ModelWriter for SmithyWriter {
    fn write(&mut self, w: &mut impl Write, model: &Model) -> Result<()> {
        self.write_header(w, model)?;
        self.write_shapes(w, model)?;
        self.write_footer(w, model)
    }
}

impl SmithyWriter {
    fn write_header(&mut self, w: &mut impl Write, model: &Model) -> Result<()> {
        writeln!(w, "$version: \"{}\"", model.version())?;
        writeln!(w)?;
        writeln!(w, "namespace {}", model.namespace())?;
        writeln!(w)?;
        for use_shape in model.uses() {
            writeln!(w, "use {}", use_shape)?;
        }
        writeln!(w)?;
        Ok(())
    }

    fn write_shapes(&mut self, w: &mut impl Write, model: &Model) -> Result<()> {
        for shape in model.shapes() {
            if let Some(doc) = shape.documentation() {
                writeln!(w, "/// {}", doc)?;
            }
            for a_trait in shape.traits() {
                writeln!(w, "@{}", a_trait.id())?;
            }
            match shape.inner() {
                ShapeInner::SimpleType(st) => {
                    writeln!(w, "{} {}", st, shape.id())?;
                }
                ShapeInner::List(_) => {
                    writeln!(w, "list {} {{", shape.id())?;
                    // writeln!(w, "    member: {}", list.member())?;
                    writeln!(w, "}}")?;
                }
                ShapeInner::Set(_) => {}
                ShapeInner::Map(_) => {}
                ShapeInner::Structure(_) => {}
                ShapeInner::Union(_) => {}
                ShapeInner::Service(_) => {
                    writeln!(w, "service {} {{", shape.id())?;
                    //self.write_members(w, shape)?;
                    writeln!(w, "}}")?;
                }
                ShapeInner::Operation(_) => {}
                ShapeInner::Resource(_) => {}
            }
            writeln!(w)?;
        }
        Ok(())
    }

    // fn write_members(&mut self, w: &mut impl Write, shape: &Shape) -> Result<()> {
    //     for (id, member) in shape.members() {
    //         if let Some(doc) = shape.documentation() {
    //             writeln!(w, "    /// {}", doc)?;
    //         }
    //         for a_trait in shape.traits() {
    //             writeln!(w, "    @{}", a_trait.id())?;
    //         }
    //         write!(w, "    {}: {}", id, member.value())?;
    //     }
    //     Ok(())
    // }

    fn write_footer(&mut self, _w: &mut impl Write, _model: &Model) -> Result<()> {
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
