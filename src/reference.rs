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

impl<T> ReferenceOr<T> {
    pub fn ref_(r: &str) -> Self {
        ReferenceOr::Reference { reference: r.to_owned() }
    }
    pub fn boxed_item(item: T) -> ReferenceOr<Box<T>> {
        ReferenceOr::Item(Box::new(item))
    }
}

impl<T> ReferenceOr<Box<T>> {
    pub fn unbox(self) -> ReferenceOr<T> {
        match self {
            ReferenceOr::Reference { reference } => ReferenceOr::Reference { reference },
            ReferenceOr::Item(boxed) => ReferenceOr::Item(*boxed),
        }
    }
}
