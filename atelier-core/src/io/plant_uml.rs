/*!
Writer to produce [PlantUML](https://plantuml.com/) text files for diagramming.

The mapping is reasonably simple, but also effective.

1. The namespace for the model becomes the name of a package enclosing all model elements.
1. Each service is a UML class with the `«service»` stereotype, and
   1. the `version` number is modeled as a field, with a value,
   1. the `operations` are modeled as UML operations, with the input and output copied from the
      operation shape, although errors are added as classes with the `«error»` stereotype,
   1. the `resources` are shown as separate classes (see next), with an aggregate relationship
      from service to resource.
1. Each resource is a UML class with the `«resourcer»` stereotype, and
   1. the `identifiers` map is modeled as individual fields on the class,
   1. the lifecycle methods are shown as UML operations, but they use the lifecycle name, and not
      the name of the operation shape,
   1. the operations and collection_operations are modeled as UML operations, with the input and
      output copied from the operation shape, although errors are added as classes with the
      `«error»` stereotype,
   1. the `resources` are shown as separate classes (see next), with an aggregate relationship
      from service to resource.
1. Each structure and union shape is modeled as a class, with a stereotype that is added, in order
   of precedence:  `«error»` if it has the error trait applied, `«union»` if it is a union shape
   or none,
   1. members of the shape are modeled as UML fields on the class.
1. Each simple shape is modeled as a `«dataType»` class,
   1. `member` for list and set as well as the `key` and `value` for a map are modeled as
      association relationships to the relevant types,
   1. the base type for the shape (string, list, etc.) is modeled as an inheritance relationship
      from the new type to a type in the Smithy model with the stereotype `«primitive»`
1. Traits on shapes are shown as UML fields, traits on members are not shown.
1. Note that the `error` trait is not shown in this way as it is used as a stereotype for those
   shapes that have it applied.
1. The `documentation` trait is also not shown in this way as it each becomes a note related to
   the shape.
*/

