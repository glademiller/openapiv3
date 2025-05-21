use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// Defines a security scheme that can be used by the operations.
/// Supported schemes are HTTP authentication, an API key (either as a
/// header or as a query parameter), OAuth2's common flows (implicit, password,
/// application and access code) as defined in RFC6749, and OpenID Connect Discovery.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum SecurityScheme {
    #[serde(rename = "apiKey")]
    APIKey {
        /// The location of the API key. Valid values are "query", "header" or
        /// "cookie".
        #[serde(rename = "in")]
        location: APIKeyLocation,
        /// The name of the header, query or cookie parameter to be used.
        name: String,
        /// A short description for security scheme. CommonMark syntax MAY be
        /// used for rich text representation.
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,

        /// Inline extensions to this object.
        #[serde(flatten, deserialize_with = "crate::util::deserialize_extensions")]
        extensions: IndexMap<String, serde_json::Value>,
    },
    #[serde(rename = "http")]
    HTTP {
        /// The name of the HTTP Authorization scheme to be used in the
        /// Authorization header as defined in RFC7235. The values used SHOULD
        /// be registered in the IANA Authentication Scheme registry.
        scheme: String,
        #[serde(rename = "bearerFormat", skip_serializing_if = "Option::is_none")]
        bearer_format: Option<String>,
        /// A short description for security scheme. CommonMark syntax MAY be
        /// used for rich text representation.
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,

        /// Inline extensions to this object.
        #[serde(flatten, deserialize_with = "crate::util::deserialize_extensions")]
        extensions: IndexMap<String, serde_json::Value>,
    },
    #[serde(rename = "oauth2")]
    OAuth2 {
        /// An object containing configuration information for the flow types
        /// supported.
        flows: OAuth2Flows,
        /// A short description for security scheme. CommonMark syntax MAY be
        /// used for rich text representation.
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,

        /// Inline extensions to this object.
        #[serde(flatten, deserialize_with = "crate::util::deserialize_extensions")]
        extensions: IndexMap<String, serde_json::Value>,
    },
    #[serde(rename = "openIdConnect")]
    OpenIDConnect {
        /// OpenId Connect URL to discover OAuth2 configuration values. This
        /// MUST be in the form of a URL.
        #[serde(rename = "openIdConnectUrl")]
        open_id_connect_url: String,
        /// A short description for security scheme. CommonMark syntax MAY be
        /// used for rich text representation.
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,

        /// Inline extensions to this object.
        #[serde(flatten, deserialize_with = "crate::util::deserialize_extensions")]
        extensions: IndexMap<String, serde_json::Value>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum APIKeyLocation {
    Query,
    Header,
    Cookie,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OAuth2Flows {
    /// Configuration for the OAuth Implicit flow
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub implicit: Option<ImplicitOAuth2Flow>,
    /// Configuration for the OAuth Resource Owner Password flow
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<PasswordOAuth2Flow>,
    /// Configuration for the OAuth Client Credentials flow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_credentials: Option<ClientCredentialsOAuth2Flow>,
    /// Configuration for the OAuth Authorization Code flow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorization_code: Option<AuthorizationCodeOAuth2Flow>,

    /// Inline extensions to this object.
    #[serde(flatten, deserialize_with = "crate::util::deserialize_extensions")]
    pub extensions: IndexMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ImplicitOAuth2Flow {
    /// The authorization URL to be used for this flow. This MUST be in the
    /// form of a URL.
    pub authorization_url: String,
    /// The URL to be used for obtaining refresh tokens. This MUST be in the
    /// form of a URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_url: Option<String>,
    /// The available scopes for the OAuth2 security scheme. A map between the
    /// scope name and a short description for it. The map MAY be empty.
    pub scopes: IndexMap<String, String>,

    /// Inline extensions to this object.
    #[serde(flatten, deserialize_with = "crate::util::deserialize_extensions")]
    pub extensions: IndexMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PasswordOAuth2Flow {
    /// The URL to be used for obtaining refresh tokens. This MUST be in the
    /// form of a URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_url: Option<String>,
    /// The token URL to be used for this flow. This MUST be in the form of a
    /// URL.
    pub token_url: String,
    /// The available scopes for the OAuth2 security scheme. A map between the
    /// scope name and a short description for it. The map MAY be empty.
    pub scopes: IndexMap<String, String>,

    /// Inline extensions to this object.
    #[serde(flatten, deserialize_with = "crate::util::deserialize_extensions")]
    pub extensions: IndexMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClientCredentialsOAuth2Flow {
    /// The URL to be used for obtaining refresh tokens. This MUST be in the
    /// form of a URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_url: Option<String>,
    /// The token URL to be used for this flow. This MUST be in the form of a
    /// URL.
    pub token_url: String,
    /// The available scopes for the OAuth2 security scheme. A map between the
    /// scope name and a short description for it. The map MAY be empty.
    pub scopes: IndexMap<String, String>,

    /// Inline extensions to this object.
    #[serde(flatten, deserialize_with = "crate::util::deserialize_extensions")]
    pub extensions: IndexMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizationCodeOAuth2Flow {
    /// The authorization URL to be used for this flow. This MUST be in the
    /// form of a URL.
    pub authorization_url: String,
    /// The token URL to be used for this flow. This MUST be in the form of a
    /// URL.
    pub token_url: String,
    /// The URL to be used for obtaining refresh tokens. This MUST be in the
    /// form of a URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_url: Option<String>,
    /// The available scopes for the OAuth2 security scheme. A map between the
    /// scope name and a short description for it. The map MAY be empty.
    pub scopes: IndexMap<String, String>,

    /// Inline extensions to this object.
    #[serde(flatten, deserialize_with = "crate::util::deserialize_extensions")]
    pub extensions: IndexMap<String, serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use crate::{OpenAPI, ReferenceOr, SecurityScheme};

    #[test]
    fn test_slack_auth() {
        let openapi: OpenAPI =
            serde_json::from_reader(std::fs::File::open("fixtures/slack.json").unwrap()).unwrap();

        assert!(matches!(
            openapi
                .components
                .as_ref()
                .unwrap()
                .security_schemes
                .get("slackAuth")
                .unwrap(),
            ReferenceOr::Item(SecurityScheme::OAuth2 { .. })
        ));
    }
}
