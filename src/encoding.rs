use crate::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Encoding {
    content_type: String,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    headers: BTreeMap<String, ReferenceOr<Header>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<QueryStyle>,
    #[serde(default, skip_serializing_if = "is_false")]
    explode: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    allow_reserved: bool,
}
