use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Example {
    #[serde(skip_serializing_if = "Option::is_none")]
    summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    // value: @TODO how to handle this
    #[serde(rename = "externalValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    external_value: Option<String>,
}
