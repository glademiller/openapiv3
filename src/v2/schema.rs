use serde::{Deserialize, Serialize};
use indexmap::IndexMap;

// http://json.schemastore.org/swagger-2.0

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Scheme {
    Http,
    Https,
    Ws,
    Wss,
}

impl Scheme {
    pub fn as_str(&self) -> &'static str {
        match self {
            Scheme::Http => "http",
            Scheme::Https => "https",
            Scheme::Ws => "ws",
            Scheme::Wss => "wss",
        }
    }
}

impl Default for Scheme {
    fn default() -> Self {
        Scheme::Http
    }
}

/// top level document
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct OpenAPI {
    /// The Swagger version of this document.
    pub swagger: String,
    pub info: Info,
    /// The host (name or ip) of the API. Example: 'swagger.io'
    /// ^[^{}/ :\\\\]+(?::\\d+)?$
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// The base path to the API. Example: '/api'.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "basePath")]
    pub base_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schemes: Option<Vec<Scheme>>,
    /// A list of MIME types accepted by the API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumes: Option<Vec<String>>,
    /// A list of MIME types the API can produce.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub produces: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// Relative paths to the individual endpoints. They must be relative
    /// to the 'basePath'.
    pub paths: IndexMap<String, PathItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definitions: Option<IndexMap<String, Schema>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<IndexMap<String, Parameter>>,
    /// mappings to http response codes or "default"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responses: Option<IndexMap<String, Response>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_definitions: Option<IndexMap<String, Security>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<Vec<IndexMap<String, Vec<String>>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<Vec<ExternalDoc>>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub struct Tag {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<Vec<ExternalDoc>>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct ExternalDoc {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// General information about the API.
///
/// https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#info-object
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub struct Info {
    /// A unique and precise title of the API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// A semantic version number of the API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "termsOfService", skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<License>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Contact {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    // TODO: Make sure the url is a valid URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    // TODO: Make sure the email is a valid email
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

/// todo x-* properties
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct License {
    /// The name of the license type. It's encouraged to use an OSI
    /// compatible license.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The URL pointing to the license.
    // TODO: Make sure the url is a valid URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// todo support x-* properties
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct PathItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub put: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<Parameter>>,
}

/// https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#operation-object
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub struct Operation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub produces: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schemes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "operationId", skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    pub responses: IndexMap<String, Response>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<Parameter>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<Vec<SecurityRequirement>>,
}

/// https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#securityRequirementObject
pub type SecurityRequirement = IndexMap<String, Vec<String>>;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ParameterLocation {
    Query,
    Header,
    Path,
    FormData,
    Body,
}

impl Default for ParameterLocation {
    fn default() -> Self {
        ParameterLocation::Query
    }
}

impl Parameter {
    pub fn valid_v3_location(&self) -> bool {
        use ParameterLocation::*;
        matches!(self.location, Query | Header | Path)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Parameter {
    pub name: String,
    #[serde(rename = "in")]
    pub location: ParameterLocation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// Only relevant for `ParameterLocation::Body`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<ReferenceOrSchema>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_items: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Required if type is array.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<ReferenceOrSchema>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "collectionFormat")]
    pub collection_format: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Response {
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<ReferenceOrSchema>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ApiKeyLocation {
    Query,
    Header,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "type")]
pub enum Security {
    #[serde(rename = "apiKey")]
    ApiKey {
        name: String,
        #[serde(rename = "in")]
        location: ApiKeyLocation,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
    },
    #[serde(rename = "oauth2")]
    Oauth2 {
        flow: Flow,
        #[serde(rename = "authorizationUrl")]
        authorization_url: String,
        #[serde(rename = "tokenUrl")]
        #[serde(skip_serializing_if = "Option::is_none")]
        token_url: Option<String>,
        scopes: IndexMap<String, String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
    },
    #[serde(rename = "basic")]
    Basic {
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
    },
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Flow {
    Implicit,
    Password,
    Application,
    AccessCode,
}

/// A [JSON schema](http://json-schema.org/) definition describing
/// the shape and properties of an object.
///
/// This may also contain a `$ref` to another definition
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Schema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub schema_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "enum")]
    pub enum_values: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<ReferenceOrSchema>>,
    // implies object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<IndexMap<String, ReferenceOrSchema>>,
    // composition
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "allOf")]
    pub all_of: Option<Vec<ReferenceOrSchema>>,
    // TODO: we need a validation step that we only collect x-* properties here.
    #[serde(flatten)]
    pub other: IndexMap<String, serde_json::Value>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum ReferenceOrSchema {
    Reference {
        #[serde(rename = "$ref")]
        reference: String,
    },
    Item(Schema),
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use serde_yaml;

    #[test]
    fn security_api_deserializes() {
        let json = r#"{"type":"apiKey", "name":"foo", "in": "query"}"#;
        assert_eq!(
            serde_yaml::from_str::<Security>(&json).unwrap(),
            Security::ApiKey {
                name: "foo".into(),
                location: serde_json::from_str("\"query\"").unwrap(),
                description: None,
            }
        );
    }

    #[test]
    fn security_api_serializes() {
        let json = r#"{"type":"apiKey","name":"foo","in":"query"}"#;
        assert_eq!(
            serde_json::to_string(&Security::ApiKey {
                name: "foo".into(),
                location: serde_json::from_str("\"query\"").unwrap(),
                description: None,
            })
                .unwrap(),
            json
        );
    }

    #[test]
    fn security_basic_deserializes() {
        let json = r#"{"type":"basic"}"#;
        assert_eq!(
            serde_yaml::from_str::<Security>(&json).unwrap(),
            Security::Basic { description: None }
        );
    }

    #[test]
    fn security_basic_serializes() {
        let json = r#"{"type":"basic"}"#;
        assert_eq!(
            json,
            serde_json::to_string(&Security::Basic { description: None }).unwrap()
        );
    }

    #[test]
    fn security_oauth_deserializes() {
        let json = r#"{"type":"oauth2","flow":"implicit","authorizationUrl":"foo/bar","scopes":{"foo":"bar"}}"#;
        let mut scopes = IndexMap::new();
        scopes.insert("foo".into(), "bar".into());
        assert_eq!(
            serde_yaml::from_str::<Security>(&json).unwrap(),
            Security::Oauth2 {
                flow: Flow::Implicit,
                authorization_url: "foo/bar".into(),
                token_url: None,
                scopes: scopes,
                description: None,
            }
        );
    }

    #[test]
    fn security_oauth_serializes() {
        let json = r#"{"type":"oauth2","flow":"implicit","authorizationUrl":"foo/bar","scopes":{"foo":"bar"}}"#;
        let mut scopes = IndexMap::new();
        scopes.insert("foo".into(), "bar".into());
        assert_eq!(
            json,
            serde_json::to_string(&Security::Oauth2 {
                flow: Flow::Implicit,
                authorization_url: "foo/bar".into(),
                token_url: None,
                scopes,
                description: None,
            })
                .unwrap()
        );
    }

    #[test]
    fn parameter_or_ref_deserializes_ref() {
        let json = r#"{"$ref":"foo/bar"}"#;
        assert_eq!(
            serde_yaml::from_str::<ReferenceOrSchema>(&json).unwrap(),
            ReferenceOrSchema::Reference {
                reference: "foo/bar".into()
            }
        );
    }

    #[test]
    fn parameter_or_ref_serializes_pref() {
        let json = r#"{"$ref":"foo/bar"}"#;
        assert_eq!(
            json,
            serde_json::to_string(&ReferenceOrSchema::Reference {
                reference: "foo/bar".into()
            })
                .unwrap()
        );
    }
}