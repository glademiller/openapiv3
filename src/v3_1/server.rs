use crate::v3_1::*;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// An object representing a Server.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Server {
    /// REQUIRED. A URL to the target host.
    /// This URL supports Server Variables and MAY be relative,
    /// to indicate that the host location is relative to the
    /// location where the OpenAPI document is being served.
    /// Variable substitutions will be made when a variable
    /// is named in {brackets}.
    pub url: String,
    /// An optional string describing the host designated
    /// by the URL. CommonMark syntax MAY be used for rich
    /// text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A map between a variable name and its value.
    /// The value is used for substitution in the server's URL template.
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub variables: IndexMap<String, ServerVariable>,
    /// Inline extensions to this object.
    #[serde(flatten, deserialize_with = "crate::util::deserialize_extensions")]
    pub extensions: IndexMap<String, serde_json::Value>,
}

#[cfg(feature = "conversions")]
use crate::v3_0;

#[cfg(feature = "conversions")]
impl From<v3_0::Server> for Server {
    fn from(s: v3_0::Server) -> Self {
        Server {
            url: s.url,
            description: s.description,
            variables: s
                .variables
                .map(|v| v.into_iter().map(|(k, v)| (k, v.into())).collect())
                .unwrap_or_default(),
            extensions: s.extensions,
        }
    }
}
