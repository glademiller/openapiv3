use std::str::FromStr;

use crate::*;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SchemaData {
    #[serde(default, skip_serializing_if = "is_false")]
    pub nullable: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub read_only: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub write_only: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub deprecated: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discriminator: Option<Discriminator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,
    /// Inline extensions to this object.
    #[serde(flatten, deserialize_with = "crate::util::deserialize_extensions")]
    pub extensions: IndexMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Schema {
    #[serde(flatten)]
    pub schema_data: SchemaData,
    #[serde(flatten)]
    pub schema_kind: SchemaKind,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(untagged)]
pub enum SchemaKind {
    Type(Type),
    OneOf {
        #[serde(rename = "oneOf")]
        one_of: Vec<ReferenceOr<Schema>>,
    },
    AllOf {
        #[serde(rename = "allOf")]
        all_of: Vec<ReferenceOr<Schema>>,
    },
    AnyOf {
        #[serde(rename = "anyOf")]
        any_of: Vec<ReferenceOr<Schema>>,
    },
    Not {
        not: Box<ReferenceOr<Schema>>,
    },
    Any(AnySchema),
}

// Custom Deserialize implementation that is similar to the logic for an
// untagged enum with awareness of all the fields we might expect to see. This
// is necessary to ensure that relevant fields aren't ignored e.g. when mixing
// a object with a oneOf/anyOf/allOf. Note that serde's deny_unknown_fields
// doesn't help us here due to its interactions with our abundant use of
// flatten.
impl<'de> Deserialize<'de> for SchemaKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct RawAnySchema {
            #[serde(rename = "type", default)]
            typ: Option<String>,
            #[serde(default)]
            pattern: Option<String>,
            #[serde(default)]
            multiple_of: Option<serde_json::Number>,
            #[serde(default)]
            exclusive_minimum: Option<bool>,
            #[serde(default)]
            exclusive_maximum: Option<bool>,
            #[serde(default)]
            minimum: Option<serde_json::Number>,
            #[serde(default)]
            maximum: Option<serde_json::Number>,
            #[serde(default)]
            properties: Option<IndexMap<String, ReferenceOr<Box<Schema>>>>,
            #[serde(default)]
            required: Option<Vec<String>>,
            #[serde(default)]
            additional_properties: Option<AdditionalProperties>,
            #[serde(default)]
            min_properties: Option<usize>,
            #[serde(default)]
            max_properties: Option<usize>,
            #[serde(default)]
            items: Option<ReferenceOr<Box<Schema>>>,
            #[serde(default)]
            min_items: Option<usize>,
            #[serde(default)]
            max_items: Option<usize>,
            #[serde(default)]
            unique_items: Option<bool>,
            #[serde(rename = "enum", default)]
            enumeration: Option<Vec<serde_json::Value>>,
            #[serde(default)]
            format: Option<String>,
            #[serde(default)]
            min_length: Option<usize>,
            #[serde(default)]
            max_length: Option<usize>,
            #[serde(default)]
            one_of: Option<Vec<ReferenceOr<Schema>>>,
            #[serde(default)]
            all_of: Option<Vec<ReferenceOr<Schema>>>,
            #[serde(default)]
            any_of: Option<Vec<ReferenceOr<Schema>>>,
            #[serde(default)]
            not: Option<Box<ReferenceOr<Schema>>>,
        }

        let any = RawAnySchema::deserialize(deserializer)?;
        match any {
            // String
            RawAnySchema {
                typ: Some(typ),
                pattern,
                multiple_of: None,
                exclusive_minimum: None,
                exclusive_maximum: None,
                minimum: None,
                maximum: None,
                properties: None,
                required: None,
                additional_properties: None,
                min_properties: None,
                max_properties: None,
                items: None,
                min_items: None,
                max_items: None,
                unique_items: None,
                enumeration,
                format,
                min_length,
                max_length,
                one_of: None,
                all_of: None,
                any_of: None,
                not: None,
            } if typ == "string"
                && enumerated_values_valid(&enumeration, serde_json::Value::is_string) =>
            {
                Ok(Self::Type(Type::String(StringType {
                    format: format.into(),
                    pattern,
                    enumeration: enumerated_values_transform(enumeration, |v| {
                        v.as_str().map(String::from)
                    }),
                    min_length,
                    max_length,
                })))
            }

            // Number
            RawAnySchema {
                typ: Some(typ),
                pattern: None,
                multiple_of,
                exclusive_minimum,
                exclusive_maximum,
                minimum,
                maximum,
                properties: None,
                required: None,
                additional_properties: None,
                min_properties: None,
                max_properties: None,
                items: None,
                min_items: None,
                max_items: None,
                unique_items: None,
                enumeration,
                format,
                min_length: None,
                max_length: None,
                one_of: None,
                all_of: None,
                any_of: None,
                not: None,
            } if typ == "number"
                && enumerated_values_valid(&enumeration, serde_json::Value::is_number) =>
            {
                Ok(Self::Type(Type::Number(NumberType {
                    format: format.into(),
                    multiple_of: multiple_of.map(|v| v.as_f64().unwrap()),
                    exclusive_minimum: exclusive_minimum.unwrap_or_default(),
                    exclusive_maximum: exclusive_maximum.unwrap_or_default(),
                    minimum: minimum.map(|v| v.as_f64().unwrap()),
                    maximum: maximum.map(|v| v.as_f64().unwrap()),
                    enumeration: enumerated_values_transform(
                        enumeration,
                        serde_json::Value::as_f64,
                    ),
                })))
            }

            // Integer
            RawAnySchema {
                typ: Some(typ),
                pattern: None,
                multiple_of,
                exclusive_minimum,
                exclusive_maximum,
                minimum,
                maximum,
                properties: None,
                required: None,
                additional_properties: None,
                min_properties: None,
                max_properties: None,
                items: None,
                min_items: None,
                max_items: None,
                unique_items: None,
                enumeration,
                format,
                min_length: None,
                max_length: None,
                one_of: None,
                all_of: None,
                any_of: None,
                not: None,
            } if typ == "integer"
                && enumerated_values_valid(&enumeration, serde_json::Value::is_i64)
                && none_or_int(&multiple_of)
                && none_or_int(&minimum)
                && none_or_int(&maximum) =>
            {
                Ok(Self::Type(Type::Integer(IntegerType {
                    format: format.into(),
                    multiple_of: multiple_of.map(|v| v.as_i64().unwrap()),
                    exclusive_minimum: exclusive_minimum.unwrap_or_default(),
                    exclusive_maximum: exclusive_maximum.unwrap_or_default(),
                    minimum: minimum.map(|v| v.as_i64().unwrap()),
                    maximum: maximum.map(|v| v.as_i64().unwrap()),
                    enumeration: enumerated_values_transform(
                        enumeration,
                        serde_json::Value::as_i64,
                    ),
                })))
            }

            // Boolean
            RawAnySchema {
                typ: Some(typ),
                pattern: None,
                multiple_of: None,
                exclusive_minimum: None,
                exclusive_maximum: None,
                minimum: None,
                maximum: None,
                properties: None,
                required: None,
                additional_properties: None,
                min_properties: None,
                max_properties: None,
                items: None,
                min_items: None,
                max_items: None,
                unique_items: None,
                enumeration,
                format: None,
                min_length: None,
                max_length: None,
                one_of: None,
                all_of: None,
                any_of: None,
                not: None,
            } if typ == "boolean"
                && enumerated_values_valid(&enumeration, serde_json::Value::is_boolean) =>
            {
                Ok(Self::Type(Type::Boolean(BooleanType {
                    enumeration: enumerated_values_transform(
                        enumeration,
                        serde_json::Value::as_bool,
                    ),
                })))
            }

            // Object
            RawAnySchema {
                typ: Some(typ),
                pattern: None,
                multiple_of: None,
                exclusive_minimum: None,
                exclusive_maximum: None,
                minimum: None,
                maximum: None,
                properties,
                required,
                additional_properties,
                min_properties,
                max_properties,
                items: None,
                min_items: None,
                max_items: None,
                unique_items: None,
                enumeration: None,
                format: None,
                min_length: None,
                max_length: None,
                one_of: None,
                all_of: None,
                any_of: None,
                not: None,
            } if typ == "object" => Ok(Self::Type(Type::Object(ObjectType {
                properties: properties.unwrap_or_default(),
                required: required.unwrap_or_default(),
                additional_properties,
                min_properties,
                max_properties,
            }))),

            // Array
            RawAnySchema {
                typ: Some(typ),
                pattern: None,
                multiple_of: None,
                exclusive_minimum: None,
                exclusive_maximum: None,
                minimum: None,
                maximum: None,
                properties: None,
                required: None,
                additional_properties: None,
                min_properties: None,
                max_properties: None,
                items,
                min_items,
                max_items,
                unique_items,
                enumeration: None,
                format: None,
                min_length: None,
                max_length: None,
                one_of: None,
                all_of: None,
                any_of: None,
                not: None,
            } if typ == "array" => Ok(Self::Type(Type::Array(ArrayType {
                items,
                min_items,
                max_items,
                unique_items: unique_items.unwrap_or_default(),
            }))),

            // OneOf
            RawAnySchema {
                typ: None,
                pattern: None,
                multiple_of: None,
                exclusive_minimum: None,
                exclusive_maximum: None,
                minimum: None,
                maximum: None,
                properties: None,
                required: None,
                additional_properties: None,
                min_properties: None,
                max_properties: None,
                items: None,
                min_items: None,
                max_items: None,
                unique_items: None,
                enumeration: None,
                format: None,
                min_length: None,
                max_length: None,
                one_of: Some(one_of),
                all_of: None,
                any_of: None,
                not: None,
            } => Ok(Self::OneOf { one_of }),

            // AllOf
            RawAnySchema {
                typ: None,
                pattern: None,
                multiple_of: None,
                exclusive_minimum: None,
                exclusive_maximum: None,
                minimum: None,
                maximum: None,
                properties: None,
                required: None,
                additional_properties: None,
                min_properties: None,
                max_properties: None,
                items: None,
                min_items: None,
                max_items: None,
                unique_items: None,
                enumeration: None,
                format: None,
                min_length: None,
                max_length: None,
                one_of: None,
                all_of: Some(all_of),
                any_of: None,
                not: None,
            } => Ok(Self::AllOf { all_of }),

            // AnyOf
            RawAnySchema {
                typ: None,
                pattern: None,
                multiple_of: None,
                exclusive_minimum: None,
                exclusive_maximum: None,
                minimum: None,
                maximum: None,
                properties: None,
                required: None,
                additional_properties: None,
                min_properties: None,
                max_properties: None,
                items: None,
                min_items: None,
                max_items: None,
                unique_items: None,
                enumeration: None,
                format: None,
                min_length: None,
                max_length: None,
                one_of: None,
                all_of: None,
                any_of: Some(any_of),
                not: None,
            } => Ok(Self::AnyOf { any_of }),

            // Not
            RawAnySchema {
                typ: None,
                pattern: None,
                multiple_of: None,
                exclusive_minimum: None,
                exclusive_maximum: None,
                minimum: None,
                maximum: None,
                properties: None,
                required: None,
                additional_properties: None,
                min_properties: None,
                max_properties: None,
                items: None,
                min_items: None,
                max_items: None,
                unique_items: None,
                enumeration: None,
                format: None,
                min_length: None,
                max_length: None,
                one_of: None,
                all_of: None,
                any_of: None,
                not: Some(not),
            } => Ok(Self::Not { not }),

            // Any
            RawAnySchema {
                typ,
                pattern,
                multiple_of,
                exclusive_minimum,
                exclusive_maximum,
                minimum,
                maximum,
                properties,
                required,
                additional_properties,
                min_properties,
                max_properties,
                items,
                min_items,
                max_items,
                unique_items,
                enumeration,
                format,
                min_length,
                max_length,
                one_of,
                all_of,
                any_of,
                not,
            } => Ok(Self::Any(AnySchema {
                typ,
                pattern,
                multiple_of: multiple_of.map(|n| n.as_f64().unwrap()),
                exclusive_minimum,
                exclusive_maximum,
                minimum: minimum.map(|n| n.as_f64().unwrap()),
                maximum: maximum.map(|n| n.as_f64().unwrap()),
                properties: properties.unwrap_or_default(),
                required: required.unwrap_or_default(),
                additional_properties,
                min_properties,
                max_properties,
                items,
                min_items,
                max_items,
                unique_items,
                enumeration: enumeration.unwrap_or_default(),
                format,
                min_length,
                max_length,
                one_of: one_of.unwrap_or_default(),
                all_of: all_of.unwrap_or_default(),
                any_of: any_of.unwrap_or_default(),
                not,
            })),
        }
    }
}

