use crate::{util::*, v3_1::*};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// Describes a single API operation on a path.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Operation {
    /// A list of tags for API documentation control.
    /// Tags can be used for logical grouping of operations
    /// by resources or any other qualifier.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    /// A short summary of what the operation does.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// A verbose explanation of the operation behavior.
    /// CommonMark syntax MAY be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Additional external documentation for this operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentation>,
    /// Unique string used to identify the operation.
    /// The id MUST be unique among all operations described in the API.
    /// Tools and libraries MAY use the operationId to uniquely identify
    /// an operation, therefore, it is RECOMMENDED to follow common
    /// programming naming conventions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    /// A list of parameters that are applicable for this operation.
    /// If a parameter is already defined at the Path Item, the new
    /// definition will override it but can never remove it.
    /// The list MUST NOT include duplicated parameters. A unique
    /// parameter is defined by a combination of a name and location.
    /// The list can use the Reference Object to link to parameters
    /// that are defined at the OpenAPI Object's components/parameters.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ReferenceOr<Parameter>>,
    /// The request body applicable for this operation.
    /// The requestBody is fully supported in HTTP methods
    /// where the HTTP 1.1 specification RFC7231 has explicitly
    /// defined semantics for request bodies. In other cases where
    /// the HTTP spec is vague (such as
    /// [GET](https://tools.ietf.org/html/rfc7231#section-4.3.1),
    /// [HEAD](https://tools.ietf.org/html/rfc7231#section-4.3.2) and
    /// [DELETE](https://tools.ietf.org/html/rfc7231#section-4.3.5)),
    /// requestBody is permitted but does not have well-defined semantics and
    /// SHOULD be avoided if possible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_body: Option<ReferenceOr<RequestBody>>,
    /// The list of possible responses as they are returned
    /// from executing this operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responses: Option<Responses>,
    /// Declares this operation to be deprecated.Default value is false.
    #[serde(default, skip_serializing_if = "is_false")]
    pub deprecated: bool,
    /// A declaration of which security mechanisms can be used for this
    /// operation. The list of values includes alternative security
    /// requirement objects that can be used. Only one of the security
    /// requirement objects need to be satisfied to authorize a request.
    /// This definition overrides any declared top-level security. To remove
    /// a top-level security declaration, an empty array can be used.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub security: Vec<SecurityRequirement>,
    /// An alternative server array to service this operation.
    /// If an alternative server object is specified at the
    /// Path Item Object or Root level, it will be overridden by this value.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub servers: Vec<Server>,
    /// Inline extensions to this object.
    #[serde(flatten, deserialize_with = "crate::util::deserialize_extensions")]
    pub extensions: IndexMap<String, serde_json::Value>,
}

#[cfg(feature = "conversions")]
use crate::v3_0;

#[cfg(feature = "conversions")]
impl From<v3_0::Operation> for Operation {
    fn from(o: v3_0::Operation) -> Self {
        Operation {
            tags: o.tags,
            summary: o.summary,
            description: o.description,
            external_docs: o.external_docs.map(Into::into),
            operation_id: o.operation_id,
            parameters: o
                .parameters
                .into_iter()
                .map(|v| ReferenceOr::from_v3_0(v))
                .collect(),
            request_body: o.request_body.map(|v| ReferenceOr::from_v3_0(v)),
            responses: Some(o.responses.into()),
            deprecated: o.deprecated,
            security: o.security.into_iter().map(|v| v.into()).collect(),
            servers: o.servers.into_iter().map(|v| v.into()).collect(),
            extensions: o.extensions,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::v3_1::{Operation, ReferenceOr, Response, Responses, StatusCode};
    use indexmap::IndexMap;
    use serde_yaml::from_str;

    fn response(desc: &str) -> ReferenceOr<Response> {
        ReferenceOr::Item(Response {
            description: desc.to_owned(),
            ..Default::default()
        })
    }

    #[test]
    fn deserialize_responses() {
        assert_eq!(
            Operation {
                responses: Some(Responses {
                    default: None,
                    responses: {
                        let mut map = IndexMap::new();
                        map.insert(StatusCode::Code(200), response("test"));
                        map
                    },
                    ..Default::default()
                }),
                ..Default::default()
            },
            from_str("{ responses: { 200: { description: 'test' } } }").unwrap(),
        );

        assert_eq!(
            Operation {
                responses: Some(Responses {
                    default: None,
                    responses: {
                        let mut map = IndexMap::new();
                        map.insert(StatusCode::Code(666), response("demo"));
                        map
                    },
                    ..Default::default()
                }),
                ..Default::default()
            },
            from_str("{ responses: { \"666\": { description: 'demo' } } }").unwrap(),
        );

        assert_eq!(
            Operation {
                responses: Some(Responses {
                    default: Some(response("def")),
                    responses: {
                        let mut map = IndexMap::new();
                        map.insert(StatusCode::Code(666), response("demo"));
                        map.insert(StatusCode::Code(418), response("demo"));
                        map
                    },
                    ..Default::default()
                }),
                ..Default::default()
            },
            from_str("{ responses: { default: { description: 'def' }, \"666\": { description: 'demo' }, 418: { description: 'demo' } } }").unwrap(),
        );
    }
}
