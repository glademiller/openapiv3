use crate::*;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// Describes a single operation parameter.
///
/// A unique parameter is defined by a combination of a name and location.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParameterData {
    /// REQUIRED. The name of the parameter. Parameter names are case sensitive.
    /// If in is "path", the name field MUST correspond to the associated path
    /// segment from the path field in the Paths Object. See Path Templating for
    /// further information.
    ///
    /// If in is "header" and the name field is "Accept", "Content-Type" or
    /// "Authorization", the parameter definition SHALL be ignored.
    ///
    /// For all other cases, the name corresponds to the parameter name
    /// used by the in property.
    pub name: String,
    /// A brief description of the parameter. This could
    /// contain examples of use. CommonMark syntax MAY be
    /// used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Determines whether this parameter is mandatory.
    /// If the parameter location is "path", this property
    /// is REQUIRED and its value MUST be true. Otherwise,
    /// the property MAY be included and its default value
    /// is false.
    #[serde(default, skip_serializing_if = "is_false")]
    pub required: bool,
    /// Specifies that a parameter is deprecated and SHOULD
    /// be transitioned out of usage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    #[serde(flatten)]
    pub format: ParameterSchemaOrContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub examples: IndexMap<String, ReferenceOr<Example>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explode: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ParameterSchemaOrContent {
    #[serde(rename = "schema")]
    Schema(ReferenceOr<Schema>),
    #[serde(rename = "content")]
    Content(Content),
}

pub type Content = IndexMap<String, MediaType>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "in")]
pub enum Parameter {
    #[serde(rename = "query")]
    Query {
        #[serde(flatten)]
        parameter_data: ParameterData,
        #[serde(default)]
        #[serde(rename = "allowReserved", skip_serializing_if = "is_false")]
        allow_reserved: bool,
        #[serde(default)]
        style: QueryStyle,
        /// Sets the ability to pass empty-valued parameters. This is
        /// valid only for query parameters and allows sending a parameter
        /// with an empty value. Default value is false. If style is used,
        /// and if behavior is n/a (cannot be serialized), the value of
        /// allowEmptyValue SHALL be ignored.
        #[serde(rename = "allowEmptyValue", skip_serializing_if = "Option::is_none")]
        allow_empty_value: Option<bool>,
    },
    #[serde(rename = "header")]
    Header {
        #[serde(flatten)]
        parameter_data: ParameterData,
        #[serde(default)]
        style: HeaderStyle,
    },
    #[serde(rename = "path")]
    Path {
        #[serde(flatten)]
        parameter_data: ParameterData,
        #[serde(default)]
        style: PathStyle,
    },
    #[serde(rename = "cookie")]
    Cookie {
        #[serde(flatten)]
        parameter_data: ParameterData,
        #[serde(default)]
        style: CookieStyle,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum PathStyle {
    #[serde(rename = "matrix")]
    Matrix,
    #[serde(rename = "label")]
    Label,
    #[serde(rename = "simple")]
    Simple,
}

impl Default for PathStyle {
    fn default() -> Self {
        PathStyle::Simple
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum QueryStyle {
    #[serde(rename = "form")]
    Form,
    #[serde(rename = "spaceDelimited")]
    SpaceDelimited,
    #[serde(rename = "pipeDelimited")]
    PipeDelimited,
    #[serde(rename = "deepObject")]
    DeepObject,
}

impl Default for QueryStyle {
    fn default() -> Self {
        QueryStyle::Form
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum CookieStyle {
    #[serde(rename = "form")]
    Form,
}

impl Default for CookieStyle {
    fn default() -> CookieStyle {
        CookieStyle::Form
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum HeaderStyle {
    #[serde(rename = "simple")]
    Simple,
}

impl Default for HeaderStyle {
    fn default() -> HeaderStyle {
        HeaderStyle::Simple
    }
}
