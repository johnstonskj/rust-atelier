/*!
Writer to produce [PlantUML](https://plantuml.com/) text files for diagramming.

# Mapping

The mapping is reasonably simple, but also effective.

1. The namespace for the model becomes the name of a package enclosing all model elements.
1. Each service is a UML class with the `«service»` stereotype, and
   1. the `version` number is modeled as a field, with a value,
   1. the `operations` are modeled as UML operations, with the input and output copied from the
      operation shape, although errors are added as classes with the `«error»` stereotype,
   1. the `resources` are shown as separate classes (see next), with an aggregate relationship
      from service to resource.
1. Each resource is a UML class with the `«resource»` stereotype, and
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

# Example

For the _message of the day_ model, this writer will generate the following text.

```text
@startuml

hide empty members

package smithy.api {
    class blob <<primitive>> { }
    class boolean <<primitive>> { }
    class document <<primitive>> { }
    class string <<primitive>> { }
    class byte <<primitive>> { }
    class short <<primitive>> { }
    class integer <<primitive>> { }
    class long <<primitive>> { }
    class float <<primitive>> { }
    class double <<primitive>> { }
    class bigInteger <<primitive>> { }
    class bigDecimal <<primitive>> { }
    class timestamp <<primitive>> { }
}

package example.motd {

    class BadDateValue <<error>> {
        errorMessage: String
    }

    class GetMessageInput {
        date: string
    }

    class MessageOfTheDay <<service>> {
        version: string = "2020-06-21"
    }
    note "Provides a Message of the day." as MessageOfTheDay_note_0
    MessageOfTheDay .. MessageOfTheDay_note_0
    MessageOfTheDay *-- Message

    class Message <<resource>> {
        date: string
        create(in: GetMessageInput): GetMessageOutput
    }

    class Date <<dataType>> {
        @pattern = "^\d\d\d\d\-\d\d-\d\d$"
        ..
    }

    smithy.api.string <|-- Date

    class GetMessageOutput {
        message: String
    }

}
example.motd ..> smithy.api

@enduml
```

Which would produce an image like the following.

![PlantUML](https://raw.githubusercontent.com/johnstonskj/rust-atelier/master/atelier-lib/doc/motd-model.png)

*/

use crate::core::io::ModelWriter;
use crate::core::model::shapes::{Shape, ShapeKind, TopLevelShape};
use crate::core::model::values::Value;
use crate::core::model::{Model, NamespaceID, ShapeID};
use crate::core::prelude::PRELUDE_NAMESPACE;
use crate::core::syntax::{
    MEMBER_COLLECTION_OPERATIONS, MEMBER_OPERATIONS, MEMBER_VERSION, SHAPE_BIG_DECIMAL,
    SHAPE_BIG_INTEGER, SHAPE_BLOB, SHAPE_BOOLEAN, SHAPE_BYTE, SHAPE_DOCUMENT, SHAPE_DOUBLE,
    SHAPE_FLOAT, SHAPE_INTEGER, SHAPE_LONG, SHAPE_RESOURCE, SHAPE_SERVICE, SHAPE_SHORT,
    SHAPE_STRING, SHAPE_TIMESTAMP, SHAPE_UNION,
};
use atelier_core::model::shapes::HasTraits;
use atelier_core::model::HasIdentity;
use std::collections::HashSet;
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

///
/// The extension to use when reading from, or writing to, files of this type.
///
pub const FILE_EXTENSION: &str = "uml";

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

