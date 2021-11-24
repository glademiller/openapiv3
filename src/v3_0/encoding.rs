use crate::{util::*, v3_0::*};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// A single encoding definition applied to a single schema property.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Encoding {
    /// The Content-Type for encoding a specific property.
    /// Default value depends on the property type: for string
    /// with format being binary – application/octet-stream;
    /// for other primitive types – text/plain;
    /// for object - application/json;
    /// for array – the default is defined based on the inner type.
    /// The value can be a specific media type (e.g. application/json),
    /// a wildcard media type (e.g. image/*), or a comma-separated list of the
    /// two types.
    pub content_type: Option<String>,
    /// A map allowing additional information to be provided as headers,
    /// for example Content-Disposition. Content-Type is described separately
    /// and SHALL be ignored in this section. This property SHALL be ignored
    /// if the request body media type is not a multipart.
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub headers: IndexMap<String, ReferenceOr<Header>>,
    /// Describes how a specific property value will be serialized depending
    /// on its type. See Parameter Object for details on the style property.
    /// The behavior follows the same values as query parameters, including
    /// default values. This property SHALL be ignored if the request body
    /// media type is not application/x-www-form-urlencoded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<QueryStyle>,
    /// When this is true, property values of type array or object generate
    /// separate parameters for each value of the array, or key-value-pair
    /// of the map. For other types of properties this property has no effect.
    /// When style is form, the default value is true.
    /// For all other styles, the default value is false. This property
    /// SHALL be ignored if the request body media type is
    /// not application/x-www-form-urlencoded.
    ///
    /// In this Library this value defaults to false always despite the
    /// specification.
    #[serde(default, skip_serializing_if = "is_false")]
    pub explode: bool,
    /// Determines whether the parameter value SHOULD allow reserved characters,
    /// as defined by RFC3986 :/?#[]@!$&'()*+,;= to be included without
    /// percent-encoding. The default value is false. This property SHALL be
    /// ignored if the request body media type is not
    /// application/x-www-form-urlencoded.
    #[serde(default, skip_serializing_if = "is_false")]
    pub allow_reserved: bool,
    /// Inline extensions to this object.
    #[serde(flatten, deserialize_with = "crate::util::deserialize_extensions")]
    pub extensions: IndexMap<String, serde_json::Value>,
}
