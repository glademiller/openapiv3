use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Contact information for the exposed API.
pub struct Contact {
    /// The identifying name of the contact person/organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The URL pointing to the contact information.
    /// MUST be in the format of a URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The email address of the contact person/organization.
    /// MUST be in the format of an email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}
