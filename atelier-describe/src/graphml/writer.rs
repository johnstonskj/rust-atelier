/*!
Writes out a model in the [GraphML](http://graphml.graphdrawing.org/index.html) representation form.

For more information see the GraphML [specification](http://graphml.graphdrawing.org/specification.html),
and [primer](http://graphml.graphdrawing.org/primer/graphml-primer.html).

# Mapping

The outer **graphml** and **graph** elements are created as per the specification. For each shape in
the model a GraphML **node** is created, with the following properties:

1. The node's ID is the fully qualified shape ID.
1. The node will have a child **data** element, with key "type", and a value that represents the shape type.
1. **IFF** the shape is a service, the node will have a data value, with key "version", and a value
   which is the service's version string.

A set of **edge**s are also created with the node ID from above as the source:

1. For each trait applied to the shape the target of the edge is the trait's shape ID and the
   edge has a child **data** element, with key "trait", and the value `true`.
1. For each member of an aggregate shape the target of the edge is the member's target shape ID and
   the edge has a child **data** element, with key "member", and a value from `member`, `key`, or
   `value`.
1. For each member of a service shape the target of the edge is the member's target shape ID and
   the edge has a child **data** element, with key "member", where the value is the _member name_
   component of the member's ID.

# Example

For the _message of the day_ model, this writer will generate the following XML.

```xml
<?xml version="1.0" encoding="UTF-8"?>
<graphml xmlns="http://graphml.graphdrawing.org/xmlns"
    xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://graphml.graphdrawing.org/xmlns
        http://graphml.graphdrawing.org/xmlns/1.0/graphml.xsd">
    <key id="smithy-version" for="graph" attr.name="smithy.version" attr.type="string"/>
    <key id="type" for="node" attr.name="type" attr.type="string"/>
    <key id="version" for="node" attr.name="version" attr.type="string"/>
    <key id="trait" for="edge" attr.name="trait" attr.type="boolean"/>
    <key id="member" for="edge" attr.name="member" attr.type="string">
        <default>member</default>
    </key>
    <graph id="model" edgedefault="directed">
        <data key="smithy.version">1.0</data>
        <node id="example.motd#GetMessageOutput">
            <data key="type">structure</data>
        </node>
        <edge id="e0" source="example.motd#GetMessageOutput" target="smithy.api#String">
            <data key="member">message</data>
        </edge>
        <node id="example.motd#BadDateValue">
            <data key="type">structure</data>
        </node>
        <edge id="e1" source="example.motd#BadDateValue" target="smithy.api#error">
            <data key="trait">true</data>
        </edge>
        <edge id="e2" source="example.motd#BadDateValue" target="smithy.api#String">
            <data key="member">errorMessage</data>
        </edge>
        <node id="example.motd#Date">
            <data key="type">string</data>
        </node>
        <edge id="e3" source="example.motd#Date" target="smithy.api#pattern">
            <data key="trait">true</data>
        </edge>
        <node id="example.motd#MessageOfTheDay">
            <data key="type">service</data>
            <data key="version">2020-06-21</data>
        </node>
        <edge id="e4" source="example.motd#MessageOfTheDay" target="smithy.api#documentation">
            <data key="trait">true</data>
        </edge>
        <edge id="e5" source="example.motd#MessageOfTheDay" target="example.motd#Message">
            <data key="member">resources</data>
        </edge>
        <node id="example.motd#Message">
            <data key="type">resource</data>
        </node>
        <edge id="e6" source="example.motd#Message" target="example.motd#GetMessage">
            <data key="member">read</data>
        </edge>
        <node id="example.motd#GetMessage">
            <data key="type">operation</data>
        </node>
        <edge id="e7" source="example.motd#GetMessage" target="smithy.api#readonly">
            <data key="trait">true</data>
        </edge>
        <edge id="e8" source="example.motd#GetMessage" target="example.motd#GetMessageInput">
            <data key="member">input</data>
        </edge>
        <edge id="e9" source="example.motd#GetMessage" target="example.motd#GetMessageInput">
            <data key="member">output</data>
        </edge>
        <edge id="e10" source="example.motd#GetMessage" target="example.motd#BadDateValue">
            <data key="member">errors</data>
        </edge>
        <node id="example.motd#GetMessageInput">
            <data key="type">structure</data>
        </node>
        <edge id="e11" source="example.motd#GetMessageInput" target="example.motd#Date">
            <data key="member">date</data>
        </edge>
    </graph>
</graphml>
```

*/

