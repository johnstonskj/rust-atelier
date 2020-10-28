# Atelier: crate atelier_rdf

Provides the ability to read and write [Smithy](https://github.com/awslabs/smithy) models to and from the W3C's 
[Resource Description Framework (RDF)](https://www.w3.org/RDF/).

[![crates.io](https://img.shields.io/crates/v/atelier_rdf.svg)](https://crates.io/crates/atelier_rdf)
[![docs.rs](https://docs.rs/atelier_rdf/badge.svg)](https://docs.rs/atelier_rdf)

This crate provides a common mapping between the Smithy semantic model, and an RDF vocabulary. This mapping can be used to 
serialize the resulting RDF graph to one of the standard RDF representations, or store in a graph store. The mapping
allows for models to be augmented by additional facts in the RDF graph and allows for inference over the model in it's
RDF form.

## Changes

**Version 0.1.2**

* Using latest `rdftk_iri`, which involved changes. 
* Also, removed `SmithUrn` type and use `IRIRef` directly instead.

**Version 0.1.1**

* Provide a common `Model` to RDF mapping; document this in the `model` module, and implement `model::model_to_rdf`.

**Version 0.1.0**

* Provides `SmithyUrn` type as a URI for RDF usage.

## TODO

1. Write to RDF
1. Parse from RDF
