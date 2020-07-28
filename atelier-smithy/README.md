# Atelier: crate atelier_smithy

Provides the ability to read and write [Smithy](https://github.com/awslabs/smithy) models in its own native language representation.

[![crates.io](https://img.shields.io/crates/v/atelier_smithy.svg)](https://crates.io/crates/atelier_smithy)
[![docs.rs](https://docs.rs/atelier_smithy/badge.svg)](https://docs.rs/atelier_smithy)

TBD

## Changes

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
