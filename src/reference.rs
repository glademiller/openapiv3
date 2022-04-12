use serde::{Deserialize, Serialize};
use crate::{OpenAPI, Schema};

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
    /// let j: ReferenceOr<u8> = ReferenceOr::Reference { reference: String::new() };
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
    /// let j: ReferenceOr<u8> = ReferenceOr::Reference { reference: String::new() };
    /// assert_eq!(j.as_item(), None);
    /// ```
    pub fn as_item(&self) -> Option<&T> {
        match self {
            ReferenceOr::Reference { .. } => None,
            ReferenceOr::Item(i) => Some(i),
        }
    }


}

impl<T: 'static> ReferenceOr<T> {
    pub fn as_ref(&self) -> ReferenceOr<&T> {
        match self {
            ReferenceOr::Reference { reference } => ReferenceOr::Reference { reference: reference.clone() },
            ReferenceOr::Item(i) => ReferenceOr::Item(i),
        }
    }
}

pub fn get_struct_name_from_reference(reference: &str) -> Option<&str> {
    let mut parts = reference.split('/');
    if parts.next() != Some("#") {
        return None;
    }
    if parts.next() != Some("components") {
        return None;
    }
    if parts.next() != Some("schemas") {
        return None;
    }
    parts.next()
}

impl<'a> ReferenceOr<&'a Schema> {
    pub fn resolve(&self, spec: &'a OpenAPI) -> Option<&'a Schema> {
        match self {
            ReferenceOr::Reference { reference } => {
                let name = get_struct_name_from_reference(&reference).unwrap();
                spec.components.as_ref().and_then(|c|
                    c.schemas.get(name).and_then(|ref_or_schema| {
                        let s = ref_or_schema.as_ref();
                        s.resolve(spec)
                    })
                )
            },
            ReferenceOr::Item(schema) => Some(schema),
        }
    }

    pub fn get_struct_name(&self) -> Option<&str> {
        match self {
            ReferenceOr::Reference { reference } => get_struct_name_from_reference(&reference),
            ReferenceOr::Item(_schema) => None,
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

    pub fn unbox_ref(&self) -> ReferenceOr<&T> {
        match self {
            ReferenceOr::Reference { reference } => ReferenceOr::Reference { reference: reference.to_owned() },
            ReferenceOr::Item(boxed) => ReferenceOr::Item(boxed.as_ref()),
        }
    }
}
