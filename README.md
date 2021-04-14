# The _Atelier_ Project

Rust native library and tools for the AWS [Smithy](https://github.com/awslabs/smithy) Interface Definition Language.

![MIT License](https://img.shields.io/badge/license-mit-118811.svg)
![Minimum Rust Version](https://img.shields.io/badge/Min%20Rust-1.40-green.svg)
![Rust](https://github.com/johnstonskj/rust-atelier/workflows/Rust/badge.svg)
![Audit](https://github.com/johnstonskj/rust-atelier/workflows/Security%20audit/badge.svg)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/rust-atelier.svg)](https://github.com/johnstonskj/rust-atelier/stargazers)
[![Gitpod ready-to-code](https://img.shields.io/badge/Gitpod-ready--to--code-blue?logo=gitpod)](https://gitpod.io/#https://github.com/johnstonskj/rust-atelier)

Smithy is a framework around a language and runtime neutral IDL for service definition. It consists of a semantic model,
a native IDL representation, and a mapping to JSON. The goal of this work is to replicate these core components of 
Smithy in Rust but also to experiment in alternate integrations such as the representation of the semantic model in 
RDF. This work should provide the basis for Rust code generation from a Smithy model and other tooling opportunities,
but those are not included in this repo.

**The Name "Atelier"**

Given that a Smithy is "_the workshop of a smith_", this project has been named for "_an artist's or designer's studio or 
workroom_". Also, given that Fashion Tech. is just a damn fun place to be these days, it seemed appropriate. 

## Crates

This repo contains a number of crates that provide different aspects, or capabilities, around Smithy processing. For 
clients that wish to use a number of features but don't want to track individual version compatibility the `atelier_lib` 
crate incorporates all, except the cargo command, using features to select the different capabilities.

| crate                                    | Content                                                | Status                              |
|------------------------------------------|--------------------------------------------------------|-------------------------------------|
| [`atelier_core`](./atelier-core)         | The Model, Builder, and Selector types and I/O traits. | Model: Done, Builder: Done, Tests: OK |
| [`atelier_describe`](./atelier-describe) | Model documentation writer, uses `somedoc` crate.      | Writer: 75%, Tests: Low             |
| [`atelier_json`](./atelier-json)         | The serializer/de-serializer for the JSON AST          | Reader/Writer: Done, Tests: Low     |
| [`atelier_lib`](./atelier-lib)           | Re-export structures from previous crates.             | Done.                               |
| [`atelier_openapi`](./atelier-openapi)   | A Serializer only to OpenAPI                           | Not Started                         |
| [`atelier_query`](./atelier-query)       | Evaluator for Smithy select expressions.               | Not Started                         |
| [`atelier_rdf`](./atelier-rdf)           | The serializer/de-serializer to RDF                    | Model: Done                         |
| [`atelier_smithy`](./atelier-smithy)     | The serializer/de-serializer for native Smithy.        | Reader/Writer: Done, Tests: Low     |
| [`cargo_atelier`](./cargo-atelier)       | Cargo command to lint, validate, and convert models.   | Commands working correctly.         |

## Book

This repo also contains the source (using [mdbook](https://rust-lang.github.io/mdBook/)) of a more complete documentation
set. This is built using GitHub pages and accessible at [rust-atelier.dev](https://rust-atelier.dev/).

## Changes

See individual crate README files.