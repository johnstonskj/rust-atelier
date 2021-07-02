# Adding an Artifact Representation


# Model Writer

The example below is pretty much the implementation of the `atelier_core::io::debug` module, it writes the model
using the `Debug` implementation associated with those objects.

```rust
use atelier_core::io::ModelWriter;
use atelier_core::model::Model;
use atelier_core::error::Result as ModelResult;
use std::io::Write;

#[derive(Debug)]
pub struct FooWriter {}

impl Default for FooWriter {
    fn default() -> Self {
        Self {}
    }
}

impl ModelWriter for FooWriter {
    fn write(&mut self, w: &mut impl Write, model: &Model) -> ModelResult<()> {
        todo!()
    }
}
```

## Add transform function

```rust
pub fn model_to_foo(source: &Model) -> Result<Foo> {
    todo!()
}
```

```rust
impl ModelWriter for FooWriter {
    fn write(&mut self, w: &mut impl Write, model: &Model) -> ModelResult<()> {
        let foo = model_to_foo(model)?;
        write!(w, "{}", foo)?;
        Ok(())
    }
}
```

# Model Reader

```rust
use atelier_core::io::ModelReader;
use atelier_core::model::Model;
use atelier_core::error::Result as ModelResult;
use std::io::Write;

#[derive(Debug)]
pub struct FooReader {}

impl Default for FooReader {
    fn default() -> Self {
        Self {}
    }
}

impl ModelReader for FooReader {
    fn read(&mut self, r: &mut impl Read) -> ModelResult<Model> {
        todo!()
    }
}
```

## Add transform function

```rust
pub fn pub fn parse_model(r: &mut impl Read) -> ModelResult<Model> {
    todo!()
}
```

```rust
impl ModelReader for FooReader {
    fn read(&mut self, r: &mut impl Read) -> ModelResult<Model> {
        parse_model(r)
    }
}
```
