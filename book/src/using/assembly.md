# Model Assembly


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



```rust
use atelier_assembler::ModelAssembler;
use atelier_core::error::Result;
use atelier_core::model::Model;
use std::convert::TryFrom;

let mut assembler = ModelAssembler::default();

assembler.push_str("tests/good");

let model: Result<Model> = Model::try_from(assembler);
```