impl ModelWriter for PlantUmlWriter {
    fn write(&mut self, w: &mut impl Write, model: &Model) -> crate::core::error::Result<()> {
        writeln!(w, "@startuml")?;
        writeln!(w)?;
        writeln!(w, "hide empty members")?;
        writeln!(w)?;
        if self.expand_smithy_api {
            self.write_smithy_model(w)?;
        }
        let namespaces: HashSet<&NamespaceID> =
            model.shape_names().map(ShapeID::namespace).collect();
        for namespace in namespaces {
            writeln!(w, "package {} {{", namespace)?;
            writeln!(w)?;
            for element in model.shapes() {
                match element.body() {
                    ShapeKind::Simple(_) => self.write_data_type(w, element, model)?,
                    ShapeKind::Service(_) => self.write_service(w, element, model)?,
                    ShapeKind::Resource(_) => self.write_resource(w, element, model)?,
                    ShapeKind::Structure(_) | ShapeKind::Union(_) => {
                        self.write_class(w, element, model)?
                    }
                    _ => {}
                }
            }
            writeln!(w, "}}")?;
            if self.expand_smithy_api {
                writeln!(w, "{} ..> smithy.api", namespace)?;
            }
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

    fn write_smithy_model(&self, w: &mut impl Write) -> crate::core::error::Result<()> {
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
        service: &TopLevelShape,
        model: &Model,
    ) -> crate::core::error::Result<()> {
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
        resource: &TopLevelShape,
        model: &Model,
    ) -> crate::core::error::Result<()> {
        writeln!(w, "    class {} <<{}>> {{", resource.id(), SHAPE_RESOURCE)?;
        let notes = self.write_class_traits(w, resource, model)?;
        let body = resource.body().as_resource().unwrap();
        for (id, shape_id) in body.identifiers() {
            writeln!(w, "        {}: {}", id, shape_id)?;
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
        structure: &TopLevelShape,
        model: &Model,
    ) -> crate::core::error::Result<()> {
        let (is_union, body) = match structure.body() {
            ShapeKind::Structure(s) => (false, s),
            ShapeKind::Union(s) => (true, s),
            _ => unreachable!(),
        };
        if structure.has_trait(&ShapeID::from_str("smithy.api#trait").unwrap()) {
            writeln!(w, "    annotation {} {{", structure.id())?;
        } else if structure.has_trait(&ShapeID::from_str("smithy.api#error").unwrap()) {
            writeln!(w, "    class {} <<error>> {{", structure.id())?;
        } else if is_union {
            writeln!(w, "    class {} <<{}>> {{", structure.id(), SHAPE_UNION)?;
        } else {
            writeln!(w, "    class {} {{", structure.id())?;
        }
        let notes = self.write_class_traits(w, structure, model)?;
        for member in body.members() {
            if member.is_member() {
                writeln!(w, "        {}: {}", member.id(), member.target())?;
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
        data_type: &TopLevelShape,
        model: &Model,
    ) -> crate::core::error::Result<()> {
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
    ) -> crate::core::error::Result<()> {
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
                self.type_string(&operation.input().as_ref().unwrap(), model)
            } else {
                String::new()
            },
            if operation.has_output() {
                format!(
                    ": {}",
                    self.type_string(&operation.output().as_ref().unwrap(), model)
                )
            } else {
                String::new()
            },
        )?;
        Ok(())
    }

    fn write_class_traits(
        &self,
        w: &mut impl Write,
        shape: &TopLevelShape,
        _model: &Model,
    ) -> crate::core::error::Result<Vec<String>> {
        let mut traits: Vec<String> = Default::default();
        let mut notes: Vec<String> = Default::default();
        for a_trait in shape.traits() {
            if a_trait.id() == &ShapeID::from_str("smithy.api#error").unwrap() {
                // ignore
            } else if a_trait.id() == &ShapeID::from_str("smithy.api#documentation").unwrap() {
                if let Some(Value::String(s)) = a_trait.value() {
                    notes.push(s.clone())
                }
            } else {
                traits.push(match a_trait.value() {
                    None | Some(Value::None) => format!("    @{}", a_trait.id()),
                    Some(Value::String(v)) => format!("    @{} = \"{}\"", a_trait.id(), v),
                    Some(Value::Number(v)) => format!("    @{} = {}", a_trait.id(), v),
                    Some(Value::Boolean(v)) => format!("    @{} = {}", a_trait.id(), v),
                    Some(Value::Array(_)) => format!("    @{} = [ .. ]", a_trait.id()),
                    Some(Value::Object(_)) => format!("    @{} = {{ .. }}", a_trait.id()),
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
    ) -> crate::core::error::Result<()> {
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
                ShapeKind::Simple(st) => st.to_string(),
                ShapeKind::List(list) => format!("List<{}>", list.member().target().shape_name()),
                ShapeKind::Set(set) => format!("Set<{}>", set.member().target().shape_name()),
                ShapeKind::Map(map) => format!(
                    "Map<{}, {}>",
                    map.key().target().shape_name(),
                    map.value().target().shape_name()
                ),
                _ => type_id.to_string(),
            }
        } else {
            type_id.to_string()
        }
    }
}
