use serde::{Deserialize, Serialize};

/// License information for the exposed API.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct License {
    /// REQUIRED. The license name used for the API.
    pub name: String,
    /// A URL to the license used for the API. MUST be in the format of a URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
