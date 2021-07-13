# Extending Atelier

The core Atelier functionality contains a number of extension points, usually in the way of traits that can be
implemented by other clients.

1. Adding an [Artifact Representation](file_io.md) allows the reading and/or writing of different file formats.
1. Adding a [Linter](linter.md) allows for the creation of custom lint rules. 
1. Adding a [Validator](validator.md) allows for the creation of custom validation rules.
1. Adding a [Model Transformation](transformer.md) allows for the creation of model-to-model transformations.

Unlike the Java implementation Rust does not have a dynamic discovery mechanism, so additional linters and 
validators are not automatically added at runtime. This means that custom implementations cannot be used without 
additional work in existing tools such as [cargo-atelier](../using/cargo.md).