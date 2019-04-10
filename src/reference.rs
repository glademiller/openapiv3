use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ReferenceOr<T> {
    Reference {
        #[serde(rename = "$ref")]
        reference: String
    },
    Item(T),
}
