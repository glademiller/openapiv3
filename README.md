[Crate](https://crates.io/crates/openapiv3-extended) | [Github](https://github.com/kurtbuilds/openapiv3-extended)

> **NOTE**: This is a fork of https://github.com/glademiller/openapiv3. It builds on it by adding many methods
> for creating and modifying OpenAPI specs with code, and adding v2 support. There are a few minor breaking changes
> where I found incompatibility with the spec, but the compiler should trivially help you resolve these.

# Installation

You can declare it directly:

```toml
[dependencies]
# Note that it's still imported as `use openapiv3::{...}`, despite the crate name being `openapiv3-extended`
openapiv3-extended = "..."
```

Or you can declare it while explicitly naming the imported package.

```toml
[dependencies]
# This declaration is equivalent, and is more explicit about the package name.
openapiv3 = { version = "...", package="openapiv3-extended" }
```

# OpenAPI v3 ![example workflow](https://github.com/glademiller/openapiv3/actions/workflows/rust.yml/badge.svg)

This crate provides data structures that represent the [OpenAPI v3.0.x specification](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.3.md).
Note this does not cover OpenAPI v3.1 (yet) which was an incompatible change.

# Usage

Here is a basic example: 

```rust
use serde_json;
use openapiv3::OpenAPI;

fn main() {
    let data = include_str!("openapi.json");
    let openapi: OpenAPI = serde_json::from_str(data).unwrap();
    println!("{:?}", openapi);
}
```

You can use this crate to upgrade a Swagger 2.0 spec to OpenAPI 3.0.x. To support v2, enable the `v2`  feature.

```rust
// [dependencies]
// openapiv3 = { version = "2.1", features = ["v2"] }
use openapiv3::VersionedOpenAPI;

fn main() {
    let data = include_str!("swagger.json");
    let openapi: VersionedOpenAPI = serde_json::from_str(data).unwrap();
    println!("{:?}", openapi);
    let openapi: OpenAPI = openapi.upgrade(); // version 3.0
    println!("{:?}", openapi);
}
```

## Goals
* Provide a deserialization for the specification that maps cleanly to Rust enums etc.

## Non Goals
* Deserialization and subsequent re-serialization are 100% the same.
    * Some defaults show-up when serializing that may not have existed in the input.

## Issues
Schemas without a type will end up as any data type as per the specification and can have any parameters of any schema type. Some Open API documents don't include the type parameter it would be nice to try to derive the type but the crate as of right now meets my needs.