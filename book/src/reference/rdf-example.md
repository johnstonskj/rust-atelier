# Example

Presented below is the *Message of the Day* Example with the corresponding Smithy and
RDF sections side by side for comparison. You can download the Turtle [source file](motd.ttl) as well.

<table class="plain">
<thead>
<tr>
<th></th> <th> Smithy IDL </th> <th> RDF Representation </th>
</tr>
</thead>

<tbody>
<tr>
<td> 1 </td>
<td style="vertical-align: top;">
</td>
<td style="vertical-align: top;">

```turtle
@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
@prefix smithy: <https://awslabs.github.io/smithy/rdf-1.0#> .
@prefix api: <urn:smithy:smithy.api:> .
@prefix : <urn:smithy:example.motd:> .
```

</td>
</tr>

<tr>
<td> 2 </td>
<td style="vertical-align: top;">

```smithy
$version: "1.0"

namespace example.motd
```

</td>
<td style="vertical-align: top;">

```turtle
[]
  a smithy:Model ;
  smithy:smithy_version "1.0" ;
  smithy:shape
    :GetMessageOutput ,
    :Message ,
    :GetMessageInput ,
    :BadDateValue ,
    :GetMessage ,
    :MessageOfTheDay ,
    :Date .
```

</td>
</tr>

<tr>
<td> 3 </td>
<td style="vertical-align: top;">

```smithy
@pattern(
  "^\\d\\d\\d\\d\\-\\d\\d\\-\\d\\d$"
)
string Date
```

</td>
<td style="vertical-align: top;">

```turtle
:Date
  a smithy:String ;
  smithy:apply [
    smithy:trait api:pattern ;
    smithy:value "^\\d\\d\\d\\d\\-\\d\\d-\\d\\d$"
  ] .
```

</td>
</tr>

<tr>
<td> 4 </td>
<td style="vertical-align: top;">

```smithy
resource Message {
   identifiers: {
      date: Date
   }
   read: GetMessage
}
```

</td>
<td style="vertical-align: top;">

```turtle
:Message
  a smithy:Resource ;
  smithy:identifiers [
    a rdf:Bag ;
    rdf:_1 [
      smithy:key "date" ;
      smithy:target :Date
    ]
  ] ;
  smithy:read :GetMessage .
```

</td>
</tr>

<tr>
<td> 5 </td>
<td style="vertical-align: top;">

```smithy
structure GetMessageInput {
   date: Date
}
```

</td>
<td style="vertical-align: top;">

```turtle
:GetMessageInput
  a smithy:Structure ;
  smithy:member [
    a :Date ;
    smithy:name "date"^^xsd:string
  ] .
```

</td>
</tr>

<tr>
<td> 6 </td>
<td style="vertical-align: top;">

```smithy
structure GetMessageOutput {
   @required
   message: String
}
```

</td>
<td style="vertical-align: top;">

```turtle
:GetMessageOutput
  a smithy:Structure ;
  smithy:member [
    a api:String ;
    smithy:name "message"^^xsd:string ;
    smithy:apply [ smithy:trait api:required ] ;
  ] .
```

</td>
</tr>

<tr>
<td> 7 </td>
<td style="vertical-align: top;">

```smithy
@error("client")
structure BadDateValue {
   @required
   errorMessage: String
}
```

</td>
<td style="vertical-align: top;">

```turtle
:BadDateValue
  a smithy:Structure ;
  smithy:apply [
    smithy:trait api:error ;
    smithy:value "client"
  ] ;
  smithy:member [
    a api:String ;
    smithy:name "errorMessage"^^xsd:string ;
    smithy:apply [ smithy:trait <urn:smithy:smithy.api:required> ] ;
  ] .
```

</td>
</tr>

<tr>
<td> 8 </td>
<td style="vertical-align: top;">

```smithy
@readonly
operation GetMessage {
   input: GetMessageInput
   output: GetMessageInput
   errors: [ BadDateValue ]
}
```

</td>
<td style="vertical-align: top;">

```turtle
:GetMessage
  a smithy:Operation ;
  smithy:input :GetMessageInput ;
  smithy:output :GetMessageOutput ;
  smithy:error :BadDateValue .
```

</td>
</tr>

<tr>
<td> 9 </td>
<td style="vertical-align: top;">

```smithy
@documentation(
  "Provides a Message of the day."
)
service MessageOfTheDay {
   version: "2020-06-21"
   resources: [ Message ]
}
```

</td>
<td style="vertical-align: top;">

```turtle
:MessageOfTheDay
  a smithy:Service ;
  smithy:apply [
    smithy:trait api:documentation ;
    smithy:value "Provides a Message of the day."
  ] ;
  smithy:version "2020-06-21" ;
  smithy:resource :Message .
```

</td>
</tr>

</tbody>
</table>
