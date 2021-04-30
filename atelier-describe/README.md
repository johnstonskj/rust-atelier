# Atelier: crate atelier_describe

Provides the ability to write documentation for [Smithy](https://github.com/awslabs/smithy) models.

[![crates.io](https://img.shields.io/crates/v/atelier_openapi.svg)](https://crates.io/crates/atelier_describe)
[![docs.rs](https://docs.rs/atelier_openapi/badge.svg)](https://docs.rs/atelier_describe)

This crate provides two mechanisms for generating human-readable documentation for a Smithy model
using the crate [somedoc](https://crates.io/crates/somedoc).

Firstly, the `DocumentationWriter` structure implements the `atelier_core::io::ModelWriter` trait and so may be used 
in the same manner as other model writers. The `ModelWriter::new` function takes an argument that will denote the 
format to produce, but provides little other control over the generation. Internally this writer implementation calls 
the following function.

The function `describe_model` will produce a `somedoc::model::Document` instance from a `atelier_core::modrel::Model`. 
This instance may then be rendered according to the writers provided by somedoc. This provides complete control over 
the actual formatting step, and the same generated Document may be written multiple times if required.

# Examples

The following demonstrates how to use the `describe_model` function.

```rust
use atelier_core::model::Model;
use atelier_describe::describe_model;
use somedoc::write::{write_document_to_string, OutputFormat};

let model = make_model();

let documentation = describe_model(&model).unwrap();

let doc_string = write_document_to_string(&documentation, OutputFormat::Html).unwrap();
```

The following example demonstrates the `ModelWriter` trait and outputs the documentation, in 
[CommonMark](https://spec.commonmark.org/) format, to stdout.

```rust
use atelier_core::model::Model;
use atelier_core::io::ModelWriter;
use atelier_describe::DocumentationWriter;
use std::io::stdout;

let model = make_model();

let mut writer = DocumentationWriter::default();

let documentation = writer.write(&mut stdout(), &model).unwrap();
```

## Changes

**Version 0.1.5**

* Refactored to have a similar structure to other reader/writer crates.
* Moved GraphML and PlantUML into this crate from lib.

**Version 0.1.4**

* Updated due to trait API changes in core.

**Version 0.1.3**

* Refactored `DocumentationWriter` so that it only emits CommonMark, more closely aligned with Smithy IDL.

**Version 0.1.1**

* Fixed: bumped somedoc dependency version.

**Version 0.1.1**

* Fixed: removed local path dependency for `somedoc`.

**Version 0.1.0**

This initial version is reasonably usable, although incomplete.


## TODO

1. Complete both prelude, and custom, trait output.
1. Test cases.
