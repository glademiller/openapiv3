mod util;
pub mod v3_0;
pub mod v3_1;

pub use self::util::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "openapi")]
pub enum OpenAPI {
    #[serde(rename = "3.0.0", alias = "3.0.1", alias = "3.0.2", alias = "3.0.3")]
    Version30(v3_0::OpenAPI),
    #[serde(rename = "3.1.0")]
    Version31(v3_1::OpenAPI),
}
