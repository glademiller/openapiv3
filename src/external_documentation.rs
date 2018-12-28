use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Allows referencing an external resource for extended documentation.
pub struct ExternalDocumentation {
    /// A short description of the target documentation.
    /// CommonMark syntax MAY be used for rich text representation.
    pub description: Option<String>,
    /// REQUIRED. The URL for the target documentation.
    /// Value MUST be in the format of a URL.
    pub url: String,
}
