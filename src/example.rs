use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Example {
    /// Short description for the example.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// Long description for the example.
    /// CommonMark syntax MAY be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Embedded literal example. The value field and externalValue
    /// field are mutually exclusive. To represent examples of
    /// media types that cannot naturally represented in JSON or YAML,
    /// use a string value to contain the example, escaping where necessary.
    #[serde(rename = "externalValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_value: Option<String>,
}
