# The LineOrientedWriter

This representation is such
that it is always ordered and has no whitespace or other ambiguities and so can be directly
compared as a whole. This is valuable for testing but can also be extremely fast for parsing
tools.

# Example

```rust
use atelier_core::io::ModelWriter;
use atelier_core::io::lines::LineOrientedWriter;
use atelier_core::model::Model;
# fn make_model() -> Model { Model::default() }
let model = make_model();

let mut writer = LineOrientedWriter::default();
let result = writer.write(&mut std::io::stdout(), &model);
assert!(result.is_ok())
```

# Representation Format

The following is a description of the production of the format.

```text
segment-separator = "::" ;
target-operator = "=>" ;
value-assignment-operator = "<=" ;
```

The numbers on the left of the lines below are for reference within in the production rule text.

```text
 1. {shape_type}::{shape_id}
 2. {shape_type}::{shape_id}::trait::{shape_id}
 3. {shape_type}::{shape_id}::trait::{shape_id}<={value...}
 4. {shape_type}::{shape_id}::{member_name}=>{target_shape_id}
 5. {shape_type}::{shape_id}::{member_name}::trait::{shape_id}
 6. {shape_type}::{shape_id}::{member_name}::trait::{shape_id}<={value...}
 7. resource::{shape_id}::identifier::{identifier}=>{shape_id}
 8. service::{shape_id}::rename::{shape_id}<={identifier}
 9. meta::{identifier}<={value...}
10. ()
11. {simple_value}
12. [{integer}]<={value...}
13. {{identifier}}<={value...}
```

**Shape Production Rules**

For each top-level shape:

* emit the name of the shape's type, a _segment-separator_, and the shape's fully qualified name, and a newline (1).
* For each trait applied to this shape:
    * append to the above string the value "`::trait`", and the trait's fully qualified name (2),
    * if the trait has a value, append the _value-assignment-operator_ and follow the value production rules below (3).
    * finish with a newline.
* For each member of the shape:
    * emit a line with the member identifier, the _target-operator_, and the target's fully qualified name, and a newline (4),
        * for array-valued members the member name emitted is the singular form with a line per value (error for errors, etc.).
        * If the shape is a resource; emit the "`identifiers`" map-valued member:
            * append an additional "`::identifier::`" string,
            * emit each key followed by the _target-operator_, and the target's fully qualified name, and a newline (7),
        * If the shape is a service; emit the "`rename`" map-valued member:
            * append an additional "`::rename::`" string,
            * emit each key (fully qualified shape ID), followed by the _value-assignment-operator_, and the value (identifier), and a newline (8),
    * For each trait applied to the member:
        * append to the above string the value "`::trait`", and the trait's fully qualified name (5),
        * if the trait has a value, append the _value-assignment-operator_ and follow the value production rules below (6).
        * finish with a newline.

**Metadata Production Rules**

For each value in the model's metadata map:

* use the string "`meta`" as if it where a shape name followed by the _segment-separator_.
* append the key name, the _value-assignment-operator_ and follow the value production rules below (9).

**Value Production Rules**

* For null values simply emit the string `"()"` (10).
* For boolean, numeric, and string values emit their natural form (11).
    * Ensure string values quote the characters `'\n'`, `'\r'`, and `'"'`.
* For arrays:
    * emit a line per index, with `'['`, the index as a zero-based integer, `']'`, the _value-assignment-operator_
      and follow these same value production rules (12),
    * an empty array will be denoted by the string `"[]"`.
* For objects:
    * emit a line per key, with `"{"`, the key name, `"}"`, the _value-assignment-operator_ and follow
      these same value production rules (13),
    * an empty object MUST be denoted by the string `"{}"`.

Finally, all lines MUST be sorted to ensure the overall output can be compared.

## Examples

A simple string shape with a trait applied.

```smithy
// "pattern" is a trait.
@pattern("^[A-Za-z0-9 ]+$")
string CityId
```

```text
string::example.weather#CityId
string::example.weather#CityId::trait::smithy.api#pattern<="^[A-Za-z0-9 ]+$"
```

An operation, note the rename of "errors" to "error as the member identifier.

```smithy
@readonly
operation GetCity {
    input: GetCityInput
    output: GetCityOutput
    errors: [NoSuchResource]
}
```

```text
operation::example.weather#GetCity
operation::example.weather#GetCity::error=>example.weather#NoSuchResource
operation::example.weather#GetCity::input=>example.weather#GetCityInput
operation::example.weather#GetCity::output=>example.weather#GetCityInput
operation::example.weather#GetCity::trait::smithy.api#readonly
```

A service, note the object-based trait "paginated", and the comment that has been turned into a
documentation trait.

```smithy
/// Provides weather forecasts.
@paginated(inputToken: "nextToken", outputToken: "nextToken",
           pageSize: "pageSize")
service Weather {
    version: "2006-03-01"
    resources: [City]
    operations: [GetCurrentTime]
    rename: {
        "foo.example#Widget": "FooWidget"
    }
}
```

```text
service::example.weather#Weather
service::example.weather#Weather::operation=>example.weather#GetCurrentTime
service::example.weather#Weather::resource=>example.weather#City
service::example.weather#Weather::rename::foo.example#Widget<=FooWidget
service::example.weather#Weather::trait::smithy.api#documentation<="Provides weather forecasts."
service::example.weather#Weather::trait::smithy.api#paginated<={inputToken}="nextToken"
service::example.weather#Weather::trait::smithy.api#paginated<={outputToken}="nextToken"
service::example.weather#Weather::trait::smithy.api#paginated<={pageSize}="pageSize"
service::example.weather#Weather::version<="2006-03-01"
```
