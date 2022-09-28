use crate::*;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OpenAPI {
    /// REQUIRED. This string MUST be the semantic version number of the
    /// OpenAPI Specification version that the OpenAPI document uses.
    /// The openapi field SHOULD be used by tooling specifications and
    /// clients to interpret the OpenAPI document. This is not related to
    /// the API info.version string.
    pub openapi: String,
    /// REQUIRED. Provides metadata about the API.
    /// The metadata MAY be used by tooling as required.
    pub info: Info,
    /// An array of Server Objects, which provide connectivity information to a
    /// target server. If the servers property is not provided, or is an empty
    /// array, the default value would be a Server Object with a url value of /.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub servers: Vec<Server>,
    /// REQUIRED. The available paths and operations for the API.
    pub paths: Paths,
    /// An element to hold various schemas for the specification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Components>,
    /// A declaration of which security mechanisms can be used across the API.
    /// The list of values includes alternative security requirement objects
    /// that can be used. Only one of the security requirement objects need to
    /// be satisfied to authorize a request. Individual operations can override
    /// this definition. Global security settings may be overridden on a per-path
    /// basis.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<Vec<SecurityRequirement>>,
    /// A list of tags used by the specification with additional metadata.
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
    /// The iterated items are tuples of `(&str, &str, &Operation, &PathItem)` containing
    /// the path, method,  and the operation.
    ///
    /// Path items containing `$ref`s are skipped.
    pub fn operations(&self) -> impl Iterator<Item=(&str, &str, &Operation, &PathItem)> {
        self.paths
            .iter()
            .filter_map(|(path, item)| item.as_item().map(|i| (path, i)))
            .flat_map(|(path, item)| {
                item.iter()
                    .map(move |(method, op)| (path.as_str(), method, op, item))
            })
    }

    pub fn operations_mut(&mut self) -> impl Iterator<Item=(&str, &str, &mut Operation)> {
        self.paths
            .iter_mut()
            .filter_map(|(path, item)| item.as_mut().map(|i| (path, i)))
            .flat_map(|(path, item)| {
                item.iter_mut()
                    .map(move |(method, op)| (path.as_str(), method, op))
            })
    }

    pub fn get_operation_mut(&mut self, operation_id: &str) -> Option<&mut Operation> {
        self.operations_mut().find(|(_, _, op)| op.operation_id.as_ref().unwrap() == operation_id).map(|(_, _, op)| op)
    }

    pub fn get_operation(&self, operation_id: &str) -> Option<&Operation> {
        self.operations()
            .find(|(_, _, op, _)| op.operation_id.as_ref().unwrap() == operation_id)
            .map(|(_, _, op, _)| op)
    }

    pub fn schemas_mut(&mut self) -> &mut IndexMap<String, ReferenceOr<Schema>> {
        &mut self.components
            .as_mut()
            .unwrap()
            .schemas
    }

    pub fn schemas(&self) -> &IndexMap<String, ReferenceOr<Schema>> {
        &self.components
            .as_ref()
            .unwrap()
            .schemas
    }
}
