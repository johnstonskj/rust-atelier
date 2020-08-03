# Appendix: RDF Mapping

This provides a brief description of the Model to RDF mapping; the qualified names in the examples
below use the prefix "smithy" which is defined in [`vocabulary::PREFIX`](../vocabulary/constant.PREFIX.html)
and which maps to the namespace IRI in [`vocabulary::NAMESPACE`](../vocabulary/constant.NAMESPACE.html).

These values are set in the examples below in [Turtle](https://www.w3.org/TR/turtle/) syntax as a
common preamble:

```turtle
@prefix smithy: <https://awslabs.github.io/smithy/vocab/1.0#> .
@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
```

Note the inclusion of the `xsd` namespace for literals.

## Model

1. Each model MUST have a subject, either a provided IRI or a blank node will be created.
1. This subject MUST have an `rdf:type` of `smithy:Model`.
1. This subject MAY have a relationship, typed as `smithy:shapes` to a node with `rdf:type` of
   `rdf:Bag`. This relationship may be omitted if the model contains no shapes.

```turtle
_:subject a smithy:Model ;
            smithy:shapes _:shapes .

_:shapes a rdf:Bag .
```

## Shape

1. Each shape MUST be present as a member of the `smithy:shapes` bag introduced above.
1. The identifier is the URN form of the shapes **shape ID**.
1. The shape MUST include an `rdf:type` statement that denotes it's Smithy type.
1. Additional requirements are type specific and introduced below.

```turtle
_:shapes rdf:li <urn:smithy:example.motd:Shape> .

<urn:smithy:example.motd:Shape> a smithy:String .
```

1. Simple shapes;
   1. no additional rules.
1. List and Set shapes;
   1. An additional statement for the shape MUST be present with the predicate `smithy:member`
      and the object being the URN of the target shape.
   1. This member MAY have traits (see below).
1. Map shapes;
   1. An additional statement for the shape MUST be present with the predicate `smithy:key`
      and the object being the URN of the target shape.
   1. An additional statement for the shape MUST be present with the predicate `smithy:value`
      and the object being the URN of the target shape.
   1. These members MAY have traits (see below).
1. Structure and Union shapes;
   1. Each member of the shape becomes a statement with the shape ID as predicate and the object
      being a URN for the target shape.
   1. These members MAY have traits (see below).
1. Service shapes;
   1. An additional statement for the shape MUST be present with the predicate `smithy:version` and
      the object being a literal, non-empty, string.
   1. Each member of the shape becomes a statement with the corresponding predicate `smithy:*`
      and the object being the URN of the target shape.
   1. For the multi-valued members `operations`, and `resources`, the statement SHALL be repeated
      once for each value.
1. Operation shapes;
   1. Each member of the shape becomes a statement with the corresponding predicate `smithy:*`
      and the object being the URN of the target shape.
   1. For the multi-valued member `errors` the statement SHALL be repeated once for each value.
1. Resource Shapes;
   1. The resource subject MAY have a relationship, typed as `smithy:identifiers` to a node with
      `rdf:type` of `rdf:Bag`. This relationship may be omitted if the model contains no identifier
      pairs.
      1. Each identifier pair consists of a blank node in the bag with two statements;
         1. one with the predicate `smithy:key` and the object being a literal string for the identifier name,
         1. one with the predicate `smithy:value` and the object being the URN of the target shape.
   1. Each member of the shape becomes a statement with the corresponding predicate `smithy:*`
      and the object being the URN of the target shape.
   1. For the multi-valued members `operations`, `collectionOperations`, and `resources`, the
      statement SHALL be repeated once for each value.

## Traits

Any shape, either a top-level shape, or a member, may have traits applied, these are represented as
follows:

1. This shape MAY have a relationship, typed as `smithy:traits` to a node with `rdf:type` of
   `rdf:Bag`. This relationship may be omitted if the shape has no applied traits.
1. Each applied trait is represented as a blank node, with predicate `rdf:li` in the trait bag.
1. This new node MUST include a statement with the predicate `smithy:trait` and object being the
   URN of the trait shape.
1. The new node MAY include a statement with the predicate `smithy:value` and object being the
   value applied with this shape (see production below).

```turtle
<urn:smithy:example.motd:Shape> a smithy:String ;
            smithy:traits _:shape_traits .

_:shape_traits a rdf:Bag ;
            rdf:li _:a_trait .

_:a_trait smithy:trait <urn:smithy:smithy.api:required> .
```

## Values

Values are attached to a node with the predicate `smithy:value` and the value represented as follows:

1. string values MUST be represented as unqualified string literals,
1. boolean values MUST be represented as string literals with the type `xsd:boolean`,
1. numeric values MUST be represented as string literals with either the type `xsd:signedLong` or
   `xsd:double`.
1. null values MUST be represented as `rdf:nil`,
1. array values MUST be represented as a new blank node,
   1. this node MUST have a statement with `rdf:type` of `rdf:List`,
   1. each element in the array occurs in this list with the predicate `rdf:li` and object being
      the value represented using these same production rules,
1. object values MUST be represented as a new blank node,
   1. this node MUST have a statement with `rdf:type` of `rdf:Bag`,
   1. each element in the object occurs in this list with the predicate `rdf:li` and object being
      a new node blank node,
   1. this node MUST have a statement with `smithy:key` and the object being a string literal
      for the identifier name,
   1. this node MUST have a statement with `smithy:value` and the object being the URN of the
      target shape.

```turtle
_:a_trait smithy:trait <urn:smithy:smithy.api:documentation> ;
            smithy:value "Here is some documentation".
```
