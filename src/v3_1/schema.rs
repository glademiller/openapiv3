#![allow(clippy::large_enum_variant)]
use crate::v3_1::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SchemaObject {
    #[serde(flatten)]
    pub json_schema: schemars::schema::Schema,
    /// Additional external documentation for this schema.
    #[serde(rename = "externalDocs")]
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
}

#[cfg(feature = "conversions")]
mod conversions {
    use crate::v3_0;
    use serde_json::Value as JValue;
    impl From<v3_0::Schema> for super::SchemaObject {
        fn from(s: v3_0::Schema) -> Self {
            let oldval = serde_json::to_value(&s).expect("Convert Schema to serde_json::Value");
            serde_json::from_value(oldval)
                .expect("Convert Openapi v3.0.0 Schema to Openapi V3.1.0 Schema")
        }
    }
}
