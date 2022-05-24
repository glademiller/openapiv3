use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// Contact information for the exposed API.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Contact {
    /// The identifying name of the contact person/organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The URL pointing to the contact information.
    /// This MUST be in the format of a URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The email address of the contact person/organization.
    /// This MUST be in the format of an email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Inline extensions to this object.
    #[serde(flatten, deserialize_with = "crate::util::deserialize_extensions")]
    pub extensions: IndexMap<String, serde_json::Value>,
}

#[cfg(feature = "conversions")]
use crate::v3_0;

#[cfg(feature = "conversions")]
impl From<v3_0::Contact> for Contact {
    fn from(c: v3_0::Contact) -> Self {
        Contact {
            name: c.name,
            url: c.url,
            email: c.email,
            extensions: c.extensions,
        }
    }
}
