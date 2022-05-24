use crate::v3_1::*;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// Adds metadata to a single tag that is used by the
/// Operation Object. It is not mandatory to have a
/// Tag Object per tag defined in the Operation Object instances.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Tag {
    /// REQUIRED. The name of the tag.
    pub name: String,
    /// A description for the tag.
    /// CommonMark syntax MAY be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Additional external documentation for this tag.
    #[serde(rename = "externalDocs", skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentation>,
    /// Inline extensions to this object.
    #[serde(flatten, deserialize_with = "crate::util::deserialize_extensions")]
    pub extensions: IndexMap<String, serde_json::Value>,
}

#[cfg(feature = "conversions")]
use crate::v3_0;

#[cfg(feature = "conversions")]
impl From<v3_0::Tag> for Tag {
    fn from(t: v3_0::Tag) -> Self {
        Tag {
            name: t.name,
            description: t.description,
            external_docs: t.external_docs.map(Into::into),
            extensions: t.extensions,
        }
    }
}
