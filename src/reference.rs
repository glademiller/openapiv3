use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ReferenceOr<T> {
    Reference {
        #[serde(rename = "$ref")]
        reference: String,
    },
    Item(T),
}

// We implement Deserialize by hand in order to provide a useful error message.
// The derived error is typically aggravating: "data did not match any variant
// of untagged enum ReferenceOr". Instead, we deserialize to a Value, look for
// $ref, and otherwise return the response from T::deserialize.
impl<'de, T> Deserialize<'de> for ReferenceOr<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;

        fn to_ref<S>(value: &serde_json::Value) -> Option<ReferenceOr<S>> {
            let obj = value.as_object()?;
            if obj.len() != 1 {
                return None;
            }
            let ref_val = obj.get("$ref")?;
            let reference = ref_val.as_str()?.to_string();
            Some(ReferenceOr::Reference { reference })
        }

        if let Some(r) = to_ref(&value) {
            Ok(r)
        } else {
            use serde::de::Error;
            let item = T::deserialize(value).map_err(D::Error::custom)?;
            Ok(Self::Item(item))
        }
    }
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

impl<T> ReferenceOr<Box<T>> {
    pub fn unbox(self) -> ReferenceOr<T> {
        match self {
            ReferenceOr::Reference { reference } => ReferenceOr::Reference { reference },
            ReferenceOr::Item(boxed) => ReferenceOr::Item(*boxed),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    #[test]
    fn test_bad_responses() {
        // Note: missing the "description" field
        let value = json!({
            "200": {
                "content": {
                    "application/json": {
                        "schema": {
                            "type": "string"
                        }
                    }
                }
            }
        });

        match serde_json::from_value::<crate::Responses>(value) {
            Ok(_) => unreachable!(),
            Err(e) => assert!(
                e.to_string().contains("missing field `description`"),
                "unhelpful error: {e}"
            ),
        }
    }
}
