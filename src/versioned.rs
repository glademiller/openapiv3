use crate as v3;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum VersionedOpenAPI {
    #[cfg(feature = "v2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2")))]
    V2(crate::v2::OpenAPI),
    V3(v3::OpenAPI),
}

impl VersionedOpenAPI {
    pub fn upgrade(self) -> v3::OpenAPI {
        pub use VersionedOpenAPI::*;
        match self {
            #[cfg(feature = "v2")]
            V2(v2) => v2.into(),
            V3(v3) => v3,
        }
    }
}