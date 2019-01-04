use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Discriminator {
    property_name: String,
    #[serde(default)]
    mapping: BTreeMap<String, String>,
}
