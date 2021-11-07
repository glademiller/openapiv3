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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
        #[serde(rename = "not")]
        not: Box<ReferenceOr<Schema>>,
    },
    Any(AnySchema),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Type {
    String(StringType),
    Number(NumberType),
    Integer(IntegerType),
    Object(ObjectType),
    Array(ArrayType),
    Boolean {},
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum AdditionalProperties {
    Any(bool),
    Schema(Box<ReferenceOr<Schema>>),
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AnySchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_of: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_minimum: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_maximum: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<ReferenceOr<Box<Schema>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_items: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_items: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
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
    pub enumeration: Vec<f64>,
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
    pub enumeration: Vec<i64>,
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum NumberFormat {
    Float,
    Double,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum IntegerFormat {
    Int32,
    Int64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum StringFormat {
    Date,
    #[serde(rename = "date-time")]
    DateTime,
    Password,
    Byte,
    Binary,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::{Schema, SchemaKind};

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
    fn test_not() {
        let value = json! {
            {
                "not": {}
            }
        };

        let schema = serde_json::from_value::<Schema>(value).unwrap();
        assert!(matches!(schema.schema_kind, SchemaKind::Not { not: _ }));
    }
}
