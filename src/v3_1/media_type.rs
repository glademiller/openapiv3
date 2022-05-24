use crate::v3_1::*;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MediaType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<SchemaObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub examples: IndexMap<String, ReferenceOr<Example>>,
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub encoding: IndexMap<String, Encoding>,
    /// Inline extensions to this object.
    #[serde(flatten, deserialize_with = "crate::util::deserialize_extensions")]
    pub extensions: IndexMap<String, serde_json::Value>,
}

#[cfg(feature = "conversions")]
use crate::v3_0;

#[cfg(feature = "conversions")]
impl From<v3_0::MediaType> for MediaType {
    fn from(m: v3_0::MediaType) -> Self {
        MediaType {
            schema: m.schema.and_then(|v| v.into_item().map(Into::into)),
            example: m.example,
            examples: m
                .examples
                .into_iter()
                .map(|(k, v)| (k, ReferenceOr::from_v3_0(v)))
                .collect(),
            encoding: m.encoding.into_iter().map(|(k, v)| (k, v.into())).collect(),
            extensions: m.extensions,
        }
    }
}
