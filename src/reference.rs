use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ReferenceOr<T> {
    Reference {
        #[serde(rename = "$ref")]
        reference: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        summary: Option<String>,
    },
    Item(T),
}

impl<T> ReferenceOr<T> {
    pub fn ref_(r: &str) -> Self {
        ReferenceOr::Reference {
            reference: r.to_owned(),
            description: None,
            summary: None,
        }
    }
    pub fn boxed_item(item: T) -> ReferenceOr<Box<T>> {
        ReferenceOr::Item(Box::new(item))
    }

    /// Converts this [ReferenceOr] to the item inside, if it exists.
    ///
    /// The return value will be [Option::Some] if this was a [ReferenceOr::Item] or [Option::None] if this was a [ReferenceOr::Reference].
    ///
    /// # Examples
    ///
    /// ```
    /// # use openapiv3::ReferenceOr;
    ///
    /// let i = ReferenceOr::Item(1);
    /// assert_eq!(i.into_item(), Some(1));
    ///
    /// let j: ReferenceOr<u8> = ReferenceOr::Reference { reference: String::new(), description: None, summary: None };
    /// assert_eq!(j.into_item(), None);
    /// ```
    pub fn into_item(self) -> Option<T> {
        match self {
            ReferenceOr::Reference { .. } => None,
            ReferenceOr::Item(i) => Some(i),
        }
    }

    /// Returns a reference to to the item inside this [ReferenceOr], if it exists.
    ///
    /// The return value will be [Option::Some] if this was a [ReferenceOr::Item] or [Option::None] if this was a [ReferenceOr::Reference].
    ///
    /// # Examples
    ///
    /// ```
    /// # use openapiv3::ReferenceOr;
    ///
    /// let i = ReferenceOr::Item(1);
    /// assert_eq!(i.as_item(), Some(&1));
    ///
    /// let j: ReferenceOr<u8> = ReferenceOr::Reference { reference: String::new(), description: None, summary: None };
    /// assert_eq!(j.as_item(), None);
    /// ```
    pub fn as_item(&self) -> Option<&T> {
        match self {
            ReferenceOr::Reference { .. } => None,
            ReferenceOr::Item(i) => Some(i),
        }
    }
}

impl<T> ReferenceOr<Box<T>> {
    pub fn unbox(self) -> ReferenceOr<T> {
        match self {
            ReferenceOr::Reference {
                reference,
                description,
                summary,
            } => ReferenceOr::Reference {
                reference,
                description,
                summary,
            },
            ReferenceOr::Item(boxed) => ReferenceOr::Item(*boxed),
        }
    }
}
