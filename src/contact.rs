use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Contact information for the exposed API.
pub struct Contact {
    /// The identifying name of the contact person/organization.
    pub name: Option<String>,
    /// The URL pointing to the contact information.
    /// MUST be in the format of a URL.
    pub url: Option<String>,
    /// The email address of the contact person/organization.
    /// MUST be in the format of an email address.
    pub email: Option<String>,
}
