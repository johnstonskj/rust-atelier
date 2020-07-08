# Smithy Overview

Smithy is effectively a framework consisting of an abstract model, a custom IDL language, a mapping to/from JSON, and a build process. The abstract model is therefore consistent across different representations while different representations may be used for human usage as well as machine/tool usage. 

## Framework

The following figure demonstrates the framework elements and their relations.

<a name="fig_1_1"></a>![Smithy Conceptual Model](../img/smithy-model-concept.svg)
<div class="caption figure">1.1: Smithy Conceptual Model</div>

* **Model**; is the abstract model, this has no file or format details associated with it, and is the in-memory model used by tools.
* **Representation**; is a particular file format such as the Smithy IDL or JSON AST.
* **Mapping**; is a set of rules that allow for reading and writing a representation. Some representations may not provide a meaningful mapping for read or write.
* **Artifact**; is a file on the file system, in a particular representation. Models may be split across multiple stored artifacts, and those artifacts do not need the same representation.

The build process takes multiple artifacts, validates them, and combines them into a single instance of the abstract model. Files may represent different parts of the IDL for a given application but also their dependencies thus enabling the sharing of common shapes.

### Transformations

The build process mentioned above takes advantage of a number of transformations and while the term process is useful in the build sense the following terms are more specific.

* _model-to-model_; the act of creating one output model from one or more input model, such as a projection to select only certain shapes from the input model(s).
* _model-to-artifact_; the act of creating external representation artifacts, _or_ generating code or infrastructure artifacts.
* _artifact-to-model_; the act of creating a model from one or more artifacts.

The following figure shows that a transform has to have one or more models and may have zero or more artifacts.

<a name="fig_1_2"></a>![Transformations](../img/smithy-model-transforms.svg)
<div class="caption figure">1.2: Transformations</div>


## Abstract Model

The abstract model is not abstract in the Object-Oriented sense, but is abstracted over all potential serialization representations. most tools will only ever deal with the abstract model with serialization and deserialization simply handled by representation mappings.

<a name="fig_1_3"></a>![Abstract Model](../img/smithy-model-model.svg)
<div class="caption figure">1.3: Abstract Model</div>

* **Model**; a container of shapes and optional [metadata](#values).
* **Shape**; a defined thing, shapes are either _simple_, _aggregate_ or _service_ types as described [below](#shapes).
* **Aggregate**; a shape that contains other shapes, such as a `list` of `string`s or an address `structure`.
* **Applied Trait**; [traits](#traits) are a meta feature that allows for values to be associated with shapes. Trait applications  are typically used by tools for validation and code generation.
* **ShapeID**; the identifier for all shapes defined in a model, all members of defined shapes, and the names of all traits.

### Shape ID

The shape identifier is a key element of the abstract model as a representation, such as the Smithy IDL, may simplify typing by the author by allowing the use of relative shape identifiers. All shape identifiers in the abstract model _must be_ absolute identifiers (include a shape name and namespace).

<a name="fig_1_4"></a>![Shape ID](../img/smithy-model-shapeid.svg)
<div class="caption figure">1.4: Shape ID</div>

Shape ID has three query methods, `is_absolute` is true if the id has a namespace; `is_relative` is true if the id _does not_ have a namespace; and `is_member` returns true if the id has a member name. It also has four conversion methods, `to_absolute` returns a new id with the shape name and any member name intact but with the provided namespace; `to_relative` returns a new id with the shape name and any member name intact but any previous namespace is removed; `to_member` returns a new id with the namespace and any shape name intact but with the provided member name; and `to_shape` returns a new id with the namespace and any shape name intact but any previous member name is removed.

## Shapes

Shapes come in three kinds; _simple_, _aggregate_, and _service_. A simple shape represents an atomic or primitive value such as `integer` or `string`. Aggregate values have members such as a `list` of `string`s or an address `structure`. Service shapes have specific semantics, unlike the very generic simple and aggregate shapes, as they represent either a _service_, a _resource_ managed by a service, or _operations_ on services and resources.

<a name="fig_1_5"></a>![Shapes](../img/smithy-model-shapes.svg)
<div class="caption figure">1.5: Shapes</div>

Note that the inheritance relationships in this model are not necessary to implement the abstract model semantics but do make it more understandable.

### Traits

Traits in the Smithy IDL look very much like Java annotations, and fulfill a similar role; _In the Java computer programming language, an annotation is a form of syntactic metadata that can be added to Java source code._ — _Wikipedia_. However, in Java and other programming languages that support annotations these _must_ be added to the declaration of the source element. In contrast, Smithy allows traits to be _applied_ to a shape in a different artifact or different model entirely.

The term _applied trait_ refers to the usage of a trait either directly or indirectly applied to a shape or member. A _trait declaration_ is simply a simple or aggregate shape declaration with the meta-trait `trait` applied to it.

## Values

There are a few places in the abstract model where data values are required, the following demonstrates the values supported by the model. 

<a name="fig_1_6"></a>![Data](../img/smithy-model-data.svg)
<div class="caption figure">1.6: Data</div>

* **metadata**; every Model has an optional metadata `Object` which is often used to store tool or process specific values.
* **nodeValue**; a trait application has values for the structure that defines the trait itself.
* **container**/**members**; an aggregate value, such as `Array` or `Object`, is a _container_ of _members_ or member values.
 