use crate::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
#[derive(Debug, Clone, Serialize, Deserialize)]
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Schema {
    Schema(Box<SchemaVariant>),
    Any(Box<AnySchema>),
    OneOf {
        #[serde(default, rename = "oneOf")]
        one_of: Vec<ReferenceOr<Schema>>,
    },
    AllOf {
        #[serde(default, rename = "allOf")]
        all_of: Vec<ReferenceOr<Schema>>,
    },
    AnyOf {
        #[serde(default, rename = "anyOf")]
        any_of: Vec<ReferenceOr<Schema>>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnySchema {
    #[serde(flatten)]
    schema_data: SchemaData,
    #[serde(skip_serializing_if = "Option::is_none")]
    pattern: Option<String>,
    #[serde(rename = "multipleOf", skip_serializing_if = "Option::is_none")]
    multiple_of: Option<f64>,
    #[serde(rename = "exclusiveMinimum", skip_serializing_if = "Option::is_none")]
    exclusive_minimum: Option<bool>,
    #[serde(rename = "exclusiveMaximum", skip_serializing_if = "Option::is_none")]
    exclusive_maximum: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum: Option<f64>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    properties: BTreeMap<String, ReferenceOr<Box<Schema>>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    required: Vec<String>,
    #[serde(
        default,
        rename = "additionalProperties",
        skip_serializing_if = "is_false"
    )]
    additional_properties: bool, //@todo support this as an empty object
    #[serde(rename = "minProperties", skip_serializing_if = "Option::is_none")]
    min_propeties: Option<usize>,
    #[serde(rename = "maxProperties", skip_serializing_if = "Option::is_none")]
    max_properties: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<ReferenceOr<Box<Schema>>>, //@todo Mixed type arrays using oneOf
    #[serde(rename = "minItems", skip_serializing_if = "Option::is_none")]
    min_items: Option<usize>,
    #[serde(rename = "maxItems", skip_serializing_if = "Option::is_none")]
    max_items: Option<usize>,
    #[serde(rename = "uniqueItems", skip_serializing_if = "Option::is_none")]
    unique_items: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<String>,
}

//@todo  This breaks things
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SchemaVariant {
    #[serde(rename = "string")]
    String {
        #[serde(default, skip_serializing_if = "VariantOrUnknownOrEmpty::is_empty")]
        format: VariantOrUnknownOrEmpty<StringFormat>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pattern: Option<String>,
        #[serde(flatten)]
        schema_data: SchemaData,
        #[serde(rename = "enum", default, skip_serializing_if = "Vec::is_empty")]
        enumeration: Vec<String>,
    },
    #[serde(rename = "number")]
    Number {
        #[serde(default, skip_serializing_if = "VariantOrUnknownOrEmpty::is_empty")]
        format: VariantOrUnknownOrEmpty<NumberFormat>,
        #[serde(flatten)]
        schema_data: SchemaData,
        #[serde(rename = "multipleOf")]
        #[serde(skip_serializing_if = "Option::is_none")]
        multiple_of: Option<f64>,
        #[serde(default, rename = "exclusiveMinimum", skip_serializing_if = "is_false")]
        exclusive_minimum: bool,
        #[serde(default, rename = "exclusiveMaximum", skip_serializing_if = "is_false")]
        exclusive_maximum: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        minimum: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        maximum: Option<f64>,
        #[serde(rename = "enum", default, skip_serializing_if = "Vec::is_empty")]
        enumeration: Vec<f64>,
    },
    #[serde(rename = "integer")]
    Integer {
        #[serde(default, skip_serializing_if = "VariantOrUnknownOrEmpty::is_empty")]
        format: VariantOrUnknownOrEmpty<IntegerFormat>,
        #[serde(flatten)]
        schema_data: SchemaData,
        #[serde(rename = "multipleOf")]
        #[serde(skip_serializing_if = "Option::is_none")]
        multiple_of: Option<i64>,
        #[serde(default, rename = "exclusiveMinimum", skip_serializing_if = "is_false")]
        exclusive_minimum: bool,
        #[serde(default, rename = "exclusiveMaximum", skip_serializing_if = "is_false")]
        exclusive_maximum: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        minimum: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        maximum: Option<i64>,
        #[serde(rename = "enum", default, skip_serializing_if = "Vec::is_empty")]
        enumeration: Vec<i64>,
    },
    #[serde(rename = "object")]
    Object {
        #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
        properties: BTreeMap<String, ReferenceOr<Box<Schema>>>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        required: Vec<String>,
        #[serde(
            default,
            rename = "additionalProperties",
            skip_serializing_if = "is_false"
        )]
        additional_properties: bool, //@todo support this as an empty object
        #[serde(rename = "minProperties")]
        #[serde(skip_serializing_if = "Option::is_none")]
        min_properties: Option<usize>,
        #[serde(rename = "maxProperties")]
        #[serde(skip_serializing_if = "Option::is_none")]
        max_properties: Option<usize>,
        #[serde(flatten)]
        schema_data: SchemaData,
    },
    #[serde(rename = "array")]
    Array {
        items: ReferenceOr<Box<Schema>>, //@todo Mixed type arrays using oneOf
        #[serde(rename = "minItems")]
        #[serde(skip_serializing_if = "Option::is_none")]
        min_items: Option<usize>,
        #[serde(rename = "maxItems")]
        #[serde(skip_serializing_if = "Option::is_none")]
        max_items: Option<usize>,
        #[serde(default, rename = "uniqueItems", skip_serializing_if = "is_false")]
        unique_items: bool,
        #[serde(flatten)]
        schema_data: SchemaData,
    },
    #[serde(rename = "boolean")]
    Boolean {
        #[serde(flatten)]
        schema_data: SchemaData,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NumberFormat {
    #[serde(rename = "float")]
    Float,
    #[serde(rename = "double")]
    Double,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntegerFormat {
    #[serde(rename = "int32")]
    Int32,
    #[serde(rename = "int64")]
    Int64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StringFormat {
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "datetime")]
    DateTime,
    #[serde(rename = "password")]
    Password,
    #[serde(rename = "byte")]
    Byte,
    #[serde(rename = "binary")]
    Binary,
}
