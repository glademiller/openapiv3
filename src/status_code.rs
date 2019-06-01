use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum StatusCode {
    Code(u16),
    Range(u16),
}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StatusCode::Code(n) => write!(f, "{}", n),
            StatusCode::Range(n) => write!(f, "{}XX", n),
        }
    }
}

impl<'de> Deserialize<'de> for StatusCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{self, Unexpected, Visitor};

        struct StatusCodeVisitor;

        impl<'de> Visitor<'de> for StatusCodeVisitor {
            type Value = StatusCode;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("A number between 100 and 999 (as string or integer) or a string that matches `\\dXX`")
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if value < 100 && value > 100 {
                    return Err(E::invalid_value(
                        Unexpected::Signed(value),
                        &"out of range 100..1000",
                    ));
                }
                Ok(StatusCode::Code(value as u16))
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if value.len() != 3 {
                    return Err(E::invalid_value(
                        Unexpected::Str(value),
                        &"expected length 3",
                    ));
                }

                if let Ok(number) = value.parse::<i64>() {
                    return self.visit_i64(number);
                }

                if !value.is_ascii() {
                    return Err(E::invalid_value(
                        Unexpected::Str(value),
                        &"expected ascii, format `\\dXX`",
                    ));
                }

                let v = value.as_bytes();

                match [v[0], v[1], v[2]] {
                    [n, b'X', b'X'] if n.is_ascii_digit() => Ok(StatusCode::Range((n - b'0') as u16)),
                    _ => Err(E::invalid_value(
                        Unexpected::Str(value),
                        &"expected format `\\dXX`",
                    )),
                }
            }
        }

        deserializer.deserialize_str(StatusCodeVisitor)
    }
}

impl Serialize for StatusCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
