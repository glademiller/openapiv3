# OpenAPI v3 &emsp;

This crate aims to provide data structures that represent the [Open API v3 specification](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.0.md).

## Example

```rust
use serde_json;
use openapiv3::OpenAPI;

fn main() {
    let data = include_str!("openapi.json");
    let openapi: OpenAPI = serde_json::from_str(data).expect("Could not deserialize input");
    println!("{:?}", openapi);
}
```

## Goals
* Provide a deserialization for the specification that maps cleanly to Rust enums etc.

## Non Goals
* Deserialization and subsequent re-serialization are 100% the same.
    * Some defaults show-up when serializing that may not have existed in the input.

## Issues
Schemas without a type will endup as any data type as per the specification and can have any parameters of any schema type. Some Open API documents don't include the type parameter it would be nice to try to derive the type but the crate as of right now meets my needs.

## Similar Crates
* [openapi](https://crates.io/crates/openapi)

## License

This crate is licensed under either of

    Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
    MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.