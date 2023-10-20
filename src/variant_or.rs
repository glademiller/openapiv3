use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum VariantOrUnknown<T> {
    Item(T),
    Unknown(String),
}

impl<T> From<String> for VariantOrUnknown<T>
where
    T: FromStr,
{
    fn from(s: String) -> Self {
        match T::from_str(&s) {
            Ok(t) => Self::Item(t),
            Err(_) => Self::Unknown(s),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum VariantOrUnknownOrEmpty<T> {
    Item(T),
    Unknown(String),
    Empty,
}

impl<T> VariantOrUnknownOrEmpty<T> {
    pub fn is_empty(&self) -> bool {
        match self {
            VariantOrUnknownOrEmpty::Empty => true,
            _ => false,
        }
    }
}

impl<T> Default for VariantOrUnknownOrEmpty<T> {
    fn default() -> Self {
        VariantOrUnknownOrEmpty::Empty
    }
}

impl<T> From<Option<String>> for VariantOrUnknownOrEmpty<T>
where
    T: FromStr,
{
    fn from(v: Option<String>) -> Self {
        match v {
            Some(s) => match T::from_str(&s) {
                Ok(t) => VariantOrUnknownOrEmpty::Item(t),
                Err(_) => VariantOrUnknownOrEmpty::Unknown(s),
            },
            None => VariantOrUnknownOrEmpty::Empty,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{StringFormat, VariantOrUnknownOrEmpty};

    #[test]
    fn test_variant_from() {
        assert_eq!(
            VariantOrUnknownOrEmpty::<StringFormat>::from(None),
            VariantOrUnknownOrEmpty::Empty,
        );
        assert_eq!(
            VariantOrUnknownOrEmpty::<StringFormat>::from(Some("date".to_string())),
            VariantOrUnknownOrEmpty::Item(StringFormat::Date),
        );
        assert_eq!(
            VariantOrUnknownOrEmpty::<StringFormat>::from(Some("yolo".to_string())),
            VariantOrUnknownOrEmpty::Unknown("yolo".to_string()),
        );
    }
}
