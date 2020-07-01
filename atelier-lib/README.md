# Atelier: crate atelier_lib

A combined crate for all Atelier sub-crates incorporated as features. Atelier is a Rust native
library, and tools, for the AWS [Smithy](https://github.com/awslabs/smithy) Interface Definition
Language.

[![crates.io](https://img.shields.io/crates/v/atelier_lib.svg)](https://crates.io/crates/atelier_lib)
[![docs.rs](https://docs.rs/atelier_lib/badge.svg)](https://docs.rs/atelier_lib)


The aim of this crate is to provide a single client interface over a set of crates that provide
different Atelier capabilities. The following table shows the mapping from individual crate to the 
combined module path in this library. The column _Default_ indicates those that are included in the 
default feature, although the core will be included regardless of any feature selection.

| Feature name | Default | Individual crate  | Target module path                | Purpose                                               |
|--------------|---------|-------------------|-----------------------------------|-------------------------------------------------------|
| N/A          | **Yes** | `atelier_core`    | `atelier_lib::core`               | Core models only.                                     |
| "json"       | No      | `atelier_json`    | `atelier_lib::format::json`       | Reading and Writing JSON AST representation.          |
| "openapi"    | No      | `atelier_openapi` | `atelier_lib::format::openapi`    | Reading and Writing OpenAPI representations.          |
| "smithy"     | Yes     | `atelier_smithy`  | `atelier_lib::format::smithy`     | Reading and Writing the Smithy native representation. |

## Changes


**Version 0.1.2**

* New dependency versions:
  * core: v0.1.3
  * json: v0.1.2
  * smithy: v0.1.2

**Version 0.1.1**

* All re-exports configured using feature flags.