use crate::io::ModelWriter;
use crate::model::shapes::{Shape, ShapeBody, Valued};
use crate::model::values::NodeValue;
use crate::model::{Annotated, Model, Named, ShapeID};
use crate::prelude::PRELUDE_NAMESPACE;
use crate::syntax::{
    MEMBER_COLLECTION_OPERATIONS, MEMBER_OPERATIONS, MEMBER_VERSION, SHAPE_BIG_DECIMAL,
    SHAPE_BIG_INTEGER, SHAPE_BLOB, SHAPE_BOOLEAN, SHAPE_BYTE, SHAPE_DOCUMENT, SHAPE_DOUBLE,
    SHAPE_FLOAT, SHAPE_INTEGER, SHAPE_LONG, SHAPE_RESOURCE, SHAPE_SERVICE, SHAPE_SHORT,
    SHAPE_STRING, SHAPE_TIMESTAMP, SHAPE_UNION,
};
use std::io::Write;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This writer produces textual output supported by the [PlantUML](https://plantuml.com/) tools.
/// This is a relatively simple (and incomplete) mapping used to provide a useful overview diagram.
///
#[derive(Debug)]
pub struct PlantUmlWriter {
    expand_smithy_api: bool,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for PlantUmlWriter {
    fn default() -> Self {
        Self {
            expand_smithy_api: false,
        }
    }
}

impl<'a> ModelWriter<'a> for PlantUmlWriter {
    const REPRESENTATION: &'static str = "PlantUML";

    fn write(&mut self, w: &mut impl Write, model: &'a Model) -> crate::error::Result<()> {
        writeln!(w, "@startuml")?;
        writeln!(w)?;
        writeln!(w, "hide empty members")?;
        writeln!(w)?;
        if self.expand_smithy_api {
            self.write_smithy_model(w)?;
        }
        writeln!(w, "package {} {{", model.namespace())?;
        writeln!(w)?;
        for element in model.shapes() {
            match element.body() {
                ShapeBody::SimpleType(_) => self.write_data_type(w, element, model)?,
                ShapeBody::Service(_) => self.write_service(w, element, model)?,
                ShapeBody::Resource(_) => self.write_resource(w, element, model)?,
                ShapeBody::Structure(_) | ShapeBody::Union(_) => {
                    self.write_class(w, element, model)?
                }
                _ => {}
            }
        }
        writeln!(w, "}}")?;
        if self.expand_smithy_api {
            writeln!(w, "{} ..> smithy.api", model.namespace())?;
        }
        writeln!(w)?;
        writeln!(w, "@enduml")?;
        Ok(())
    }
}

impl PlantUmlWriter {
    ///
    /// Construct a new writer, denoting whether you want to also visualize the Smithy API model
    /// as well. The default behavior is _not_ to expand this model.
    ///
    pub fn new(expand_smithy_api: bool) -> Self {
        Self { expand_smithy_api }
    }

    fn write_smithy_model(&self, w: &mut impl Write) -> crate::error::Result<()> {
        writeln!(w, "package {} {{", PRELUDE_NAMESPACE)?;
        for name in &[
            SHAPE_BLOB,
            SHAPE_BOOLEAN,
            SHAPE_DOCUMENT,
            SHAPE_STRING,
            SHAPE_BYTE,
            SHAPE_SHORT,
            SHAPE_INTEGER,
            SHAPE_LONG,
            SHAPE_FLOAT,
            SHAPE_DOUBLE,
            SHAPE_BIG_INTEGER,
            SHAPE_BIG_DECIMAL,
            SHAPE_TIMESTAMP,
        ] {
            writeln!(w, "    class {} <<primitive>> {{ }}", name)?;
        }
        writeln!(w, "}}")?;
        writeln!(w)?;
        Ok(())
    }

    fn write_service(
        &self,
        w: &mut impl Write,
        service: &Shape,
        model: &Model,
    ) -> crate::error::Result<()> {
        writeln!(w, "    class {} <<{}>> {{", service.id(), SHAPE_SERVICE)?;
        let notes = self.write_class_traits(w, service, model)?;
        let body = service.body().as_service().unwrap();
        writeln!(
            w,
            "        {}: {} = \"{}\"",
            MEMBER_VERSION,
            SHAPE_STRING,
            body.version()
        )?;
        for operation in body.operations() {
            self.write_operation(w, operation, model, None)?;
        }
        writeln!(w, "    }}")?;
        self.write_class_notes(w, service.id(), notes)?;
        for resource in body.resources() {
            writeln!(w, "    {} *-- {}", service.id(), resource)?;
        }
        writeln!(w)?;
        Ok(())
    }

    fn write_resource(
        &self,
        w: &mut impl Write,
        resource: &Shape,
        model: &Model,
    ) -> crate::error::Result<()> {
        writeln!(w, "    class {} <<{}>> {{", resource.id(), SHAPE_RESOURCE)?;
        let notes = self.write_class_traits(w, resource, model)?;
        let body = resource.body().as_resource().unwrap();
        for (id, shape_id) in body.identifiers() {
            writeln!(w, "        {}: {}", id, self.type_string(shape_id, model))?;
        }
        if let Some(id) = body.create() {
            self.write_operation(w, id, model, Some("create"))?;
        }
        if let Some(id) = body.put() {
            self.write_operation(w, id, model, Some("create"))?;
        }
        if let Some(id) = body.read() {
            self.write_operation(w, id, model, Some("create"))?;
        }
        if let Some(id) = body.update() {
            self.write_operation(w, id, model, Some("create"))?;
        }
        if let Some(id) = body.delete() {
            self.write_operation(w, id, model, Some("create"))?;
        }
        if let Some(id) = body.list() {
            self.write_operation(w, id, model, Some("create"))?;
        }
        if body.has_operations() {
            writeln!(w, "        ..{}..", MEMBER_OPERATIONS)?;
            for operation in body.operations() {
                self.write_operation(w, operation, model, None)?;
            }
        }
        if body.has_collection_operations() {
            writeln!(w, "        ..{}..", MEMBER_COLLECTION_OPERATIONS)?;
            for operation in body.collection_operations() {
                self.write_operation(w, operation, model, None)?;
            }
        }
        writeln!(w, "    }}")?;
        self.write_class_notes(w, resource.id(), notes)?;
        for other in body.resources() {
            writeln!(w, "    {} *-- {}", resource.id(), other)?;
        }
        writeln!(w)?;
        Ok(())
    }

    fn write_class(
        &self,
        w: &mut impl Write,
        structure: &Shape,
        model: &Model,
    ) -> crate::error::Result<()> {
        let (is_union, body) = match structure.body() {
            ShapeBody::Structure(s) => (false, s),
            ShapeBody::Union(s) => (true, s),
            _ => unreachable!(),
        };
        if structure.has_trait(&ShapeID::from_str("trait").unwrap()) {
            writeln!(w, "    annotation {} {{", structure.id())?;
        } else if structure.has_trait(&ShapeID::from_str("error").unwrap()) {
            writeln!(w, "    class {} <<error>> {{", structure.id())?;
        } else if is_union {
            writeln!(w, "    class {} <<{}>> {{", structure.id(), SHAPE_UNION)?;
        } else {
            writeln!(w, "    class {} {{", structure.id())?;
        }
        let notes = self.write_class_traits(w, structure, model)?;
        for member in body.members() {
            if let Some(NodeValue::ShapeID(shape_id)) = member.value() {
                writeln!(
                    w,
                    "        {}: {}",
                    member.id(),
                    self.type_string(shape_id, model)
                )?;
            } else {
                unreachable!()
            }
        }
        writeln!(w, "    }}")?;
        self.write_class_notes(w, structure.id(), notes)?;
        writeln!(w)?;
        Ok(())
    }

    fn write_data_type(
        &self,
        w: &mut impl Write,
        data_type: &Shape,
        model: &Model,
    ) -> crate::error::Result<()> {
        writeln!(w, "    class {} <<dataType>> {{", data_type.id())?;
        let notes = self.write_class_traits(w, data_type, model)?;
        writeln!(w, "    }}")?;
        writeln!(w, "    {}", notes.join("\n"))?;
        if self.expand_smithy_api {
            let simple = data_type.body().as_simple().unwrap();
            writeln!(
                w,
                "    {}.{} <|-- {}",
                PRELUDE_NAMESPACE,
                simple,
                data_type.id()
            )?;
        }
        writeln!(w)?;
        Ok(())
    }

    fn write_operation(
        &self,
        w: &mut impl Write,
        oper_id: &ShapeID,
        model: &Model,
        other_name: Option<&str>,
    ) -> crate::error::Result<()> {
        let operation = model.shape(oper_id).unwrap();
        let name = operation.id();
        let operation = operation.body().as_operation().unwrap();
        writeln!(
            w,
            "        {}(in: {}){}",
            match other_name {
                None => name.to_string(),
                Some(name) => name.to_string(),
            },
            if operation.has_input() {
                self.type_string(operation.input().unwrap(), model)
            } else {
                String::new()
            },
            if operation.has_output() {
                format!(": {}", self.type_string(operation.output().unwrap(), model))
            } else {
                String::new()
            },
        )?;
        Ok(())
    }

    fn write_class_traits(
        &self,
        w: &mut impl Write,
        shape: &Shape,
        _model: &Model,
    ) -> crate::error::Result<Vec<String>> {
        let mut traits: Vec<String> = Default::default();
        let mut notes: Vec<String> = Default::default();
        for a_trait in shape.traits() {
            if a_trait.id() == &ShapeID::from_str("error").unwrap() {
                // ignore
            } else if a_trait.id() == &ShapeID::from_str("documentation").unwrap() {
                if let Some(NodeValue::String(s)) = a_trait.value() {
                    notes.push(s.clone())
                }
            } else {
                traits.push(match a_trait.value() {
                    None | Some(NodeValue::None) => format!("    @{}", a_trait.id()),
                    Some(NodeValue::String(v)) => format!("    @{} = \"{}\"", a_trait.id(), v),
                    Some(NodeValue::TextBlock(v)) => {
                        format!("    @{} = \"\"\"{}\"\"\"", a_trait.id(), v)
                    }
                    Some(NodeValue::Number(v)) => format!("    @{} = {}", a_trait.id(), v),
                    Some(NodeValue::ShapeID(v)) => format!("    @{} = {}", a_trait.id(), v),
                    Some(NodeValue::Boolean(v)) => format!("    @{} = {}", a_trait.id(), v),
                    Some(NodeValue::Array(_)) => format!("    @{} = [ .. ]", a_trait.id()),
                    Some(NodeValue::Object(_)) => format!("    @{} = {{ .. }}", a_trait.id()),
                });
            }
        }
        if !traits.is_empty() {
            writeln!(w, "    {}", traits.join("\n"))?;
            writeln!(w, "        ..")?;
        }
        Ok(notes)
    }

    fn write_class_notes(
        &self,
        w: &mut impl Write,
        shape_id: &ShapeID,
        notes: Vec<String>,
    ) -> crate::error::Result<()> {
        for (idx, note) in notes.iter().enumerate() {
            if note.contains('\n') {
                writeln!(w, "    note as {}_note_{}", shape_id, idx)?;
                writeln!(w, "{:?}", note)?;
                writeln!(w, "    end note")?;
            } else {
                writeln!(w, "    note {:?} as {}_note_{}", note, shape_id, idx)?;
            }
            writeln!(w, "    {} .. {}_note_{}", shape_id, shape_id, idx)?;
        }
        Ok(())
    }

    fn type_string(&self, type_id: &ShapeID, model: &Model) -> String {
        if let Some(shape) = model.shape(type_id) {
            match shape.body() {
                ShapeBody::SimpleType(st) => st.to_string(),
                ShapeBody::List(list) => format!("List<{}>", list.member()),
                ShapeBody::Set(set) => format!("Set<{}>", set.member()),
                ShapeBody::Map(map) => format!("Map<{}, {}>", map.key(), map.value()),
                _ => type_id.to_string(),
            }
        } else {
            type_id.to_string()
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::error::ErrorSource;
    use crate::io::plant_uml::PlantUmlWriter;
    use crate::io::write_model_to_string;
    use crate::model::builder::{
        MemberBuilder, ModelBuilder, OperationBuilder, ResourceBuilder, ServiceBuilder,
        ShapeBuilder, SimpleShapeBuilder, StructureBuilder, TraitBuilder,
    };
    use crate::model::Model;
    use crate::Version;

    fn make_example_model() -> Model {
        let model: Model = ModelBuilder::new("example.motd", Some(Version::V10))
            .shape(
                ServiceBuilder::new("MessageOfTheDay")
                    .documentation("Provides a Message of the day.")
                    .version("2020-06-21")
                    .resource("Message")
                    .into(),
            )
            .shape(
                ResourceBuilder::new("Message")
                    .identifier("date", "Date")
                    .read("GetMessage")
                    .into(),
            )
            .shape(
                SimpleShapeBuilder::string("Date")
                    .add_trait(TraitBuilder::pattern(r"^\d\d\d\d\-\d\d-\d\d$").into())
                    .into(),
            )
            .shape(
                OperationBuilder::new("GetMessage")
                    .readonly()
                    .input("GetMessageInput")
                    .output("GetMessageOutput")
                    .error("BadDateValue")
                    .into(),
            )
            .shape(
                StructureBuilder::new("GetMessageInput")
                    .add_member(MemberBuilder::new("date").refers_to("Date").into())
                    .into(),
            )
            .shape(
                StructureBuilder::new("GetMessageOutput")
                    .add_member(MemberBuilder::string("message").required().into())
                    .into(),
            )
            .shape(
                StructureBuilder::new("BadDateValue")
                    .error(ErrorSource::Client)
                    .add_member(MemberBuilder::string("errorMessage").required().into())
                    .into(),
            )
            .into();
        model
    }

    #[test]
    fn test_uml_writer() {
        let model = make_example_model();
        let mut writer = PlantUmlWriter::new(true);
        let output = write_model_to_string(&mut writer, &model);
        assert!(output.is_ok());
        println!("{}", output.unwrap())
    }
}
