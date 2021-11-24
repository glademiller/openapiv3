use crate::v3_1::*;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OpenAPI {
    /// REQUIRED. This string MUST be the version number of the OpenAPI
    /// Specification that the OpenAPI document uses. The `openapi` field SHOULD
    /// be used by tooling to interpret the OpenAPI document. This is *not*
    /// related to the API `info.version` string.
    pub openapi: String,
    /// REQUIRED. Provides metadata about the API.
    /// The metadata MAY be used by tooling as required.
    pub info: Info,
    /// The default value for the `$schema` keyword within Schema Objects
    /// contained within this OAS document. This MUST be in the form of a URI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_schema_dialect: Option<String>,
    /// An array of Server Objects, which provide connectivity information to a
    /// target server. If the servers property is not provided, or is an empty
    /// array, the default value would be a Server Object with a url value of /.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub servers: Vec<Server>,
    /// The available paths and operations for the API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paths: Option<Paths>,
    /// The incoming webhooks that MAY be received as part of this API and that
    /// the API consumer MAY choose to implement. Closely related to the
    /// `callbacks` feature, this section describes requests initiated other
    /// than by an API call, for example by an out of band registration. The key
    /// name is a unique string to refer to each webhook, while the (optionally
    /// referenced) Path Item Object describes a request that may be initiated
    /// by the API provider and the expected responses.
    #[serde(default)]
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub webhooks: IndexMap<String, ReferenceOr<PathItem>>,
    /// An element to hold various schemas for the document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Components>,
    /// A declaration of which security mechanisms can be used across the API.
    /// The list of values includes alternative security requirement objects
    /// that can be used. Only one of the security requirement objects need to
    /// be satisfied to authorize a request. Individual operations can override
    /// this definition. Global security settings may be overridden on a
    /// per-path basis.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<Vec<SecurityRequirement>>,
    /// A list of tags used by the document with additional metadata.
    /// The order of the tags can be used to reflect on their order by the
    /// parsing tools. Not all tags that are used by the Operation Object
    /// must be declared. The tags that are not declared MAY be organized
    /// randomly or based on the tool's logic. Each tag name in the list
    /// MUST be unique.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<Tag>,
    /// Additional external documentation.
    #[serde(rename = "externalDocs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentation>,
    /// Inline extensions to this object.
    #[serde(flatten, deserialize_with = "crate::util::deserialize_extensions")]
    pub extensions: IndexMap<String, serde_json::Value>,
}

impl OpenAPI {
    /// Iterates through all [Operation]s in this API.
    ///
    /// The iterated items are tuples of `(&str, &str, &Operation)` containing
    /// the path, method,  and the operation.
    ///
    /// Path items containing `$ref`s are skipped.
    pub fn operations(&self) -> impl Iterator<Item = (&str, &str, &Operation)> {
        self.paths.iter().flat_map(|paths| {
            paths
                .iter()
                .filter_map(|(path, item)| item.as_item().map(|i| (path, i)))
                .flat_map(|(path, item)| {
                    item.iter()
                        .map(move |(method, op)| (path.as_str(), method, op))
                })
        })
    }
}
