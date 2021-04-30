# Atelier: crate atelier_rdf

Provides the ability to read and write [Smithy](https://github.com/awslabs/smithy) models to and from the W3C's 
[Resource Description Framework (RDF)](https://www.w3.org/RDF/).

[![crates.io](https://img.shields.io/crates/v/atelier_rdf.svg)](https://crates.io/crates/atelier_rdf)
[![docs.rs](https://docs.rs/atelier_rdf/badge.svg)](https://docs.rs/atelier_rdf)

This crate provides a common mapping between the Smithy semantic model, and an RDF vocabulary. This mapping can be used to 
serialize the resulting RDF graph to one of the standard RDF representations, or store in a graph store. The mapping
allows for models to be augmented by additional facts in the RDF graph and allows for inference over the model in it's
RDF form.

# Example - Mapping

The following simply constructs an RDF Graph from a provided model.

```rust
use atelier_core::model::Model;
use atelier_rdf::model::model_to_rdf;

let model = make_model();
let rdf_graph = model_to_rdf(&model, None).unwrap();
```

# Example - Writer

This example writes the provided model in RDF's [Turtle](https://www.w3.org/TR/turtle/) serialization representation.

```rust
use atelier_core::model::Model;
use atelier_core::io::ModelWriter;
use atelier_rdf::writer::RdfWriter;
use std::io::stdout;

let model = make_model();
let mut writer = RdfWriter::default();
writer.write(&mut stdout(), &model).unwrap();
```

## Changes

**Version 0.1.6**

* Updated due to trait API changes in core.

**Version 0.1.5**

* New cleaner mapping to RDF.
* Moved detailed mapping docs to the book.
* Merged model module into reader and writer.

**Version 0.1.4**

* Added `ModelWriter` implementation
* Added more documentation to lib/module files and README
* Code optimization in `iri_to_shape`.

**Version 0.1.3**

* Changes based on new core traits `HasIdentity` and `HasTraits`.

**Version 0.1.2**

* Using latest `rdftk_iri`, which involved changes. 
* Also, removed `SmithUrn` type and use `IRIRef` directly instead.

**Version 0.1.1**

* Provide a common `Model` to RDF mapping; document this in the `model` module, and implement `model::model_to_rdf`.

**Version 0.1.0**

* Provides `SmithyUrn` type as a URI for RDF usage.
