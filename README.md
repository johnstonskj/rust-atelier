# Atelier Project

Rust native library and tools for the AWS [Smithy](https://github.com/awslabs/smithy) Interface Definition Language.

![MIT License](https://img.shields.io/badge/license-mit-118811.svg)
![Minimum Rust Version](https://img.shields.io/badge/Min%20Rust-1.40-green.svg)
![Rust](https://github.com/johnstonskj/rust-atelier/workflows/Rust/badge.svg)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/rust-atelier.svg)](https://github.com/johnstonskj/rust-atelier/stargazers)

Given that a Smithy is "_the workshop of a smith_", this project has been named for "_an artist's or designer's studio or workroom_". Also, given that Fashion Tech. is just a damn fun place to be these days, it seemed appropriate. This repo contains a number of crates that provide different aspects, or capabilities, around Smithy processing. For clients that wish to use a number of features but don't want to track individual version compatibility the `atelier_lib` crate incorporates all but the cargo command using features to select the different capabilities.

## Crates

| crate                                  | Content                                                | Status                              |
|----------------------------------------|--------------------------------------------------------|-------------------------------------|
| [`atelier_core`](./atelier-core)       | The Model, Builder, and Selector types and I/O traits. | Model: Done, Builder: Done, Selector: 20%, Tests: 5% |
| [`atelier_smithy`](./atelier-smithy)   | The serializer/de-serializer for native Smithy.        | Writer: 95%, Reader: 95%, Tests: 5% |
| [`atelier_json`](./atelier-json)       | The serializer/de-serializer for the JSON AST          | Writer: 95%, Reader: 95%, Tests: 5% |
| [`atelier_openapi`](./atelier-openapi) | A Serializer only to OpenAPI                           | Not Started                         |
| [`atelier_lib`](./atelier-lib)         | Re-export structures from previous crates.             | Done.                               |
| [`cargo_atelier`](./cargo-atelier)     | Cargo command to lint, validate, and convert models.   | Commands working correctly.         |


## Changes

**Version 0.1.2**

* cargo command working, validation and linters started

**Version 0.1.2**

* Core API support for readers.
* Parser for Smithy representation.
* Parser for Smithy's JSON AST representation.
* Additional documentation.

**Version 0.1.1**

* Cleaned up core and builder APIs. 
* Documented all core package APIs.
* Goal: documentation and examples.

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
1. Parser and writer for OpenAPI representation.
1. Work on cargo command for processing Smithy files.
