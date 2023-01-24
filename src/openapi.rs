use crate::*;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

    pub fn get_operation(&self, operation_id: &str) -> Option<(&Operation, &PathItem)> {
        self.operations()
            .find(|(_, _, op, _)| op.operation_id.as_ref().unwrap() == operation_id)
            .map(|(_, _, op, item)| (op, item))
    }

    pub fn components_mut(&mut self) -> &mut Components {
        self.components.as_mut().unwrap()
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

    pub fn clean(&mut self) {
        for (_c, schema) in self.schemas_mut() {
            let ReferenceOr::Item(schema) = schema else {
                continue;
            };
            match &mut schema.schema_kind {
                SchemaKind::Type(Type::String(StringType {
                                                  enumeration,
                                                  ..
                                              })) => {
                    enumeration.sort();
                }
                SchemaKind::OneOf { .. } => {}
                SchemaKind::AllOf { .. } => {}
                SchemaKind::AnyOf { .. } => {}
                SchemaKind::Not { .. } => {}
                SchemaKind::Any(_) => {}
                _ => {}
            }
        }
    }

    /// Merge another OpenAPI document into this one, keeping original schemas on conflict.
    /// `a.merge(b)` will have all schemas from `a` and `b`, but keep `a` for any duplicates.
    pub fn merge(mut self, other: OpenAPI) -> Result<Self, MergeError> {
        merge_map(&mut self.info.extensions, other.info.extensions);

        merge_vec(&mut self.servers, other.servers, |a, b| a.url == b.url);

        for (path, item) in other.paths {
            let item = item.into_item().ok_or_else(|| MergeError::new("PathItem references are not yet supported. Please opena n issue if you need this feature."))?;
            if self.paths.paths.contains_key(&path) {
                let self_item = self.paths.paths.get_mut(&path).unwrap().as_mut().ok_or_else(|| MergeError::new("PathItem references are not yet supported. Please open an issue if you need this feature."))?;
                option_or(&mut self_item.get, item.get);
                option_or(&mut self_item.put, item.put);
                option_or(&mut self_item.post, item.post);
                option_or(&mut self_item.delete, item.delete);
                option_or(&mut self_item.options, item.options);
                option_or(&mut self_item.head, item.head);
                option_or(&mut self_item.patch, item.patch);
                option_or(&mut self_item.trace, item.trace);

                merge_vec(&mut self_item.servers, item.servers, |a, b| a.url == b.url);
                merge_map(&mut self_item.extensions, item.extensions);

                if self_item.parameters.len() != item.parameters.len() {
                    return Err(MergeError(format!("PathItem {} parameters do not have the same length", path)));
                }
                for (a, b) in self_item.parameters.iter_mut().zip(item.parameters) {
                    let a = a.as_item().ok_or_else(|| MergeError::new("Parameter references are not yet supported. Please open an issue if you need this feature."))?;
                    let b = b.as_item().ok_or_else(|| MergeError::new("Parameter references are not yet supported. Please open an issue if you need this feature."))?;
                    let a = a.parameter_data_ref();
                    let b = b.parameter_data_ref();
                    if a.name != b.name {
                        return Err(MergeError(format!("PathItem {} parameter {} does not have the same name as {}", path, a.name, b.name)));
                    }
                }
            } else {
                self.paths.paths.insert(path, ReferenceOr::Item(item));
            }
        }

        if self.components.is_none() {
            self.components = other.components
        } else if let (Some(self_components), Some(other_components)) = (&mut self.components, other.components) {
            merge_map(&mut self_components.extensions, other_components.extensions);
            merge_map(&mut self_components.schemas, other_components.schemas);
            merge_map(&mut self_components.responses, other_components.responses);
            merge_map(&mut self_components.parameters, other_components.parameters);
            merge_map(&mut self_components.examples, other_components.examples);
            merge_map(&mut self_components.request_bodies, other_components.request_bodies);
            merge_map(&mut self_components.headers, other_components.headers);
            merge_map(&mut self_components.security_schemes, other_components.security_schemes);
            merge_map(&mut self_components.links, other_components.links);
            merge_map(&mut self_components.callbacks, other_components.callbacks);
        }

        if self.security.is_none() {
            self.security = other.security;
        } else if let (Some(self_security), Some(other_security)) = (&mut self.security, other.security) {
            merge_vec(self_security, other_security, |a, b| {
                if a.len() != b.len() {
                    return false;
                }
                a.iter().all(|(a, _)| b.contains_key(a))
            });
        }

        merge_vec(&mut self.tags, other.tags, |a, b| a.name == b.name);

        match self.external_docs.as_mut() {
            Some(ext) => {
                if let Some(other) = other.external_docs {
                    merge_map(&mut ext.extensions, other.extensions)
                }
            }
            None => self.external_docs = other.external_docs
        }

        merge_map(&mut self.extensions, other.extensions);
        Ok(self)
    }

    /// Merge another OpenAPI document into this one, replacing any duplicate schemas.
    /// `a.merge_overwrite(b)` will have all schemas from `a` and `b`, but keep `b` for any duplicates.
    pub fn merge_overwrite(self, other: OpenAPI) -> Result<Self, MergeError> {
        other.merge(self)
    }

    pub fn add_schema(&mut self, name: &str, schema: Schema) {
        self.schemas_mut().insert(name.to_string(), ReferenceOr::Item(schema));
    }
}

impl Default for OpenAPI {
    fn default() -> Self {
        // 3.1 is a backwards incompatible change that we don't support yet.
        OpenAPI {
            openapi: "3.0.3".to_string(),
            info: Default::default(),
            servers: vec![],
            paths: Default::default(),
            components: Some(Default::default()),
            security: None,
            tags: vec![],
            external_docs: None,
            extensions: Default::default(),
        }
    }
}

fn merge_vec<T>(original: &mut Vec<T>, mut other: Vec<T>, cmp: fn(&T, &T) -> bool) {
    other.retain(|o| !original.iter().any(|r| cmp(o, r)));
    original.extend(other);
}

fn merge_map<K, V>(original: &mut IndexMap<K, V>, mut other: IndexMap<K, V>) where K: Eq + std::hash::Hash {
    other.retain(|k, _| !original.contains_key(k));
    original.extend(other);
}

fn option_or<T>(original: &mut Option<T>, other: Option<T>) {
    if original.is_none() {
        *original = other;
    }
}

#[derive(Debug)]
pub struct MergeError(String);

impl MergeError {
    pub fn new(msg: &str) -> Self {
        MergeError(msg.to_string())
    }
}

impl std::error::Error for MergeError {}

impl std::fmt::Display for MergeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_basic() {
        let mut a = OpenAPI::default();
        a.servers.push(Server {
            url: "http://localhost".to_string(),
            ..Server::default()
        });
        let mut b = OpenAPI::default();
        b.servers.push(Server {
            url: "http://localhost".to_string(),
            ..Server::default()
        });
        a = a.merge(b).unwrap();
        assert_eq!(a.servers.len(), 1);
    }
}