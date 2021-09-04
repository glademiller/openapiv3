use std::hash::Hash;
use std::marker::PhantomData;

use indexmap::IndexMap;
use serde::{
    de::{IgnoredAny, Visitor},
    Deserialize, Deserializer,
};

#[allow(clippy::trivially_copy_pass_by_ref)] // needs to match signature for use in serde attribute
#[inline]
pub const fn is_false(v: &bool) -> bool {
    !(*v)
}

pub(crate) fn deserialize_extensions<'de, D>(
    deserializer: D,
) -> Result<IndexMap<String, serde_json::Value>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_map(PredicateVisitor(
        |key: &String| key.starts_with("x-"),
        PhantomData,
    ))
}

/// Used to deserialize IndexMap<K, V> that are flattened within other structs.
/// This only adds keys that satisfy the given predicate.
pub(crate) struct PredicateVisitor<F, K, V>(pub F, pub PhantomData<(K, V)>);

impl<'de, F, K, V> Visitor<'de> for PredicateVisitor<F, K, V>
where
    F: Fn(&K) -> bool,
    K: Deserialize<'de> + Eq + Hash,
    V: Deserialize<'de>,
{
    type Value = IndexMap<K, V>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a map whose fields obey a predicate")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut ret = Self::Value::default();

        loop {
            match map.next_key::<K>() {
                Err(_) => (),
                Ok(None) => break,
                Ok(Some(key)) if self.0(&key) => {
                    let _ = ret.insert(key, map.next_value()?);
                }
                Ok(Some(_)) => {
                    let _ = map.next_value::<IgnoredAny>()?;
                }
            }
        }

        Ok(ret)
    }
}
