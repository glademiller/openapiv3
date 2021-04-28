use crate::*;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// Describes the operations available on a single path.
/// A Path Item MAY be empty, due to ACL constraints.
/// The path itself is still exposed to the documentation
/// viewer but they will not know which operations and
/// parameters are available.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PathItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub put: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace: Option<Operation>,
    /// An alternative server array to service all operations in this path.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub servers: Vec<Server>,
    /// A list of parameters that are applicable for all the
    /// operations described under this path. These parameters
    /// can be overridden at the operation level, but cannot be
    /// removed there. The list MUST NOT include duplicated parameters.
    /// A unique parameter is defined by a combination of a name and location.
    /// The list can use the Reference Object to link to parameters that
    /// are defined at the OpenAPI Object's components/parameters.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ReferenceOr<Parameter>>,
    /// Inline extensions to this object.
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}

impl PathItem {
    pub fn iter(&self) -> impl Iterator<Item = &Operation> + '_ {
        vec![
            &self.get,
            &self.put,
            &self.post,
            &self.delete,
            &self.options,
            &self.head,
            &self.patch,
            &self.trace,
        ]
        .into_iter()
        .flat_map(|o| o.iter())
    }
}

/// Holds the relative paths to the individual endpoints and
/// their operations. The path is appended to the URL from the
/// Server Object in order to construct the full URL. The Paths
/// MAY be empty, due to ACL constraints.
pub type Paths = IndexMap<String, ReferenceOr<PathItem>>;
