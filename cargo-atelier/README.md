# Atelier: crate cargo_atelier

A cargo command for using [Smithy](https://github.com/awslabs/smithy) models as a part of a build process.

[![crates.io](https://img.shields.io/crates/v/cargo_atelier.svg)](https://crates.io/crates/cargo_atelier)
[![docs.rs](https://docs.rs/cargo_atelier/badge.svg)](https://docs.rs/cargo_atelier)

# Usage

```bash
> cargo atelier --help
cargo-atelier 0.1.2
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
    help        Prints this message or the help of the given subcommand(s)
    lint        Run standard linter rules on a model file
    validate    Run standard validators on a model file
```

Both the lint and validate commands use a common mechanism for printing results and will by default print using a 
colorized output. As different linter and validation rules can be used the _reported by_ row informs you which rule-set
has determined the error.

# Example Lint

For the following badly formatted Smithy file,

```text
namespace org.example.smithy

@ThisIsNotAGoodName
structure thisIsMyStructure {
    lower: String,
    Upper: String,
    someJSONThing: someUnknownShape,
    OK: Boolean
}
```

The following issues will be output when the linter is run.

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

For the following erroneous Smithy file,

```text
namespace org.example.smithy

@unknownTrait
structure MyStructure {
    known: String,
    unknown: NotString,
    wrongType: SomeOperation,
}

operation SomeOperation {
    input: SomeService
}

service SomeService {
    operations: [MyStructure]
```

The following issues will be output when the validation is run.

```bash
> cargo atelier validate -i test-models/validation-test.smithy -r smithy

[error] Shape, or member, has a trait that refers to an unknown identifier: unknownTrait
	Reported by NoOrphanedReferences on/for element `MyStructure`.

[error] Shape, or member, refers to an unknown identifier: NotString
	Reported by NoOrphanedReferences on/for element `MyStructure$unknown`.

[error] Structure member may not refer to a service, operation, resource or apply.
	Reported by CorrectTypeReferences on/for element `MyStructure$wrongType`.

[warning] Structure member's type (smithy.api#NotString) cannot be resolved to a shape in this model.
	Reported by CorrectTypeReferences on/for element `MyStructure$unknown`.

[error] Service operation must be an operation.
	Reported by CorrectTypeReferences on/for element `SomeService`.

[error] Operation input may not refer to a service, operation, resource or apply.
	Reported by CorrectTypeReferences on/for element `SomeOperation`.
```

## Changes

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
