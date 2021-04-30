# Atelier: crate atelier_assembler

This crate provides the model assembly capability, to merge files into a single in-memory `Model`. 

[![crates.io](https://img.shields.io/crates/v/atelier_assembler.svg)](https://crates.io/crates/atelier_assembler)
[![docs.rs](https://docs.rs/atelier_assembler/badge.svg)](https://docs.rs/atelier_assembler)

A tool can add files one-by-one, or from a directory, and then process them all into a single model. This
implementation understands the different registered file extensions so that it can read files
in different representations and assemble them seamlessly.

For more information, see [the Rust Atelier book](https://rust-atelier.dev/using/assembly.html).

## Changes

**Version 0.1.0**

* Extracted from the atelier_lib crate.

