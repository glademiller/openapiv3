use crate::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// The Header Object follows the structure of the Parameter Object with the following changes:
///
/// 1) name MUST NOT be specified, it is given in the corresponding headers map.
/// 2) in MUST NOT be specified, it is implicitly in header.
/// 3) All traits that are affected by the location MUST be applicable to a location of header (for example, style).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    /// A brief description of the parameter. This could
    /// contain examples of use. CommonMark syntax MAY be
    /// used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub style: HeaderStyle,
    /// Determines whether this parameter is mandatory.
    /// If the parameter location is "path", this property
    /// is REQUIRED and its value MUST be true. Otherwise,
    /// the property MAY be included and its default value
    /// is false.
    #[serde(default, skip_serializing_if = "is_false")]
    pub required: bool,
    /// Specifies that a parameter is deprecated and SHOULD
    /// be transitioned out of usage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    #[serde(flatten)]
    pub format: ParameterSchemaOrContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub examples: BTreeMap<String, ReferenceOr<Example>>,
}
