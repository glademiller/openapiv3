use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ReferenceOr<T> {
    Reference {
        #[serde(rename = "$ref")]
        reference: String,
    },
    Item(T),
}

impl<T> ReferenceOr<T> {
    pub fn ref_(r: &str) -> Self {
        ReferenceOr::Reference {
            reference: r.to_owned(),
        }
    }
    pub fn boxed_item(item: T) -> ReferenceOr<Box<T>> {
        ReferenceOr::Item(Box::new(item))
    }

    pub fn into_item(self) -> Option<T> {
        match self {
            ReferenceOr::Reference { reference: _ } => None,
            ReferenceOr::Item(i) => Some(i),
        }
    }

    pub fn as_item(&self) -> Option<&T> {
        match self {
            ReferenceOr::Reference { reference: _ } => None,
            ReferenceOr::Item(i) => Some(i),
        }
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