use atelier_core::error::{Error as ModelError, Result as ModelResult};
use atelier_core::io::ModelWriter;
use atelier_core::model::shapes::{
    AppliedTraits, ListOrSet, Map, Operation, Resource, Service, Simple, StructureOrUnion,
};
use atelier_core::model::visitor::{walk_model_mut, MutableModelVisitor};
use atelier_core::model::{Model, ShapeID};
use atelier_core::syntax::{
    MEMBER_COLLECTION_OPERATIONS, MEMBER_CREATE, MEMBER_DELETE, MEMBER_ERRORS, MEMBER_INPUT,
    MEMBER_KEY, MEMBER_LIST, MEMBER_MEMBER, MEMBER_OPERATIONS, MEMBER_OUTPUT, MEMBER_PUT,
    MEMBER_READ, MEMBER_RESOURCES, MEMBER_UPDATE, MEMBER_VALUE, SHAPE_LIST, SHAPE_MAP,
    SHAPE_OPERATION, SHAPE_RESOURCE, SHAPE_SERVICE, SHAPE_SET, SHAPE_STRUCTURE, SHAPE_UNION,
};
use std::io::Write;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Writes out a model in the [GraphML](http://graphml.graphdrawing.org/index.html) XML form.
///
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub struct GraphMLWriter {}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

struct WriteVisitor<'a, W: Write> {
    edge_count: u16,
    writer: &'a mut W,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for GraphMLWriter {
    fn default() -> Self {
        Self {}
    }
}

