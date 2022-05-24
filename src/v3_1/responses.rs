use std::marker::PhantomData;

use crate::{util::*, v3_1::*};
use indexmap::IndexMap;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Responses {
    /// The documentation of responses other than the ones declared
    /// for specific HTTP response codes. Use this field to cover
    /// undeclared responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<ReferenceOr<Response>>,
    /// Any HTTP status code can be used as the property name,
    /// but only one property per code, to describe the expected
    /// response for that HTTP status code. This field MUST be enclosed in
    /// quotation marks (for example, "200") for compatibility between
    /// JSON and YAML. To define a range of response codes, this field
    /// MAY contain the uppercase wildcard character X. For example,
    /// 2XX represents all response codes between [200-299]. The following
    /// range definitions are allowed: 1XX, 2XX, 3XX, 4XX, and 5XX.
    /// If a response range is defined using an explicit code, the
    /// explicit code definition takes precedence over the range
    /// definition for that code.
    #[serde(flatten, deserialize_with = "deserialize_responses")]
    pub responses: IndexMap<StatusCode, ReferenceOr<Response>>,
    /// Inline extensions to this object.
    #[serde(flatten, deserialize_with = "crate::util::deserialize_extensions")]
    pub extensions: IndexMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Response {
    /// REQUIRED. A description of the response.
    /// CommonMark syntax MAY be used for rich text representation.
    pub description: String,

    /// Maps a header name to its definition.
    /// RFC7230 states header names are case insensitive.
    /// If a response header is defined with the name "Content-Type",
    /// it SHALL be ignored.
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub headers: IndexMap<String, ReferenceOr<Header>>,

    /// A map containing descriptions of potential response payloads.
    /// The key is a media type or media type range and the value
    /// describes it. For responses that match multiple keys,
    /// only the most specific key is applicable. e.g. text/plain
    /// overrides text/*
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub content: IndexMap<String, MediaType>,

    /// A map of operations links that can be followed from the response.
    /// The key of the map is a short name for the link, following
    /// the naming constraints of the names for Component Objects.
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub links: IndexMap<String, ReferenceOr<Link>>,

    /// Inline extensions to this object.
    #[serde(flatten, deserialize_with = "crate::util::deserialize_extensions")]
    pub extensions: IndexMap<String, serde_json::Value>,
}

fn deserialize_responses<'de, D>(
    deserializer: D,
) -> Result<IndexMap<StatusCode, ReferenceOr<Response>>, D::Error>
where
    D: Deserializer<'de>,
{
    // We rely on the result of StatusCode::deserialize to act as our
    // predicate; it will succeed only for status code.
    deserializer.deserialize_map(PredicateVisitor(|_: &StatusCode| true, PhantomData))
}

#[cfg(feature = "conversions")]
use crate::v3_0;

#[cfg(feature = "conversions")]
impl From<v3_0::Responses> for Responses {
    fn from(r: v3_0::Responses) -> Self {
        Responses {
            default: r.default.map(|v| ReferenceOr::from_v3_0(v)),
            responses: r
                .responses
                .into_iter()
                .map(|(k, v)| (k.into(), ReferenceOr::from_v3_0(v)))
                .collect(),
            extensions: r.extensions,
        }
    }
}

#[cfg(feature = "conversions")]
impl From<v3_0::Response> for Response {
    fn from(r: v3_0::Response) -> Self {
        Response {
            description: r.description,
            headers: r
                .headers
                .into_iter()
                .map(|(k, v)| (k, ReferenceOr::from_v3_0(v)))
                .collect(),
            content: r.content.into_iter().map(|(k, v)| (k, v.into())).collect(),
            links: r
                .links
                .into_iter()
                .map(|(k, v)| (k, ReferenceOr::from_v3_0(v)))
                .collect(),
            extensions: r.extensions,
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::v3_1::{ReferenceOr, Response, Responses, StatusCode};

    #[test]
    fn test_responses() {
        let responses = serde_json::from_str::<Responses>(
            r#"{
            "404": {
                "description": "xxx"
            },
            "x-foo": "bar",
            "ignored": "wat"
         }"#,
        )
        .unwrap();

        assert_eq!(
            responses.responses.get(&StatusCode::Code(404)),
            Some(&ReferenceOr::Item(Response {
                description: "xxx".to_string(),
                ..Default::default()
            }))
        );
        assert_eq!(responses.extensions.get("x-foo"), Some(&json!("bar")));
    }
}
