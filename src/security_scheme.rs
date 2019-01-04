use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Defines a security scheme that can be used by the operations.
/// Supported schemes are HTTP authentication, an API key (either as a
/// header or as a query parameter), OAuth2's common flows (implicit, password,
/// application and access code) as defined in RFC6749, and OpenID Connect Discovery.
#[serde(tag = "type")]
pub enum SecurityScheme {
	#[serde(rename = "apiKey")]
	APIKey {
		#[serde(rename = "in")]
		location: APIKeyLocation,
		name: String,
	},
	#[serde(rename = "http")]
	HTTP {
		scheme: String,
		#[serde(rename = "bearerFormat")]
		bearer_format: Option<String>,
	},
	#[serde(rename = "oauth2")]
	OAuth2 { flows: OAuth2Flows },
	#[serde(rename = "openIdConnect")]
	OpenIDConnect {
		#[serde(rename = "openIdConnectUrl")]
		open_id_connect_url: String,
	},
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum APIKeyLocation {
	#[serde(rename = "query")]
	Query,
	#[serde(rename = "header")]
	Header,
	#[serde(rename = "cookie")]
	Cookie,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OAuth2Flows {
	#[serde(flatten)]
	implicit: Option<OAuth2Flow>,
	#[serde(flatten)]
	password: Option<OAuth2Flow>,
	#[serde(flatten)]
	client_credentials: Option<OAuth2Flow>,
	#[serde(flatten)]
	authorization_code: Option<OAuth2Flow>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OAuth2Flow {
	#[serde(rename = "implicit")]
	Implicit {
		#[serde(rename = "authorizationUrl")]
		authorization_url: String,
		#[serde(rename = "refreshUrl")]
		refresh_url: Option<String>,
		#[serde(default)]
		scopes: BTreeMap<String, String>,
	},
	#[serde(rename = "password")]
	Password {
		#[serde(rename = "refreshUrl")]
		refresh_url: Option<String>,
		#[serde(rename = "tokenUrl")]
		token_url: String,
		#[serde(default)]
		scopes: BTreeMap<String, String>,
	},
	#[serde(rename = "clientCredentials")]
	ClientCredentials {
		#[serde(rename = "refreshUrl")]
		refresh_url: Option<String>,
		#[serde(rename = "tokenUrl")]
		token_url: String,
		#[serde(default)]
		scopes: BTreeMap<String, String>,
	},
	#[serde(rename = "authorizationCode")]
	AuthorizationCode {
		#[serde(rename = "authorizationUrl")]
		authorization_url: String,
		#[serde(rename = "tokenUrl")]
		token_url: String,
		#[serde(rename = "refreshUrl")]
		refresh_url: Option<String>,
		#[serde(default)]
		scopes: BTreeMap<String, String>,
	},
}
