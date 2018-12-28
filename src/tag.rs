use crate::*;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Adds metadata to a single tag that is used by the
/// Operation Object. It is not mandatory to have a
/// Tag Object per tag defined in the Operation Object instances.
pub struct Tag {
    /// REQUIRED. The name of the tag.
    pub name: String,
    /// A short description for the tag.
    /// CommonMark syntax MAY be used for rich text representation.
    pub description: Option<String>,
    /// Additional external documentation for this tag.
    #[serde(rename = "externalDocs")]
    pub external_docs: Option<ExternalDocumentation>,
}
