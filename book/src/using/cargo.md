# Cargo Integration

The `cargo_atelier` crate provides a cargo sub-command for processing Smithy files, and is installed in the usual manner.

```bash
> cargo install cargo_atelier
```

To ensure this installed correctly, you can check the help.

```text
> cargo atelier --help
cargo-atelier 0.2.7
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

## Linter example

For the following badly formatted Smithy file, in `test-models/lint-test.smithy`.

```smithy
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

```text
> cargo atelier lint -i test-models/lint-test.smithy

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

## Validation example

For the following erroneous Smithy file, in `test-models/validation-test.smithy`.

```smithy
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

```text
> cargo atelier validate -i test-models/validation-test.smithy

[error] Structure member may not refer to a service, operation, resource or apply.
	Reported by CorrectTypeReferences on/for element `MyStructure$wrongType`.

[warning] Structure member's type (smithy.api#NotString) cannot be resolved to a shape in this model.
	Reported by CorrectTypeReferences on/for element `MyStructure$unknown`.

[error] Service operation must be an operation.
	Reported by CorrectTypeReferences on/for element `SomeService`.

[error] Operation input may not refer to a service, operation, resource or apply.
	Reported by CorrectTypeReferences on/for element `SomeOperation`.
```


# Parameters

Common parameters that may be included with any command.

* `-V`, `--version`; prints version information (and exits).
* `-h`, `--help`; prints help information (and exits).
* `-v`, `--verbose`; turn on more logging, the more times you add the parameter the more logging you get.
* `--no-color`; turn off color support.

The following parameters are supported for all file input. File input uses the
[`atelier_assembler`](https://github.com/johnstonskj/rust-atelier//atelier-assembler) crate to read multiple files and
support multiple file representations. By default, the model assembler does not use a search path to load files. However,
this can be changed with either the `-d` flag which will load any files found in the search path in the environment
variable `$SMITHY_PATH`. Alternatively the `-s` parameter provides the name of an environment variable to use instead
of `$SMITHY_PATH`.

* `-d`, `--default-search-env`; if set, the standard `SMITHY_PATH` environment variable will be used as a search path.
* `-i`, `--in-file <in-file>`;the name of a file to read, multiple files can be specified.
* `-s`, `--search-env <search-env>`; the name of an environment variable to use as a search path.

> 
> The following will process all files in the default environment variable, with local files prepended to the
> search path.
> 
> ```bash
> > export SMITHY_PATH=./src/models:$SMITHY_PATH
> > cargo atelier validate -d
> ```
> 
> The above can also be accomplished using `-d` and `-i` together.
> 
> ```bash
> > cargo atelier validate -d -i ./src/models
> ```
> 

The following parameters are supported for all file output.

* `-n`, `--namespace <namespace>`;a namespace to write, if the output format requires one.
* `-o`, `--out-file <out-file>`; the name of a file to write to or stdout.
* `-w`, `--write-format <write-format>`; the representation of the output file, the default is dependent on the command.
