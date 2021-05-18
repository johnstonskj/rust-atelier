/*!
This module provides a *line-oriented* format for the Smithy model. This representation is such
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

The following is a description of the production of the format, note there are a number of
separator strings used; 1) "`::`" the *segment* separator, 2) "`=>`" the *target* operator,
and "`<=`" the *value assignment* operator.

```text
 1. {shape_type}::{shape_id}
 2. {shape_type}::{shape_id}::trait::{shape_id}
 3. {shape_type}::{shape_id}::trait::{shape_id}<={value...}
 4. {shape_type}::{shape_id}::{identifier}=>{shape_id}
 5. {shape_type}::{shape_id}::{identifier}::trait::{shape_id}
 6. {shape_type}::{shape_id}::{identifier}::trait::{shape_id}<={value...}
 7. {shape_type}::{shape_id}::identifier::{identifier}=>{shape_id}
 8. {shape_type}::{shape_id}::rename::{shape_id}<={identifier}
 9. meta::{identifier}<={value...}
10. ()
11. {simple_value}
12. [{integer}]={value...}
13. {{identifier}}={value...}
```

* For each top-level shape:
  * emit the name of the shape type, a segment separator, and the shape's fully qualified name (1).
  * For each trait applied to this shape:
    * append to the above string the value "`::trait`" and the trait's fully qualified name (2),
    * if the trait has a value, append the "`<=`" and follow the value production rules below (3).
  * For each member of the shape:
    * emit a line with the member identifier, "`=>`", and the target's fully qualified name (4),
      * for array-valued members the member name emitted is the singular form (error for errors, etc.).
      * If the shape is a resource; emit the "`identifiers`" map-valued member:
        * append an additional "`::identifier::`" string,
        * emit each key followed by "`=>`", and the target's fully qualified name (7),
      * If the shape is a service; emit the "`renames`" map-valued member:
        * append an additional "`::rename::`" string,
        * emit each key followed by "`<=`", and the value (8),
    * For each trait applied to the member:
      * append to the above string the value "`::trait`" and the trait's fully qualified name (5),
      * if the trait has a value, append the "`<=`" and follow the value production rules below (6).

* For each value in the model metadata map:
  * use the string "`meta`" as if it where a shape name followed by "`::`"
  * append the key name, the string "`<=`" and follow the value production rules below (9).

* For null values simply emit the string "`()`" (10).
* For boolean, numeric, and string values emit their natural form (11).
  * Ensure string values are correctly quoted.
* For arrays:
  * emit a line per index, with "`[`", the index as a zero-based integer, "`]`", the operator
    "`<=`" and follow these same value production rules (12),
  * an empty array is denoted with the string "`[]`".
* For objects:
  * emit a line per key, with "`{`", the key name, "`}`", the operator "`<=`" and follow
    these same value production rules (13),
  * an empty object is denoted with the string "`{}`".

Finally, all lines must be sorted to ensure the overall output can be compared.

## Examples

A simple string shape with a trait applied.

```text
// "pattern" is a trait.
@pattern("^[A-Za-z0-9 ]+$")
string CityId
```

```text
string::example.weather#CityId
string::example.weather#CityId::trait::smithy.api#pattern<="^[A-Za-z0-9 ]+$"
```

An operation, note the rename of "errors" to "error as the member identifier.

```text
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

A service, note the object-based trait "paginated" and the comment that has been turned into a
documentation trait.

```text
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
*/

