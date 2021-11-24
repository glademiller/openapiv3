use crate::v3_1::*;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Schema {
    #[serde(flatten)]
    pub json_schema: schemars::schema::Schema,
    /// Additional external documentation for this schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentation>,
    /// A free-form property to include an example of an instance for this
    /// schema. To represent examples that cannot be naturally represented in
    /// JSON or YAML, a string value can be used to contain the example with
    /// escaping where necessary. **Deprecated:** The `example` property has
    /// been deprecated in favor of the JSON Schema `examples` keyword. Use
    /// of `example` is discouraged, and later versions of this
    /// specification may remove it.
    pub example: Option<serde_json::Value>,
    /// Inline extensions to this object. Additional properties MAY omit the
    /// `x-` prefix within this object.
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}
