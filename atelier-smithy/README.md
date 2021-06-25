# Atelier: crate atelier_smithy

Provides the ability to read and write [Smithy](https://github.com/awslabs/smithy) models in its own native language 
representation.

[![crates.io](https://img.shields.io/crates/v/atelier_smithy.svg)](https://crates.io/crates/atelier_smithy)
[![docs.rs](https://docs.rs/atelier_smithy/badge.svg)](https://docs.rs/atelier_smithy)

The crate provides two low-level parsers, `parse_model` and `parse_selector` that correspond to the
core `Model` and `Selector` types. These are decouple to allow for the tool use cases where it is
unnecessary or even undesirable to parse all selector expressions as well as those cases where
selector expressions may be useful in non-model scenarios. 

This crate also provides implementations of both the core `ModelReader` and `ModelWriter` traits
for Smithy IDL files.

# Example

The following demonstrates the `SmithyReader` to parse a file into the in-memory model.

```rust
use atelier_core::io::read_model_from_file;
use atelier_smithy::SmithyReader;
use std::path::PathBuf;

let path = PathBufBuf::from("tests/good/waiters.smithy");
let mut reader = SmithyReader::default();
let result = read_model_from_file(&mut reader, path);
assert!(result.is_ok());
```

# Example - Selector

The following example parses the simple selector expression `"*"`, denoting any shape, into it's
in-memory model.

```rust
use atelier_smithy::parse_selector;

let result = parse_selector("*");
assert!(result.is_ok());
```

## Changes

**Version 0.2.13**

* Implemented [cargo-atelier's lint and validate should work on models built from multiple namespace
  files](https://github.com/johnstonskj/rust-atelier/issues/27) using the model assembler.
  * Fixed bugs in Smithy writer when writing values (in traits and metadata).

**Version 0.2.12**

* Changed the ID on MemberShape from ShapeID to Identifier. This matches both the formal BNF for the Smithy IDL and the
  JSON AST usage.

**Version 0.2.11**

* Fixed escaping bug for line writer that made cross-platform comparison a pain.

**Version 0.2.10**

* Fixed test case typo.

**Version 0.2.9**

* From core: Reorganized errors, made the naming and messages consistent and removed unused.

**Version 0.2.8**

* Supporting the core model builders which now use TryFrom instead of From.
* Support the "renames" collection added to service shapes in the abstract model.
* All annotation traits use an empty object as value, not None.
* Using new shared atelier-test crate.

**Version 0.2.7**

* Supporting the core model builders which now use TryFrom instead of From.
* Using new shared atelier-test crate.

**Version 0.2.6**

* Updated due to trait API changes in core.

**Version 0.2.5**

* Updated Smithy grammar and parser to match changes in the specification:
  * turned the comma "," character into whitespace so it becomes optional as a member separator
  * removed the `br` rule as to ensure members are defined on separate lines
  * simplified a number of rules that had empty/populated pairs
* Added two additional test example files

**Version 0.2.4**

* Tracking change to `ShapeType` from `atelier_core` crate.

**Version 0.2.3**

* Added support for parsing selector expressions.

**Version 0.2.2**

* Fixed: test case compile error.

**Version 0.2.1**

* Changes based on new core traits `HasIdentity` and `HasTraits`.

**Version 0.2.0**

* Major refactor after agreement on the separation of semantic model with Smithy team.

**Version 0.1.5**

* API changes for `ModelReader` and `ModelWriter`.
  * removed `representation` method
  * added `FILE_EXTENSION` constant.
* Changed internal module organization to mirror the json crate.

**Version 0.1.4**

* Validated round-tripping Smithy files, read into a model then write back out again.

**Version 0.1.3**

* Changes to the core API around `ModelReader` and `ModelWriter`.

**Version 0.1.2**

* Moved most strings to use `core::syntax`.
* Fixed issues with multiple documentation strings.
* Fixed issue with blank documentation traits.
* Fixed grammar issue with empty trait bodies.

**Version 0.1.1**

* Able to parse basic examples.

**Version 0.1.0**

* First release.
* Able to write the example weather service, constructed using the builder API.

## TODO

1. Need to have a parser.
