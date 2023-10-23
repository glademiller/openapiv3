use crate::*;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// Each Media Type Object provides schema and examples for the media type
/// identified by its key.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MediaType {
    /// The schema defining the content of the request, response, or parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<ReferenceOr<Schema>>,
    /// Example of the media type. The example object SHOULD be in the correct
    /// format as specified by the media type. The example field is mutually
    /// exclusive of the examples field. Furthermore, if referencing a schema
    /// which contains an example, the example value SHALL override the example
    /// provided by the schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<serde_json::Value>,
    /// Examples of the media type. Each example object SHOULD match the media
    /// type and specified schema if present. The examples field is mutually
    /// exclusive of the example field. Furthermore, if referencing a schema
    /// which contains an example, the examples value SHALL override the
    /// example provided by the schema.
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub examples: IndexMap<String, ReferenceOr<Example>>,
    /// A map between a property name and its encoding information. The key,
    /// being the property name, MUST exist in the schema as a property. The
    /// encoding object SHALL only apply to requestBody objects when the media
    /// type is multipart or application/x-www-form-urlencoded.
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub encoding: IndexMap<String, Encoding>,

    /// Inline extensions to this object.
    #[serde(flatten, deserialize_with = "crate::util::deserialize_extensions")]
    pub extensions: IndexMap<String, serde_json::Value>,
}
