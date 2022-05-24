use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// Allows referencing an external resource for extended documentation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ExternalDocumentation {
    /// A description of the target documentation.
    /// CommonMark syntax MAY be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// REQUIRED. The URL for the target documentation.
    /// This MUST be in the format of a URL.
    pub url: String,
    /// Inline extensions to this object.
    #[serde(flatten, deserialize_with = "crate::util::deserialize_extensions")]
    pub extensions: IndexMap<String, serde_json::Value>,
}

#[cfg(feature = "conversions")]
use crate::v3_0;

#[cfg(feature = "conversions")]
impl From<v3_0::ExternalDocumentation> for ExternalDocumentation {
    fn from(e: v3_0::ExternalDocumentation) -> Self {
        ExternalDocumentation {
            description: e.description,
            url: e.url,
            extensions: e.extensions,
        }
    }
}
