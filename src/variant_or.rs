use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum VariantOrUnknown<T> {
    Item(T),
    Unknown(String),
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(untagged)]
pub enum VariantOrUnknownOrEmpty<T> {
    Item(T),
    Unknown(String),
    #[default]
    Empty,
}

impl<T> VariantOrUnknownOrEmpty<T> {
    pub fn is_empty(&self) -> bool {
        matches!(self, VariantOrUnknownOrEmpty::Empty)
    }
}
