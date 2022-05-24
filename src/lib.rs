#![allow(clippy::large_enum_variant)]
mod util;
pub mod v3_0;
pub mod v3_1;

pub use schemars;
pub use v3_0::*;

pub mod versioned {
    pub use super::v3_0;
    pub use super::v3_1;
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    #[serde(tag = "openapi")]
    pub enum OpenApi {
        #[serde(rename = "3.0.0", alias = "3.0.1", alias = "3.0.2", alias = "3.0.3")]
        Version30(super::v3_0::OpenAPI),
        #[serde(rename = "3.1.0")]
        Version31(super::v3_1::OpenApi),
    }
}
