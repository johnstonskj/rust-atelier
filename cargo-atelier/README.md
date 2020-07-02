# Atelier: crate cargo_atelier

A cargo command for using [Smithy](https://github.com/awslabs/smithy) models as a part of a build process.

[![crates.io](https://img.shields.io/crates/v/cargo_atelier.svg)](https://crates.io/crates/cargo_atelier)
[![docs.rs](https://docs.rs/cargo_atelier/badge.svg)](https://docs.rs/cargo_atelier)

# Usage

```bash
> cargo atelier --help
cargo-atelier 0.1.1
Tools for the Smithy IDL.

USAGE:
    cargo-atelier [FLAGS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    The level of logging to perform; from off to trace

SUBCOMMANDS:
    convert     Convert file from one format to another
    help        Prints this message or the help of the given subcommand(s)
    lint        Run standard linter rules on a model file
    validate    Run standard validators on a model file
```

## Changes

**Version 0.1.2**

* Updated library dependency to get the latest Smithy reader.

**Version 0.1.1**

* Linter and Validator commands working
* Convert from json/smithy to json/smithy/uml

**Version 0.1.0**

* Placeholder release.

## TODO

1. Validator.
1. Code generator.