impl ModelWriter for GraphMLWriter {
    fn write(&mut self, w: &mut impl Write, model: &Model) -> ModelResult<()> {
        writeln!(w, r#"<?xml version="1.0" encoding="UTF-8"?>"#)?;
        writeln!(
            w,
            r#"<graphml xmlns="http://graphml.graphdrawing.org/xmlns""#
        )?;
        writeln!(
            w,
            r#"    xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance""#
        )?;
        writeln!(
            w,
            r#"    xsi:schemaLocation="http://graphml.graphdrawing.org/xmlns"#
        )?;
        writeln!(
            w,
            r#"        http://graphml.graphdrawing.org/xmlns/1.0/graphml.xsd">"#
        )?;

        self.write_keys(w)?;

        writeln!(w, r#"    <graph id="model" edgedefault="directed">"#)?;
        writeln!(
            w,
            r#"        <data key="smithy.version">{}</data>"#,
            model.smithy_version()
        )?;

        let mut visitor = WriteVisitor {
            edge_count: 0,
            writer: w,
        };
        walk_model_mut(model, &mut visitor)?;
        writeln!(w, r#"    </graph>"#)?;

        writeln!(w, r#"</graphml>"#)?;
        Ok(())
    }
}

impl GraphMLWriter {
    fn write_keys(&mut self, w: &mut impl Write) -> ModelResult<()> {
        // ----------------------------------------------------------------------------------------
        // Graph attributes
        // ----------------------------------------------------------------------------------------
        writeln!(
            w,
            r#"    <key id="smithy-version" for="graph" attr.name="smithy.version" attr.type="string"/>"#
        )?;
        // ----------------------------------------------------------------------------------------
        // Node attributes
        // ----------------------------------------------------------------------------------------
        writeln!(
            w,
            r#"    <key id="type" for="node" attr.name="type" attr.type="string"/>"#
        )?;
        writeln!(
            w,
            r#"    <key id="version" for="node" attr.name="version" attr.type="string"/>"#
        )?;
        // ----------------------------------------------------------------------------------------
        // Edge attributes
        // ----------------------------------------------------------------------------------------
        writeln!(
            w,
            r#"    <key id="trait" for="edge" attr.name="trait" attr.type="boolean"/>"#
        )?;
        writeln!(
            w,
            r#"    <key id="member" for="edge" attr.name="member" attr.type="string">"#
        )?;
        writeln!(w, r#"        <default>member</default>"#)?;
        writeln!(w, r#"    </key>"#)?;
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------

impl<'a, W: Write> MutableModelVisitor for WriteVisitor<'a, W> {
    type Error = ModelError;

    fn simple_shape(
        &mut self,
        id: &ShapeID,
        traits: &AppliedTraits,
        value: &Simple,
    ) -> Result<(), Self::Error> {
        self.node(id, &value.to_string())?;
        self.traits(id, traits)?;
        Ok(())
    }

    fn list(
        &mut self,
        id: &ShapeID,
        traits: &AppliedTraits,
        value: &ListOrSet,
    ) -> Result<(), Self::Error> {
        self.node(id, SHAPE_LIST)?;
        self.traits(id, traits)?;
        self.member(id, value.member().target(), MEMBER_MEMBER)?;
        Ok(())
    }

    fn set(
        &mut self,
        id: &ShapeID,
        traits: &AppliedTraits,
        value: &ListOrSet,
    ) -> Result<(), Self::Error> {
        self.node(id, SHAPE_SET)?;
        self.traits(id, traits)?;
        self.member(id, value.member().target(), MEMBER_MEMBER)?;
        Ok(())
    }

    fn map(
        &mut self,
        id: &ShapeID,
        traits: &AppliedTraits,
        value: &Map,
    ) -> Result<(), Self::Error> {
        self.node(id, SHAPE_MAP)?;
        self.traits(id, traits)?;
        self.member(id, value.key().target(), MEMBER_KEY)?;
        self.member(id, value.value().target(), MEMBER_VALUE)?;
        Ok(())
    }

    fn structure(
        &mut self,
        id: &ShapeID,
        traits: &AppliedTraits,
        value: &StructureOrUnion,
    ) -> Result<(), Self::Error> {
        self.node(id, SHAPE_STRUCTURE)?;
        self.traits(id, traits)?;
        for member in value.members() {
            self.member(id, member.target(), &member.id().to_string())?;
        }
        Ok(())
    }

    fn union(
        &mut self,
        id: &ShapeID,
        traits: &AppliedTraits,
        value: &StructureOrUnion,
    ) -> Result<(), Self::Error> {
        self.node(id, SHAPE_UNION)?;
        self.traits(id, traits)?;
        for member in value.members() {
            self.member(id, member.target(), &member.id().to_string())?;
        }
        Ok(())
    }

    fn operation(
        &mut self,
        id: &ShapeID,
        traits: &AppliedTraits,
        value: &Operation,
    ) -> Result<(), Self::Error> {
        self.node(id, SHAPE_OPERATION)?;
        self.traits(id, traits)?;
        if let Some(target) = value.input() {
            self.member(id, target, MEMBER_INPUT)?;
        }
        if let Some(target) = value.output() {
            self.member(id, target, MEMBER_OUTPUT)?;
        }
        for target in value.errors() {
            self.member(id, target, MEMBER_ERRORS)?;
        }
        Ok(())
    }

    fn service(
        &mut self,
        id: &ShapeID,
        traits: &AppliedTraits,
        value: &Service,
    ) -> Result<(), Self::Error> {
        writeln!(self.writer, r#"        <node id="{}">"#, id)?;
        writeln!(
            self.writer,
            r#"            <data key="type">{}</data>"#,
            SHAPE_SERVICE
        )?;
        writeln!(
            self.writer,
            r#"            <data key="version">{}</data>"#,
            value.version()
        )?;
        writeln!(self.writer, r#"        </node>"#)?;
        self.traits(id, traits)?;
        for target in value.operations() {
            self.member(id, target, MEMBER_OPERATIONS)?;
        }
        for target in value.resources() {
            self.member(id, target, MEMBER_RESOURCES)?;
        }
        Ok(())
    }

    fn resource(
        &mut self,
        id: &ShapeID,
        traits: &AppliedTraits,
        value: &Resource,
    ) -> Result<(), Self::Error> {
        self.node(id, SHAPE_RESOURCE)?;
        self.traits(id, traits)?;
        if let Some(target) = value.create() {
            self.member(id, target, MEMBER_CREATE)?;
        }
        if let Some(target) = value.put() {
            self.member(id, target, MEMBER_PUT)?;
        }
        if let Some(target) = value.read() {
            self.member(id, target, MEMBER_READ)?;
        }
        if let Some(target) = value.update() {
            self.member(id, target, MEMBER_UPDATE)?;
        }
        if let Some(target) = value.delete() {
            self.member(id, target, MEMBER_DELETE)?;
        }
        if let Some(target) = value.list() {
            self.member(id, target, MEMBER_LIST)?;
        }
        for target in value.operations() {
            self.member(id, target, MEMBER_OPERATIONS)?;
        }
        for target in value.collection_operations() {
            self.member(id, target, MEMBER_COLLECTION_OPERATIONS)?;
        }
        for target in value.resources() {
            self.member(id, target, MEMBER_RESOURCES)?;
        }
        Ok(())
    }
}

impl<'a, W: Write> WriteVisitor<'a, W> {
    fn node(&mut self, id: &ShapeID, type_str: &str) -> ModelResult<()> {
        writeln!(self.writer, r#"        <node id="{}">"#, id)?;
        writeln!(
            self.writer,
            r#"            <data key="type">{}</data>"#,
            type_str
        )?;
        writeln!(self.writer, r#"        </node>"#)?;
        Ok(())
    }

    fn member(&mut self, source: &ShapeID, target: &ShapeID, name: &str) -> ModelResult<()> {
        let edge_id = self.edge_id();
        writeln!(
            self.writer,
            r#"        <edge id="{}" source="{}" target="{}">"#,
            edge_id, source, target
        )?;
        writeln!(
            self.writer,
            r#"            <data key="member">{}</data>"#,
            name
        )?;
        writeln!(self.writer, r#"        </edge>"#)?;
        Ok(())
    }

    fn traits(&mut self, id: &ShapeID, traits: &AppliedTraits) -> ModelResult<()> {
        for trait_id in traits.keys() {
            let edge_id = self.edge_id();
            writeln!(
                self.writer,
                r#"        <edge id="{}" source="{}" target="{}">"#,
                edge_id, id, trait_id
            )?;
            writeln!(self.writer, r#"            <data key="trait">true</data>"#)?;
            writeln!(self.writer, r#"        </edge>"#)?;
        }
        Ok(())
    }

    fn edge_id(&mut self) -> String {
        let id = self.edge_count;
        self.edge_count = id + 1;
        format!("e{}", id)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
