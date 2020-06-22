# Atelier Project

Rust native library and tools for the AWS [Smithy](https://github.com/awslabs/smithy) Interface Definition Language.

![MIT License](https://img.shields.io/badge/license-mit-118811.svg)
![Minimum Rust Version](https://img.shields.io/badge/Min%20Rust-1.40-green.svg)
[![travis.ci](https://travis-ci.org/johnstonskj/rust-atelier.svg?branch=master)](https://travis-ci.org/johnstonskj/rust-atelier)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/rust-atelier.svg)](https://github.com/johnstonskj/rust-atelier/stargazers)

Given that a Smithy is "_the workshop of a smith_", this project has been named for "_an artist's or designer's studio or workroom_". Also, given that Fashion Tech. is just a damn fun place to be these days, it seemed appropriate.

## Crates

| crate                                | Content                                                | Status                              |
|--------------------------------------|--------------------------------------------------------|-------------------------------------|
| [`atelier_core`](./atelier-core)     | The Model, Builder, and Selector types and I/O traits. | Model: 80%, Builder: 75%, Selector: 20%, Tests: 0% |
| `atelier_smithy`                     | The serializer/de-serializer for native Smithy.        | Writer: 90%, Reader: 0%, Tests: 5%                 |
| `atelier_json`                       | The serializer/de-serializer for the JSON AST          | Writer: 90%, Reader: 0%, Tests: 5%                 |
| `atelier_openapi`                    | TBD                                                    | Not Started                         |
| `atelier_lib`                        | TBD                                                    | Awaiting completion of `core`.      |
| `cargo_atelier`                      | TBD                                                    | Not Started                         |

## Example

TBD

## Changes

**Version 0.1.0**

* Initial types for manipulation of Smithy Models, _not_ including selector expressions.
* Initial builder types for fluent construction of models.
* Builder example working to construct the weather model.
* Complete JSON serialization of the weather model.
* Complete Smithy serialization of the weather model.
* Goal: to be able to create a copy of the weather example model in code using the model builder API.

## TODO

1. Complete model for selector expressions.
1. Validation (internal and external) support.
1. Parser for Smithy representation.
1. Parser for Smithy's JSON AST representation.
1. Parser and writer for OpenAPI representation.
1. Work on cargo command for processing Smithy files.