# Atelier: crate atelier_lib

A combined crate for all Atelier sub-crates incorporated as features. Atelier is a Rust native
library, and tools, for the AWS [Smithy](https://github.com/awslabs/smithy) Interface Definition
Language.

[![crates.io](https://img.shields.io/crates/v/atelier_lib.svg)](https://crates.io/crates/atelier_lib)
[![docs.rs](https://docs.rs/atelier_lib/badge.svg)](https://docs.rs/atelier_lib)

The aim of this crate is to provide a single client interface over a set of crates that provide
different Atelier capabilities. The following table shows the mapping from individual crate to the 
combined module path in this library. The column _Default_ indicates those that are included in the 
default feature, although the core will be included regardless of any feature selection.

| Feature name | Default | Individual crate                                    | Target module path                | Purpose                                               |
|--------------|---------|-----------------------------------------------------|-----------------------------------|-------------------------------------------------------|
| N/A          | **Yes** | [atelier_core](https://docs.rs/atelier_core)        | `atelier_lib::core`               | Core models only.                                     |
| "describe"   | Yes     | [atelier_describe](https://docs.rs/atelier_describe)| `atelier_lib::format::describe`   | Writing model documentation.                          |
| "json"       | Yes     | [atelier_json](https://docs.rs/atelier_json)        | `atelier_lib::format::json`       | Reading and Writing JSON AST representation.          |
| "openapi"    | No      | [atelier_openapi](https://docs.rs/atelier_openapi)  | `atelier_lib::format::openapi`    | Reading and Writing OpenAPI representations.          |
| "rdf"        | No      | [atelier_rdf](https://docs.rs/atelier_rdf)          | `atelier_lib::format::rdf`        | Reading and Writing RDF representations.              |
| "smithy"     | Yes     | [atelier_smithy](https://docs.rs/atelier_smithy)    | `atelier_lib::format::smithy`     | Reading and Writing the Smithy native representation. |
| "graphml"    | No      | Included in lib code                                | `atelier_lib::format::graphml`    | Writing GraphML XML.                                  |
| "uml"        | No      | Included in lib code                                | `atelier_lib::format::uml`        | Writing PlantUML source.                              |

As shown in the table, the feature "uml" is implemented in this crate, as is the _model assembler_ feature. Model
assembly is a step often required by tools to build a single semantic model from a set of file representations.

## Changes

**Version 0.2.4**

* Moved GraphML and PlantUML writers to the describe crate.

**Version 0.2.3**

* Removed dependency on deprecated 'select' crate.
* Updated due to trait API changes in core.

**Version 0.2.2**

* Support for documentation writer.

**Version 0.2.1**

* Support for GraphML writer.

**Version 0.2.0**

* Major refactoring

**Version 0.1.6**

* Added `UnwelcomeTerms` linter to standard set.
* Added `assembler` module.
* Pulled most things out of `lib.rs` into separate modules.
* Made "json" a default feature.

**Version 0.1.5**

Bumped versions of core, json, and smithy crates.

**Version 0.1.4**

* Added the `action` module with `standard_model_lint` and `standard_model_validation` functions.

**Version 0.1.3**

* Documentation fixes.

**Version 0.1.2**

* New dependency versions:
  * core: v0.1.3
  * json: v0.1.2
  * smithy: v0.1.2

**Version 0.1.1**

* All re-exports configured using feature flags.

