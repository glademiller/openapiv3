use crate::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Describes the operations available on a single path.
/// A Path Item MAY be empty, due to ACL constraints.
/// The path itself is still exposed to the documentation
/// viewer but they will not know which operations and
/// parameters are available.
pub struct PathItem {
    pub get: Option<Operation>,
    pub put: Option<Operation>,
    pub post: Option<Operation>,
    pub delete: Option<Operation>,
    pub options: Option<Operation>,
    pub head: Option<Operation>,
    pub patch: Option<Operation>,
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
}

/// Holds the relative paths to the individual endpoints and
/// their operations. The path is appended to the URL from the
/// Server Object in order to construct the full URL. The Paths
/// MAY be empty, due to ACL constraints.
pub type Paths = BTreeMap<String, ReferenceOr<PathItem>>;
