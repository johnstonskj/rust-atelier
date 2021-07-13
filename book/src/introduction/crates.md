# Crate structure

The following figure shows the current set of crates and their dependencies. For most tools it is easiest to simply 
rely on the `lib` crate in the same way the `cargo` crate does in the figure below.

<a name="fig_1_8"></a>![Crates](img/atelier-crates.svg)
<div class="caption figure">1.8: Crates</div>

* **core**; contains the core model and framework elements such as the `ModelReader`, `ModelWriter` traits, and 
  `Action`, `Linter`, and `Validator` traits.
* **assembler**; provides the ability to load multiple files, in supported representations, and merge into a single 
  semantic model. The `ModelAssembler`, along with the `FileTypeRegistry` and `SearchPath` are intended to support tools
  that process models, not just files.
* **smithy**; contains implementations of the `ModelReader` and `ModelWriter` traits for the Smithy IDL representation.
* **json**; contains implementations of the `ModelReader` and `ModelWriter` traits for the JSON AST representation.
* **query**; _will_ contain the implementation of selector expressions as queries.
* **describe**; contains an implementation of the `ModelWriter` traits that generates formatted documentation.
* **rdf**; contains an implementation of the `ModelWriter` traits for an RDF representation.
* **openapi**; _will_ contain the transformation to open API.
* **lib**; a combined, single dependency, crate for clients that want to use a lot of the Atelier functionality.
* **cargo**; the cargo command for Smithy file processing.
