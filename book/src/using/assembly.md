# Model Assembly

The model assembler is used to create a single semantic model from a set of model artifacts. The assembler uses a _file 
type registry_ to identify different representations and how to load them. Additionally, the assembler will process
paths specified in a typical _search path_ environment variable.

## Examples

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

The next example turns off the search path handling entirely (the `None` passed to the `new` function) and then
adds a single directory to the assembler.

```rust
use atelier_assembler::{FileTypeRegistry, ModelAssembler};
use atelier_core::error::Result;
use atelier_core::model::Model;
use std::convert::TryFrom;

let mut assembler = ModelAssembler::new(FileTypeRegistry::default(), None);

assembler.push_str("tests/good");

let model: Result<Model> = Model::try_from(assembler);
```
