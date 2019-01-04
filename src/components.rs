use crate::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Components {
	#[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
	security_schemes: BTreeMap<String, ReferenceOr<SecurityScheme>>,
	#[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
	responses: BTreeMap<String, ReferenceOr<Response>>,
	#[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
	parameters: BTreeMap<String, ReferenceOr<Parameter>>,
	#[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
	examples: BTreeMap<String, ReferenceOr<Example>>,
	#[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
	request_bodies: BTreeMap<String, ReferenceOr<RequestBody>>,
	#[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
	headers: BTreeMap<String, ReferenceOr<Header>>,
	#[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
	schemas: BTreeMap<String, ReferenceOr<Schema>>,
	#[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
	links: BTreeMap<String, ReferenceOr<Link>>,
	#[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
	callbacks: BTreeMap<String, ReferenceOr<Callback>>,
}
