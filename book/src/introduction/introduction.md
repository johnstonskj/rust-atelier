# Introduction

The [Atelier](https://github.com/johnstonskj/rust-atelier) project is a suite of Rust crates that provides the ability 
to read, write, and process AWS [Smithy](https://github.com/awslabs/smithy) interface definition models. AWS is using 
Smithy extensively to define their services and to generate client and server implementations. 

The goal of the Atelier project is to provide both Rust-native crates that allow parsing and emitting of Smithy models
but also a clean-slate implementation of the Smithy specifications. This aspect has been useful, addressing ambiguities 
in the published documentation.

After a more detailed description of [Smithy](smithy.md) itself, and a tour of the [Atelier crates](crates.md) this book
will cover the following topics:

* How to programmatically create and manipulate in-memory models.
* How to read and write Smithy model files, including assembling in-memory models from multiple source files.
* How to use the lint and validate framework to check models.
* How to run the `cargo-atelier` tool to perform many of these actions from the command-line.
* How to extend the Atelier provided tools.
