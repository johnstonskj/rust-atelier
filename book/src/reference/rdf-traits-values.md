# Traits and Values


## Traits

Any shape, either a *top-level*, or a *member*, may have traits applied and these are represented as
follows.

1. The shape resource MAY have any number of properties, named `smithy:apply` with a blank node value.
    1. This blank node MUST have a property `smithy:trait` with a value which is the identifier
       of a *top-level* shape resource.
    1. This blank node MAY have a property `smithy:value` representing the trait parameter value -
       see [Value](#values) production rules later.

The following example shows traits applied to a *top-level* shape.

```turtle
motd:BadDateValue
  a smithy:Structure ;
  smithy:apply [
    smithy:trait api:error ;
    smithy:value "client"
  ] .
```

The following example shows traits applied to a *member* shape.

```turtle
<urn:smithy:example.motd:BadDateValue/errorMessage> 
  a smithy:Member ;
  smithy:target smithy:String ;
  smithy:apply [
    smithy:trait api:required
  ] .
```

## Values

Values are are used in both the model metadata section,

```turtle
smithy:metadata [
  a rdf:Bag
  rdf:_1 [
    smithy:key "domain" ;
    smithy:value "identity" ;
  ]
] .
```

as well as in passing parameters to applied traits.

```turtle
smithy:apply [
  smithy:trait api:title ;
  smithy:value "My new thing"
] .
```

The following define the production rules for values in either of these cases.

### Strings

String values MAY be represented as unqualified string literals OR as qualified strings with the data type `xsd:string`.

```turtle
[] smithy:value "My new thing" .

[] smithy:value "My new thing"^^xsd:string .
```

### Booleans

Boolean values MUST be represented as string literals with the type `xsd:boolean`.

```turtle
[] smithy:value "true"^^xsd:boolean .

# alternatively, in Turtle:

[] smithy:value true .
```

### Numbers

Number values MUST be represented as string literals with either the type `xsd:signedLong` or
   `xsd:double`.

```turtle
[] smithy:value "1"^^xsd:signedLong .

[] smithy:value "3.14"^^xsd:double" .
```

### Arrays

Array values MUST be represented as a new blank node.

1. This node MUST have a property `rdf:type` with the IRI value `rdf:Seq`.
1. Each property of this blank node follows the standard method to generate predicate names of the 
   form `rdf:_{n}` with a *value* using these same production rules.

```turtle
smithy:value [
  a rdf:Seq ;
  rdf:_1 "experimental" ;
  rdf:_2 "public"
]
```

### Objects

Object values MUST be represented as a new blank node.

1. This node MUST have a property `rdf:type` with the IRI value `rdf:Bag`.
1. Each property of this blank node follows the standard method to generate predicate names of the
   form`rdf:_{n}` with a blank node value.
    1. This node MUST have a property `smithy:key` with a string literal for the identifier name.
    1. This node MUST have a property `smithy:value` with a *value* using these same production rules.

```turtle
smithy:value [
 a rdf:Bag ;
 rdf:_1 [
   smithy:key "Homepage" ;
   smithy:value "https://www.example.com/" ;
 ] ;
 rdf:_1 [
   smithy:key "API Reference" ;
   smithy:value "https://www.example.com/api-ref" ;
 ] ;
]
```

### Null

Smithy supports the notion of a null type, this is represented by the specific IRI `smithy:null`.

```turtle
[] smithy:value smithy:null .
```