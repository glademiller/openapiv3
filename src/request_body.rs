use crate::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestBody {
    /// A brief description of the request body.
    /// This could contain examples of use.
    /// CommonMark syntax MAY be used for rich text representation.
    pub description: Option<String>,
    /// REQUIRED. The content of the request body.
    /// The key is a media type or media type range and
    /// the value describes it. For requests that match
    /// multiple keys, only the most specific key is applicable.
    ///  e.g. text/plain overrides text/*
    #[serde(default)]
    pub content: BTreeMap<String, MediaType>,
    /// Determines if the request body is required in the
    /// request. Defaults to false.
    #[serde(default)]
    pub required: bool,
}