fn none_or_int(value: &Option<serde_json::Number>) -> bool {
    match value {
        None => true,
        Some(x) => x.is_i64(),
    }
}

fn enumerated_values_transform<T, F>(
    enumeration: Option<Vec<serde_json::Value>>,
    transform: F,
) -> Vec<Option<T>>
where
    F: Fn(&serde_json::Value) -> Option<T>,
{
    match enumeration {
        Some(values) => values
            .iter()
            .map(|v| {
                if v.is_null() {
                    None
                } else {
                    Some(transform(v).unwrap())
                }
            })
            .collect::<Vec<_>>(),
        None => Default::default(),
    }
}

fn enumerated_values_valid<F>(enumeration: &Option<Vec<serde_json::Value>>, check: F) -> bool
where
    F: Fn(&serde_json::Value) -> bool,
{
    match enumeration {
        Some(values) => values.iter().all(|value| value.is_null() || check(value)),
        None => true,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Type {
    String(StringType),
    Number(NumberType),
    Integer(IntegerType),
    Object(ObjectType),
    Array(ArrayType),
    Boolean(BooleanType),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum AdditionalProperties {
    Any(bool),
    Schema(Box<ReferenceOr<Schema>>),
}

/// Catch-all for any combination of properties that doesn't correspond to one
/// of the predefined subsets.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AnySchema {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub typ: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multiple_of: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclusive_minimum: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclusive_maximum: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub properties: IndexMap<String, ReferenceOr<Box<Schema>>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<AdditionalProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_properties: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_properties: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<ReferenceOr<Box<Schema>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_items: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_items: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unique_items: Option<bool>,
    #[serde(rename = "enum", default, skip_serializing_if = "Vec::is_empty")]
    pub enumeration: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_length: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_length: Option<usize>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub one_of: Vec<ReferenceOr<Schema>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub all_of: Vec<ReferenceOr<Schema>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub any_of: Vec<ReferenceOr<Schema>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub not: Option<Box<ReferenceOr<Schema>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StringType {
    #[serde(default, skip_serializing_if = "VariantOrUnknownOrEmpty::is_empty")]
    pub format: VariantOrUnknownOrEmpty<StringFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(rename = "enum", default, skip_serializing_if = "Vec::is_empty")]
    pub enumeration: Vec<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NumberType {
    #[serde(default, skip_serializing_if = "VariantOrUnknownOrEmpty::is_empty")]
    pub format: VariantOrUnknownOrEmpty<NumberFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_of: Option<f64>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub exclusive_minimum: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub exclusive_maximum: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
    #[serde(rename = "enum", default, skip_serializing_if = "Vec::is_empty")]
    pub enumeration: Vec<Option<f64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IntegerType {
    #[serde(default, skip_serializing_if = "VariantOrUnknownOrEmpty::is_empty")]
    pub format: VariantOrUnknownOrEmpty<IntegerFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_of: Option<i64>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub exclusive_minimum: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub exclusive_maximum: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i64>,
    #[serde(rename = "enum", default, skip_serializing_if = "Vec::is_empty")]
    pub enumeration: Vec<Option<i64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ObjectType {
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub properties: IndexMap<String, ReferenceOr<Box<Schema>>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<AdditionalProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_properties: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_properties: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ArrayType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<ReferenceOr<Box<Schema>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_items: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<usize>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub unique_items: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BooleanType {
    #[serde(rename = "enum", default, skip_serializing_if = "Vec::is_empty")]
    pub enumeration: Vec<Option<bool>>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum NumberFormat {
    Float,
    Double,
}

impl FromStr for NumberFormat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "float" => Ok(Self::Float),
            "double" => Ok(Self::Double),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum IntegerFormat {
    Int32,
    Int64,
}

impl FromStr for IntegerFormat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "int32" => Ok(Self::Int32),
            "int64" => Ok(Self::Int64),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum StringFormat {
    Date,
    DateTime,
    Password,
    Byte,
    Binary,
}

impl FromStr for StringFormat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "date" => Ok(Self::Date),
            "date-time" => Ok(Self::DateTime),
            "password" => Ok(Self::Password),
            "byte" => Ok(Self::Byte),
            "binary" => Ok(Self::Binary),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::{
        AnySchema, Schema, SchemaData, SchemaKind, StringType, Type, VariantOrUnknownOrEmpty,
    };

    #[test]
    fn test_schema_with_extensions() {
        let schema = serde_json::from_str::<Schema>(
            r#"{
                "type": "boolean",
                "x-foo": "bar"
            }"#,
        )
        .unwrap();

        assert_eq!(
            schema.schema_data.extensions.get("x-foo"),
            Some(&json!("bar"))
        );
    }

    #[test]
    fn test_any() {
        let value = json! { {} };
        serde_json::from_value::<AnySchema>(value).unwrap();
    }

    #[test]
    fn test_not() {
        let value = json! {
            {
                "not": {}
            }
        };

        let schema = serde_json::from_value::<Schema>(value).unwrap();
        assert!(matches!(schema.schema_kind, SchemaKind::Not { not: _ }));
    }

    #[test]
    fn test_null() {
        let value = json! {
            {
                "nullable": true,
                "enum": [ null ],
            }
        };

        let schema = serde_json::from_value::<Schema>(value).unwrap();
        assert!(matches!(
            &schema.schema_data,
            SchemaData { nullable: true, .. }
        ));
        assert!(matches!(
            &schema.schema_kind,
            SchemaKind::Any(AnySchema { enumeration, .. }) if enumeration[0] == json!(null)));
    }

    #[test]
    fn test_object_and_one_of() {
        let value = json! {
            {
                "type": "object",
                "nullable": true,
                "description": "xyz",
                "properties": {
                    "a": {},
                    "b": {},
                    "c": {}
                },
                "oneOf": [
                    { "required": ["a"] },
                    { "required": ["b"] },
                    { "required": ["c"] }
                ],
                "x-foo": "bar"
            }
        };

        let schema = serde_json::from_value::<Schema>(value).unwrap();
        assert!(schema.schema_data.nullable);
        assert_eq!(schema.schema_data.extensions.get("x-foo").unwrap(), "bar");

        match schema.schema_kind {
            SchemaKind::Any(AnySchema {
                typ,
                properties,
                one_of,
                ..
            }) => {
                assert_eq!(typ.unwrap(), "object");
                assert_eq!(properties.len(), 3);
                assert_eq!(one_of.len(), 3);
            }
            _ => panic!("incorrect kind {:#?}", schema),
        }
    }

    #[test]
    fn test_enum_with_null() {
        let value = json! {
            {
                "type": "string",
                "nullable": true,
                "enum": [ null, "howdy" ]
            }
        };

        let schema = serde_json::from_value::<Schema>(value).unwrap();
        assert!(schema.schema_data.nullable);

        match schema.schema_kind {
            SchemaKind::Type(Type::String(StringType {
                format: VariantOrUnknownOrEmpty::Empty,
                pattern: None,
                enumeration,
                min_length: None,
                max_length: None,
            })) => {
                assert_eq!(enumeration, vec![None, Some("howdy".to_string())]);
            }
            _ => panic!("incorrect kind {:#?}", schema),
        }
    }
}