use crate::io::ModelWriter;
use crate::model::shapes::{HasTraits, MemberShape, ShapeKind, TopLevelShape};
use crate::model::values::Value;
use crate::model::{HasIdentity, Model, ShapeID};
use std::collections::HashMap;
use std::io::Write;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This type implements the `ModelWriter` trait and will output the line-oriented format when
/// `ModelWriter::write` is called.
///
/// See the [module-level](index.html) documentation for details of this format.
///
#[derive(Debug)]
pub struct LineOrientedWriter {}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Make a line-oriented version of the provided model and return as a list of strings.
///
/// See the [module-level](index.html) documentation for details of this format.
///
pub fn make_line_oriented_form(model: &Model) -> Vec<String> {
    let mut strings = Default::default();
    for (key, value) in model.metadata() {
        value_into_strings(
            &format!("{}{}{}{}", META_PREFIX, SEGMENT_SEP, key, VALUE_SEP),
            value,
            &mut strings,
        );
    }
    for shape in model.shapes() {
        shape_into_strings(shape, &mut strings)
    }
    strings.sort();
    strings
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for LineOrientedWriter {
    fn default() -> Self {
        Self {}
    }
}

impl ModelWriter for LineOrientedWriter {
    fn write(&mut self, w: &mut impl Write, model: &Model) -> crate::error::Result<()> {
        for line in make_line_oriented_form(model) {
            writeln!(w, "{}", line)?;
        }
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

const META_PREFIX: &str = "meta";
const TRAIT_PREFIX: &str = "trait";
const SEGMENT_SEP: &str = "::";
const TARGET_SEP: &str = "=>";
const VALUE_SEP: &str = "<=";

fn line_prefix(shape_type: &str, shape: &TopLevelShape) -> String {
    format!("{}{}{}", shape_type, SEGMENT_SEP, shape.id())
}
fn shape_into_strings(shape: &TopLevelShape, strings: &mut Vec<String>) {
    let prefix = match shape.body() {
        ShapeKind::Simple(v) => {
            let prefix = line_prefix(&v.to_string(), shape);
            strings.push(prefix.clone());
            prefix
        }
        ShapeKind::List(v) => {
            let prefix = line_prefix("list", shape);
            strings.push(prefix.clone());
            member_into_strings(&prefix, v.member(), strings);
            prefix
        }
        ShapeKind::Set(v) => {
            let prefix = line_prefix("set", shape);
            strings.push(prefix.clone());
            member_into_strings(&prefix, v.member(), strings);
            prefix
        }
        ShapeKind::Map(v) => {
            let prefix = line_prefix("map", shape);
            strings.push(prefix.clone());
            member_into_strings(&prefix, v.key(), strings);
            member_into_strings(&prefix, v.value(), strings);
            prefix
        }
        ShapeKind::Structure(v) => {
            let prefix = line_prefix("structure", shape);
            strings.push(prefix.clone());
            for member in v.members() {
                member_into_strings(&prefix, member, strings);
            }
            prefix
        }
        ShapeKind::Union(v) => {
            let prefix = line_prefix("union", shape);
            strings.push(prefix.clone());
            for member in v.members() {
                member_into_strings(&prefix, member, strings);
            }
            prefix
        }
        ShapeKind::Service(v) => {
            let prefix = line_prefix("service", shape);
            strings.push(prefix.clone());
            strings.push(format!(
                "{}{}version{}{:?}",
                prefix,
                SEGMENT_SEP,
                VALUE_SEP,
                v.version()
            ));
            for id in v.operations() {
                strings.push(format!("{}{}", prefix, member_target("operation", id)));
            }
            for id in v.resources() {
                strings.push(format!("{}{}", prefix, member_target("resource", id)));
            }
            for (id, ln) in v.renames() {
                strings.push(format!(
                    "{}{}rename{}{}{}{}",
                    prefix, SEGMENT_SEP, SEGMENT_SEP, id, VALUE_SEP, ln
                ));
            }
            prefix
        }
        ShapeKind::Operation(v) => {
            let prefix = line_prefix("operation", shape);
            strings.push(prefix.clone());
            if let Some(id) = v.input() {
                strings.push(format!("{}{}", prefix, member_target("input", id)));
            }
            if let Some(id) = v.output() {
                strings.push(format!("{}{}", prefix, member_target("output", id)));
            }
            for id in v.errors() {
                strings.push(format!("{}{}", prefix, member_target("error", id)));
            }
            prefix
        }
        ShapeKind::Resource(v) => {
            let prefix = line_prefix("resource", shape);
            strings.push(prefix.clone());
            let ident_prefix = format!("{}identifier", SEGMENT_SEP);
            for (k, id) in v.identifiers() {
                strings.push(format!(
                    "{}{}{}",
                    prefix,
                    ident_prefix,
                    member_target(&k.to_string(), id)
                ));
            }
            if let Some(id) = v.create() {
                strings.push(format!("{}{}", prefix, member_target("create", id)));
            }
            if let Some(id) = v.put() {
                strings.push(format!("{}{}", prefix, member_target("put", id)));
            }
            for id in v.read() {
                strings.push(format!("{}{}", prefix, member_target("read", id)));
            }
            if let Some(id) = v.update() {
                strings.push(format!("{}{}", prefix, member_target("update", id)));
            }
            if let Some(id) = v.delete() {
                strings.push(format!("{}{}", prefix, member_target("delete", id)));
            }
            for id in v.list() {
                strings.push(format!("{}{}", prefix, member_target("list", id)));
            }
            for id in v.operations() {
                strings.push(format!("{}{}", prefix, member_target("operation", id)));
            }
            for id in v.collection_operations() {
                strings.push(format!(
                    "{}{}",
                    prefix,
                    member_target("collection_operation", id)
                ));
            }
            for id in v.resources() {
                strings.push(format!("{}{}", prefix, member_target("resource", id)));
            }
            prefix
        }
        ShapeKind::Unresolved => {
            let prefix = line_prefix("unresolved", shape);
            strings.push(prefix.clone());
            prefix
        }
    };
    let prefix = format!("{}{}{}{}", prefix, SEGMENT_SEP, TRAIT_PREFIX, SEGMENT_SEP);
    traits_into_strings(&prefix, shape.traits(), strings);
}

fn member_target(member_name: &str, id: &ShapeID) -> String {
    format!("{}{}{}{}", SEGMENT_SEP, member_name, TARGET_SEP, id)
}

fn value_into_strings(prefix: &str, value: &Value, strings: &mut Vec<String>) {
    match value {
        Value::Array(vs) => {
            if vs.is_empty() {
                strings.push(format!("{}[]", prefix));
            } else {
                for (i, v) in vs.iter().enumerate() {
                    let prefix = format!("{}[{}]=", prefix, i);
                    value_into_strings(&prefix, v, strings);
                }
            }
        }
        Value::Object(vo) => {
            if vo.is_empty() {
                strings.push(format!("{}{{}}", prefix));
            } else {
                for (k, v) in vo {
                    let prefix = format!("{}{{{}}}=", prefix, k);
                    value_into_strings(&prefix, v, strings);
                }
            }
        }
        Value::Number(v) => strings.push(format!("{}{}", prefix, v)),
        Value::Boolean(v) => strings.push(format!("{}{}", prefix, v)),
        Value::String(v) => strings.push(format!("{}{:?}", prefix, v)),
        Value::None => strings.push(format!("{}()", prefix)),
    }
}

fn member_into_strings(prefix: &str, member: &MemberShape, strings: &mut Vec<String>) {
    let prefix = format!(
        "{}{}{}",
        prefix,
        SEGMENT_SEP,
        member.id().member_name().as_ref().unwrap()
    );
    strings.push(format!("{}{}{}", prefix, TARGET_SEP, member.target()));
    let prefix = format!("{}{}{}{}", prefix, SEGMENT_SEP, TRAIT_PREFIX, SEGMENT_SEP);
    traits_into_strings(&prefix, member.traits(), strings);
}

fn traits_into_strings(
    prefix: &str,
    traits: &HashMap<ShapeID, Option<Value>>,
    strings: &mut Vec<String>,
) {
    for (shape_id, value) in traits {
        match value {
            None => {
                strings.push(format!("{}{}", prefix, shape_id));
            }
            Some(value) => {
                value_into_strings(
                    &format!("{}{}{}", prefix, shape_id, VALUE_SEP),
                    value,
                    strings,
                );
            }
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
