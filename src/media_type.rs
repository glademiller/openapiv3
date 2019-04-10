use crate::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MediaType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<ReferenceOr<Schema>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub examples: BTreeMap<String, ReferenceOr<Example>>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub encoding: BTreeMap<String, Encoding>,
}
