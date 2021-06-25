# Atelier: crate atelier_assembler

This crate provides the model assembly capability, to merge files into a single in-memory `Model`. 

[![crates.io](https://img.shields.io/crates/v/atelier_assembler.svg)](https://crates.io/crates/atelier_assembler)
[![docs.rs](https://docs.rs/atelier_assembler/badge.svg)](https://docs.rs/atelier_assembler)

A tool can add files one-by-one, or from a directory, and then process them all into a single model. This
implementation understands the different registered file extensions so that it can read files
in different representations and assemble them seamlessly.

# Example

The following is the simple, and most common, method of using the assembler. This uses the
default `FileTypeRegistry` and will search for all models in the set of paths specified in
the environment variable "`SMITHY_PATH`".

```rust
use atelier_assembler::ModelAssembler;
use atelier_core::error::Result;
use atelier_core::model::Model;
use std::convert::TryFrom;

let env_assembler = ModelAssembler::default();

let model: Result<Model> = Model::try_from(env_assembler);
```

For more information, see [the Rust Atelier book](https://rust-atelier.dev/using/assembly.html).

## Changes

**Version 0.1.2**

* Implemented [cargo-atelier's lint and validate should work on models built from multiple namespace 
  files](https://github.com/johnstonskj/rust-atelier/issues/27) using the model assembler.
  * Fixed compiler warnings in this crate.

**Version 0.1.1**

* Refactored to produce:
  * A FileReader function type that parses a file type.
  * A FileType that matches a name, reader, and MIME type.
  * A FileTypeRegistry that matches one or more file extensions to a file type.

**Version 0.1.0**

* Extracted from the atelier_lib crate.

