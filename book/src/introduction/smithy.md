# Smithy Overview

Smithy is effectively a framework consisting of an abstract model, a custom IDL language, a mapping to/from JSON, and a build process. The abstract model is therefore consistent across different representations while different representations may be used for human usage as well as machine/tool usage. 

## Framework

The following figure demonstrates the framework elements and their relations.

<a name="fig_1_1"></a>![Smithy Conceptual Model](../img/smithy-model-concept.svg)
<div class="caption figure">1.1: Smithy Conceptual Model</div>

* **Model**; is the abstract model, this has no file or format details associated with it, and is the in-memory model used by tools.
* **Representation**; is a particular file format such as the Smithy IDL or JSON AST.
* **Mapping**; is a set of rules that allow for reading and writing a representation. Some representations may only support read _or_ write and not both.
* **File**; is an individual file on the file system, in a particular representation. Models may be split across multiple files, and those files do not need the same representation.

The build process takes multiple files, validate them, and combine them into a single instance of the abstract model. Files may represent different parts of the IDL for a given application but also their dependencies thus enabling the sharing of common shapes.

## Abstract Model

The abstract model is not abstract in the Object-Oriented sense, but is abstracted over all potential serialization representations. most tools will only ever deal with the abstract model with serialization and deserialization simply handled by representation mappings.

<a name="fig_1_2"></a>![Abstract Model](../img/smithy-model-model.svg)
<div class="caption figure">1.2: Abstract Model</div>

* **Model**; a container of shapes and optional metadata.
* **Shape**; a defined thing, shapes are either simple, aggregate or service types as described below.
* **Member**; a field or property of an aggregate or service shape.
* **Trait**; a meta feature that allows for values to be associated with shapes that are typically used by tools for validation and code generation.
* **ShapeID**; the identifier for all shapes defined in a model, all members of defined shapes, and the names of all traits.

### Shape ID

The shape identifier is a key element of the abstract model as a representation such as the Smithy IDL simplify typing by the author by allowing the use of relative shape identifiers. All shape identifiers in a built abstract model _must be_ absolute identifiers (include a shape name and namespace).

<a name="fig_1_3"></a>![Shape ID](../img/smithy-model-shapeid.svg)
<div class="caption figure">1.3: Shape ID</div>

Shape ID has three query methods, `is_absolute` is true if the id has a namespace; `is_relative` is true if the id _does not_ have a namespace; and `is_member` returns true if the id has a member name. It also has four conversion methods, `to_absolute` returns a new id with the shape name and any member name intact but with the provided namespace; `to_relative` returns a new id with the shape name and any member name intact but any previous namespace is removed; `to_member` returns a new id with the namespace and any shape name intact but with the provided member name; and `to_shape` returns a new id with the namespace and any shape name intact but any previous member name is removed.

## Shapes

Shapes come in three kinds; simple, aggregate, and service. A simple shape represents an atomic or primitive value such as `integer` or `string`. Aggregate values have members such as a `list` of `string`s or an address `structure`. Service values also have members like an aggregate type, however unlike the very generic simple and aggregate shapes these have additional semantics as they represent a _service_, a _resource_ managed by a service, and _operations_ on services and resources.

<a name="fig_1_4"></a>![Shapes](../img/smithy-model-shapes.svg)
<div class="caption figure">1.4: Shapes</div>

Note that the inheritance relationships in this model are not necessary to implement the abstract model semantics but do make it more understandable.

## Values

There are a few places in the abstract model where data values are required, the following demonstrates the values supported by the model. 

<a name="fig_1_5"></a>![Data](../img/smithy-model-data.svg)
<div class="caption figure">1.5: Data</div>

Every Model has an optional metadata `Object` which is often used to store tool or process specific values. Also, a trait application has values for the structure that defines the trait itself.