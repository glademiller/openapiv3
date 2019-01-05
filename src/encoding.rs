use crate::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Encoding {
    pub content_type: String,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub headers: BTreeMap<String, ReferenceOr<Header>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<QueryStyle>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub explode: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub allow_reserved: bool,
}
