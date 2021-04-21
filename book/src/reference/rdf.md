# Appendix: RDF Mapping

This appendix describes the mapping from Smithy to RDF in detail.

## RDF Representation

The examples below are shown in the RDF [Turtle](https://www.w3.org/TR/turtle/) syntax. The following
namespace prefixes are used:

```turtle
@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
@prefix smithy: <https://awslabs.github.io/smithy/rdf-1.0#> .
@prefix api: <urn:smithy:smithy.api:> .
```

* *rdf* - the RDF namespace, used for certain type assertions.
* *xsd* - XML Schema data types.
* *smithy* - the namespace for the Smithy IDL mapping itself.
* *api* - the namespace for the Smithy prelude shapes; this follows the rules in the following section
  to generate a URN for the Smithy namespace `smithy.api`.

## Shape IDs

To allow for the linking of models in RDF the key identifier in and between models need to be represented
as [IRI](https://tools.ietf.org/html/rfc3987)s. This section introduces a Simple URN naming scheme for 
*absolute* Smithy shape identifiers.

While it is clear that a stable, unique identifier should be used in the same way as the Smithy Shape ID, it
is not at all clear that this needs to carry any location information with it. It would be preferrable to use
the Smithy trait system to associate locations with models rather than forcing location onto all models and 
model elements. The choice of a [URN](https://tools.ietf.org/html/rfc8141) over [URL](https://tools.ietf.org/html/rfc3986) 
scheme was therefore easier, and provides a clear, human-readable and easily parsed identifier format.

The following rules describe the mapping from Smithy Shape ID to a URN form required by the model and
shape mapping.

1. The URI scheme MUST be exactly `urn`.
1. The URN scheme MUST be exactly `smithy`.
1. The _namespace-specific string_ (NSS) MUST be formatted as follows.
   1. The identifier's namespace component.
   1. The colon character, `':'`.
   1. The identifier's shape name component.
   1. **If** the Shape ID represents a member shape:
      1. The forward slash character, `'/'`.
      1. The identifier's member name component.
 
The following demonstrates this mapping visually.

   ```text
              example.namespace#shape$member
              |---------------| |---| |----|
   urn:smithy:example.namespace:shape/member
   ```

The following is a simplified form of the mapping described above.

```rust
use atelier_core::model::ShapeID;

fn simple_shapeid_to_urn(shape_id: &ShapeID) -> String {
   format!(
      "urn:smithy:{}:{}{}",
      shape_id.namespace(),
      shape_id.shape_name(),
      if let Some(member_name) = shape_id.member_name() {
         format!("/{}", member_name)
      } else {
         String::new()
      }
   )
}
```