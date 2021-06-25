# Atelier: crate cargo_atelier

A cargo command for using [Smithy](https://github.com/awslabs/smithy) models as a part of a build process.

[![crates.io](https://img.shields.io/crates/v/cargo_atelier.svg)](https://crates.io/crates/cargo_atelier)
[![docs.rs](https://docs.rs/cargo_atelier/badge.svg)](https://docs.rs/cargo_atelier)

# Usage

```bash
> cargo atelier --help
cargo-atelier 0.2.2
Tools for the Smithy IDL.

USAGE:
    cargo-atelier [FLAGS] <SUBCOMMAND>

FLAGS:
    -h, --help        Prints help information
    -n, --no-color    Turn off color in the output
    -V, --version     Prints version information
    -v, --verbose     The level of logging to perform; from off to trace

SUBCOMMANDS:
    convert     Convert model from one representation to another
    document    Create human-readable documentation from a model
    help        Prints this message or the help of the given subcommand(s)
    lint        Run standard linter rules on a model file
    validate    Run standard validators on a model file
```

Both the lint and validate commands use a common mechanism for printing results and will by default print using a 
colorized output. As different linter and validation rules can be used the _reported by_ row informs you which rule-set
has determined the error.

The document command creates documentation from the model, relying on specific traits for text and the prelude traits
for some semantic properties. It used the [somedoc](https://crates.io/crates/somedoc) crate to do the formatting and
so the output format specified in this tool can select any of the formats supported by somedoc.

# Example Lint

For the following badly formatted Smithy file, in `test-models/lint-test.smithy`.

```text
namespace org.example.smithy

@ThisIsNotAGoodName
structure thisIsMyStructure {
    lower: String,
    Upper: String,
    someJSONThing: someUnknownShape,
    OK: Boolean
}

string someUnknownShape

@trait
structure ThisIsNotAGoodName {}
```

The following issues will be output when the linter runs.

```bash
> cargo atelier lint -i test-models/lint-test.smithy -r smithy

[info] Shape names should conform to UpperCamelCase, i.e. ThisIsMyStructure
	Reported by NamingConventions on/for element `thisIsMyStructure`.

[info] Trait names should conform to lowerCamelCase, i.e. thisIsNotAGoodName
	Reported by NamingConventions on/for element `ThisIsNotAGoodName`.

[info] Member names should conform to lowerCamelCase, i.e. ok
	Reported by NamingConventions on/for element `thisIsMyStructure$OK`.

[info] Member name 'OK' appears to contain a known acronym, consider renaming i.e. ok
	Reported by NamingConventions on/for element `thisIsMyStructure`.

[info] Member names should conform to lowerCamelCase, i.e. someJsonThing
	Reported by NamingConventions on/for element `thisIsMyStructure$someJSONThing`.

[info] Member name 'someJSONThing' appears to contain a known acronym, consider renaming i.e. Json
	Reported by NamingConventions on/for element `thisIsMyStructure`.

[info] Shape names should conform to UpperCamelCase, i.e. SomeUnknownShape
	Reported by NamingConventions on/for element `someUnknownShape`.

[info] Member names should conform to lowerCamelCase, i.e. upper
	Reported by NamingConventions on/for element `thisIsMyStructure$Upper`.
```

# Example Validate

For the following erroneous Smithy file, in `test-models/validation-test.smithy`.

```text
namespace org.example.smithy

structure MyStructure {
    known: String,
    wrongType: SomeOperation,
}

operation SomeOperation {
    input: SomeService
}

service SomeService {
    version: "1.0",
    operations: [MyStructure]
}
```

The following issues will be output when the validation runs.

```bash
> cargo atelier validate -i test-models/validation-test.smithy -r smithy

[error] Structure member may not refer to a service, operation, resource or apply.
	Reported by CorrectTypeReferences on/for element `MyStructure$wrongType`.

[warning] Structure member's type (smithy.api#NotString) cannot be resolved to a shape in this model.
	Reported by CorrectTypeReferences on/for element `MyStructure$unknown`.

[error] Service operation must be an operation.
	Reported by CorrectTypeReferences on/for element `SomeService`.

[error] Operation input may not refer to a service, operation, resource or apply.
	Reported by CorrectTypeReferences on/for element `SomeOperation`.
```

# Example Documentation

```bash
> cargo atelier document -i test-models/lint-test.smithy -w xwiki
{{comment}}
title: Smithy Model
{{/comment}}


Smith Version: 1.0

= Namespace org.example.smithy =

(% id="shape:ThisIsNotAGoodName" %) == ThisIsNotAGoodName (structure) ==

|=Trait|=Value|
|Is Trait|##true##|


(% id="shape:someUnknownShape" %) == someUnknownShape (string) ==

(% id="shape:thisIsMyStructure" %) == thisIsMyStructure (structure) ==

=== Members ===

* ##Upper##: ##smithy.api#String##
* ##lower##: ##smithy.api#String##
* ##OK##: ##smithy.api#Boolean##
* ##someJSONThing##: [[someUnknownShape>>.||anchor=shape:someUnknownShape]]
```

## Changes

**Version 0.2.5**

* Implemented [cargo-atelier's lint and validate should work on models built from multiple namespace
  files](https://github.com/johnstonskj/rust-atelier/issues/27) using the model assembler.
  * Updated all commands to take a list of file paths as input, the assembler will figure out the file types.
  * Updated error reporting.

**Version 0.2.4**

* Change to lib API.

**Version 0.2.3**

* Fixed: bumped somedoc dependency version.

**Version 0.2.2**

* Made new command for documentation writer; this takes the `somedoc::write::OutputFormat` to select format.

**Version 0.2.1**

* Changes based on new core traits `HasIdentity` and `HasTraits`.
* Added documentation writer.

**Version 0.1.3**

* Supporting colorized output.
* Added example files for lint/validate tests. 
* Added examples here.

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
