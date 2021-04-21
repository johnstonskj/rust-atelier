# Models

1. Each model is an RDF resource, it's identifier may be an IRI or blank node.
1. The model resource MUST have a property `rdf:type` with the IRI value `smithy:Model`.
1. The model resource MUST have a property `smithy:smithy_version` with a literal string value representing the
   Smithy version used to define the model.

    ```turtle
    [] 
     a smithy:Model ;
     smithy:smithy_version "1.0" .
    ```

1. **ForEach** shape in the model the model resource MUST have a property, named `smithy:metadata` with a value 
   which is the identifier of a top-level shape resource.

   ```turtle
   [] 
     a smithy:Model ;
     smithy:smithy_version "1.0" ;
     smithy:shape <urn:smithy:example.motd:Date> .
   ```

1. The model resource MAY have a property, named `smithy:metadata` that is treated as an *Object value* and 
   generated according to the [Value](rdf-traits-values.md#values) rules.

   ```turtle
   [] 
     a smithy:Model ;
     smithy:smithy_version "1.0" ;
     smithy:shape <urn:smithy:example.motd:Date> ;
     smithy:metadata [
       a rdf:Bag
       rdf:_1 [
         smithy:key "domain" ;
         smithy:value "identity" ;
       ]
     ] .
   ```
