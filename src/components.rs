use crate::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Holds a set of reusable objects for different aspects of the OAS.
/// All objects defined within the components object will have no effect
/// on the API unless they are explicitly referenced from properties
/// outside the components object.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Components {
	/// An object to hold reusable Security Scheme Objects.
	#[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
	pub security_schemes: BTreeMap<String, ReferenceOr<SecurityScheme>>,
	/// An object to hold reusable Response Objects.
	#[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
	pub responses: BTreeMap<String, ReferenceOr<Response>>,
	/// An object to hold reusable Parameter Objects.
	#[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
	pub parameters: BTreeMap<String, ReferenceOr<Parameter>>,
	/// An object to hold reusable Example Objects.
	#[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
	pub examples: BTreeMap<String, ReferenceOr<Example>>,
	/// An object to hold reusable Request Body Objects.
	#[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
	pub request_bodies: BTreeMap<String, ReferenceOr<RequestBody>>,
	/// An object to hold reusable Header Objects.
	#[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
	pub headers: BTreeMap<String, ReferenceOr<Header>>,
	/// An object to hold reusable Schema Objects.
	#[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
	pub schemas: BTreeMap<String, ReferenceOr<Schema>>,
	/// An object to hold reusable Link Objects.
	#[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
	pub links: BTreeMap<String, ReferenceOr<Link>>,
	/// An object to hold reusable Callback Objects.
	#[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
	pub callbacks: BTreeMap<String, ReferenceOr<Callback>>,
}
