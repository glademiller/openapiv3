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
    // TODO i believe this should be called as_ref() ?
    pub fn as_item(&self) -> Option<&T> {
        match self {
            ReferenceOr::Reference { .. } => None,
            ReferenceOr::Item(i) => Some(i),
        }
    }

    pub fn as_mut(&mut self) -> Option<&mut T> {
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


pub fn get_component_name(reference: &str) -> Option<&str> {
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


impl ReferenceOr<Schema> {
    pub fn resolve<'a>(&'a self, spec: &'a OpenAPI) -> &'a Schema {
        match self {
            ReferenceOr::Reference { reference } => {
                let name = get_component_name(&reference).unwrap();
                let components = spec.components.as_ref().unwrap();
                let ref_or_schema = components.schemas.get(name).unwrap();
                match ref_or_schema {
                    ReferenceOr::Item(schema) => schema,
                    ReferenceOr::Reference { .. } => ref_or_schema.resolve(spec),
                }
            },
            ReferenceOr::Item(schema) => schema,
        }
    }
}

impl ReferenceOr<Box<Schema>> {
    pub fn unbox(&self) -> ReferenceOr<Schema> {
        match self {
            ReferenceOr::Reference { reference } => ReferenceOr::Reference { reference: reference.clone() },
            ReferenceOr::Item(boxed) => ReferenceOr::Item(*boxed.clone()),
        }
    }
}
