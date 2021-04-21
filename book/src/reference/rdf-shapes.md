# Shapes

1. Each *top-level* shape is an RDF resource, it's identifier is the URN form of the shape's **Shape ID**.
1. The shape resource MUST have a property `rdf:type` that denotes it's Smithy type.
1. The shape resource MAY have applied traits, see later for details.
1. Aggregate and Service shapes have members, these *member* shapes have a common set of rules.
1. All *top-level* and *member* shape resource MAY have [applied traits](rdf-traits-values.md#traits).

## Member Shapes

1. Each *member* shape is an RDF resource, it's identifier is the URN form of the shape's **Shape ID**.
1. The shape resource MUST have a property `rdf:type` with a value which is the identifier of a *top-level* shape 
   resource.
1. The shape resource MUST have a property `smithy:name` with a value which is a literal string for the
   name of this member, this value must be a valid identifier.

```turtle
motd:GetMessageInput
  a smithy:Structure ;
  smithy:member [
    a motd:Date ;
    smithy:name "date"
  ] .
```

## Simple shapes

No additional rules.

```turtle
<urn:smithy:example.motd:Date> a smithy:String .
```

### List shapes

1. The shape resource MUST have a property `rdf:type` with the IRI value `smithy:List`.
1. The shape resource MUST have a *member* shape with the name "member".

```turtle
<urn:smithy:example.motd:Messages> 
  a smithy:List ;
  smithy:member [
    a smithy:String ;
    smithy:name "member"
  ] .
```

### Set shapes

1. The shape resource MUST have a property `rdf:type` with the IRI value `smithy:Set`.
1. The shape resource MUST have a *member* shape with the name "member".

```turtle
<urn:smithy:example.motd:Messages> 
  a smithy:Set ;
  smithy:member [
    a smithy:String ;
    smithy:name "member"
  ] .
```

### Map shapes

1. The shape resource MUST have a property `rdf:type` with the IRI value `smithy:Map`.
1. The shape resource MUST have a *member* shape with the name "key".
1. The shape resource MUST have a *member* shape with the name "value".

```turtle
<urn:smithy:example.motd:Messages> 
  a smithy:Map ;
  smithy:member [
    a <urn:smithy:example.motd:Language> ;
    smithy:name "key"
  ] ;
  smithy:member [
    a smithy:String ;
    smithy:name "value"
  ] .
```

### Structure shapes

1. The shape resource MUST have a property `rdf:type` with the IRI value `smithy:Structure`.
1. The shape resource MAY have any number of *member* shapes.

```turtle
<urn:smithy:example.motd:MessageResponse> 
  a smithy:Structure ;
  smithy:member [
    a smithy:String ;
    smithy:name "language"
  ] ;
  smithy:member [
    a smithy:String ;
    smithy:name "message"
  ] .
```

### Union shapes

1. The shape resource MUST have a property `rdf:type` with the IRI value `smithy:Structure`.
1. The shape resource MAY have any number of *member* shapes.

```turtle
<urn:smithy:example.motd:MessageResponse> 
  a smithy:Union ;
  smithy:member [
    a smithy:Integer ;
    smithy:name "messageCode"
  ] ;
  smithy:member [
    a smithy:String ;
    smithy:name "message"
] .
```

### Operation shapes

1. The shape resource MUST have a property `rdf:type` with the IRI value `smithy:Operation`.
1. The shape resource MAY have a property, named `smithy:input` with a value which is the identifier
   of a top-level shape resource.
1. The shape resource MAY have a property, named `smithy:output` with a value which is the identifier
   of a top-level shape resource.
1. The shape resource MAY have any number of properties, named `smithy:error` with a value which is the 
   identifier of a top-level shape resource.

```turtle
<urn:smithy:example.motd:GetMessage>
  a smithy:Operation ;
  smithy:input <urn:smithy:example.motd:GetMessageRequest> ;
  smithy:output <urn:smithy:example.motd:GetMessageResponse> ;
  smithy:error <urn:smithy:example.motd:BadDateValue> .
 ```

### Resource shapes

1. The shape resource MUST have a property `rdf:type` with the IRI value `smithy:Resource`.
1. The shape resource MAY have a property, named `smithy:identifiers` with a blank node value.
    1. This blank node MUST have a property `rdf:type` with the IRI value `rdf:Bag`.
    1. Each property of this blank node follows the standard method to generate predicate names of the
       form `rdf:_{n}` with a blank node value.
        1. This blank node MUST have a property `smithy:key` with a literal string value representing the
           identifier item's key.
        1. This blank node MUST have a property `smithy:target` with a value which is the identifier
           of a top-level shape resource.
1. The shape resource MAY have a property, named `smithy:create` with a value which is the identifier
   of a top-level shape resource.
1. The shape resource MAY have a property, named `smithy:put` with a value which is the identifier
   of a top-level shape resource.
1. The shape resource MAY have a property, named `smithy:read` with a value which is the identifier
   of a top-level shape resource.
1. The shape resource MAY have a property, named `smithy:update` with a value which is the identifier
   of a top-level shape resource.
1. The shape resource MAY have a property, named `smithy:delete` with a value which is the identifier
   of a top-level shape resource.
1. The shape resource MAY have a property, named `smithy:list` with a value which is the identifier of a shape resource.
1. The shape resource MAY have any number of properties, named `smithy:operation` with a value which is the
   identifier of a top-level shape resource.
1. The shape resource MAY have any number of properties, named `smithy:collectionOperation` with a value which is the
   identifier of a top-level shape resource.
1. The shape resource MAY have any number of properties, named `smithy:resource` with a value which is the
   identifier of a top-level shape resource.

```turtle
weather:Forecast
  a smithy:Resource ;
  smithy:identifiers [
    a rdf:Bag ;
    rdf:_1 [
      smithy:key "forecastId" ;
      smithy:target weather:ForecastId
    ]
  ] ;
  smithy:read weather:GetForecast .
```

### Service shapes

1. The shape resource MUST have a property `rdf:type` with the IRI value `smithy:Service`.
1. The shape resource MUST have a property `smithy:version` with a literal,
   non-empty, string value.
1. The shape resource MAY have any number of properties, named `smithy:operation` with a value which is the
   identifier of a top-level shape resource.
1. The shape resource MAY have any number of properties, named `smithy:resource` with a value which is the
   identifier of a top-level shape resource.
1. The shape resource MAY have a property, named `smithy:rename` with a blank node value.
    1. This blank node MUST have a property `rdf:type` with the IRI value `rdf:Bag`.
    1. Each property of this blank node follows the standard method to generate predicate names of the
       form `rdf:_{n}` with a blank node value.
        1. This blank node MUST have a property `smithy:shape` with a value which is the identifier
           of a top-level shape resource.
        1. This blank node MUST have a property `smithy:name` with a literal string value.

```turtle
example:MyService
  a smithy:Service ;
  smithy:version "2017-02-11" ;
  smithy:operations [
    a rdf:Bag ;
    rdf:_1 example:GetSomething
  ] ;
  smithy:rename [
    a rdf:Bag ;
    rdf:_1 [
      smithy:shape <urn:smithy:foo.example:Widget> ;
      smithy:name "FooWidget"
    ]
  ] .
```

