use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Describes a single operation parameter.
///
/// A unique parameter is defined by a combination of a name and location.
pub struct Parameter {
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
    /// REQUIRED. The location of the parameter. Possible values
    /// are "query", "header", "path" or "cookie".
    #[serde(rename = "in")]
    pub location: ParameterLocation,
    /// A brief description of the parameter. This could
    /// contain examples of use. CommonMark syntax MAY be
    /// used for rich text representation.
    pub description: Option<String>,
    /// Determines whether this parameter is mandatory.
    /// If the parameter location is "path", this property
    /// is REQUIRED and its value MUST be true. Otherwise,
    /// the property MAY be included and its default value
    /// is false.
    #[serde(default)]
    pub required: bool,
    /// Specifies that a parameter is deprecated and SHOULD
    /// be transitioned out of usage.
    pub deprecated: Option<bool>,
    /// Sets the ability to pass empty-valued parameters. This is
    /// valid only for query parameters and allows sending a parameter
    /// with an empty value. Default value is false. If style is used,
    /// and if behavior is n/a (cannot be serialized), the value of
    /// allowEmptyValue SHALL be ignored.
    #[serde(rename = "allowEmptyValue")]
    pub allow_empty_value: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParameterLocation {
    #[serde(rename = "query")]
    Query,
    #[serde(rename = "header")]
    Header,
    #[serde(rename = "path")]
    Path,
    #[serde(rename = "cookie")]
    Cookie,
}
