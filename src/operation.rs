use crate::*;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// Describes a single API operation on a path.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Operation {
    /// A list of tags for API documentation control.
    /// Tags can be used for logical grouping of operations
    /// by resources or any other qualifier.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ReferenceOr<Parameter>>,
    /// The request body applicable for this operation.
    /// The requestBody is only supported in HTTP methods
    /// where the HTTP 1.1 specification RFC7231 has explicitly
    /// defined semantics for request bodies. In other cases where
    /// the HTTP spec is vague, requestBody SHALL be ignored by consumers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_body: Option<ReferenceOr<RequestBody>>,
    /// REQUIRED. The list of possible responses as they are returned
    /// from executing this operation.
    pub responses: Responses,
    /// A map of possible out-of band callbacks related to the parent
    /// operation. The key is a unique identifier for the Callback Object. Each
    /// value in the map is a Callback Object that describes a request that may
    /// be initiated by the API provider and the expected responses.
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub callbacks: IndexMap<String, Callback>,
    /// Declares this operation to be deprecated.Default value is false.
    #[serde(default, skip_serializing_if = "is_false")]
    pub deprecated: bool,
    /// A declaration of which security mechanisms can be used for this operation.
    /// The list of values includes alternative security requirement objects that can
    /// be used. Only one of the security requirement objects need to be satisfied to
    /// authorize a request. This definition overrides any declared top-level security.
    /// To remove a top-level security declaration, an empty array can be used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security: Option<Vec<SecurityRequirement>>,
    /// An alternative server array to service this operation.
    /// If an alternative server object is specified at the
    /// Path Item Object or Root level, it will be overridden by this value.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub servers: Vec<Server>,
    /// Inline extensions to this object.
    #[serde(flatten, deserialize_with = "crate::util::deserialize_extensions")]
    pub extensions: IndexMap<String, serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use crate::{Operation, ReferenceOr, Responses, StatusCode};
    use indexmap::IndexMap;
    use serde_yaml::from_str;

    #[test]
    fn deserialize_responses() {
        assert_eq!(
            Operation {
                responses: Responses {
                    default: None,
                    responses: {
                        let mut map = IndexMap::new();
                        map.insert(StatusCode::Code(200), ReferenceOr::ref_("test"));
                        map
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            from_str("{ responses: { 200: { $ref: 'test' } } }").unwrap(),
        );

        assert_eq!(
            Operation {
                responses: Responses {
                    default: None,
                    responses: {
                        let mut map = IndexMap::new();
                        map.insert(StatusCode::Code(666), ReferenceOr::ref_("demo"));
                        map
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            from_str("{ responses: { \"666\": { $ref: 'demo' } } }").unwrap(),
        );

        assert_eq!(
            Operation {
                responses: Responses {
                    default: Some(ReferenceOr::ref_("def")),
                    responses: {
                        let mut map = IndexMap::new();
                        map.insert(StatusCode::Code(666), ReferenceOr::ref_("demo"));
                        map.insert(StatusCode::Code(418), ReferenceOr::ref_("demo"));
                        map
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            from_str("{ responses: { default: { $ref: 'def' }, \"666\": { $ref: 'demo' }, 418: { $ref: 'demo' } } }").unwrap(),
        );
    }
}
