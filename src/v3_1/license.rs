use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// License information for the exposed API.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct License {
    /// REQUIRED. The license name used for the API.
    pub name: String,
    /// An [SPDX](https://spdx.org/spdx-specification-21-web-version#h.jxpfx0ykyb60) license expression for the API. The `identifier` field is mutually exclusive of the `url` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// A URL to the license used for the API. This MUST be in the form of a
    /// URL. The `url` field is mutually exclusive of the `identifier` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Inline extensions to this object.
    #[serde(flatten, deserialize_with = "crate::util::deserialize_extensions")]
    pub extensions: IndexMap<String, serde_json::Value>,
}

#[cfg(feature = "conversions")]
use crate::v3_0;

#[cfg(feature = "conversions")]
impl From<v3_0::License> for License {
    fn from(i: v3_0::License) -> Self {
        License {
            name: i.name,
            identifier: None,
            url: i.url,
            extensions: i.extensions,
        }
    }
}
