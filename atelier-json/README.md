# Atelier: crate atelier_json

Provides the ability to read and write [Smithy](https://github.com/awslabs/smithy) models in the JSON AST representation.

[![crates.io](https://img.shields.io/crates/v/atelier_json.svg)](https://crates.io/crates/atelier_json)
[![docs.rs](https://docs.rs/atelier_json/badge.svg)](https://docs.rs/atelier_json)

TBD

## Changes

**Version 0.2.0**

* Major refactor after agreement on the separation of semantic model with Smithy team.

**Version 0.1.4**

* API changes for `ModelReader` and `ModelWriter`.
  * removed `representation` method
  * added `FILE_EXTENSION` constant.

**Version 0.1.3**

* Changes to the core API around `ModelReader` and `ModelWriter`.

**Version 0.1.2**

* Fixed issue in ID->Shape mapping.
* Fixed issue with missing `members` on structure and union shapes.
* Added a set of JSON files from the AWS repository as parser tests.
* Renamed integration test names for easier reporting. 

**Version 0.1.1**

* Able to read a JSON representation.

**Version 0.1.0**

* First release.
* Able to write the example weather service, constructed using the builder API.

## TODO

None.