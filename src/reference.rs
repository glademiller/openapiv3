use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use crate::{OpenAPI, Parameter, RequestBody, Response, Schema};

/// Represents a reference to an OpenAPI Schema. This should probably be moved to openapiv3-extended
/// e.g. #/components/schemas/Account or #/components/schemas/Account/properties/name
pub enum SchemaReference {
    Schema {
        schema: String,
    },
    Property {
        schema: String,
        property: String,
    },
}

impl SchemaReference {
    pub fn from_str(reference: &str) -> Self {
        let mut ns = reference.rsplit('/');
        let name = ns.next().unwrap();
        match ns.next().unwrap() {
            "schemas" => {
                Self::Schema {
                    schema: name.to_string(),
                }
            }
            "properties" => {
                let schema_name = ns.next().unwrap();
                Self::Property {
                    schema: schema_name.to_string(),
                    property: name.to_string(),
                }
            }
            _ => panic!("Unknown reference: {}", reference),
        }
    }
}


impl std::fmt::Display for SchemaReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SchemaReference::Schema { schema } => write!(f, "#/components/schemas/{}", schema),
            SchemaReference::Property { schema, property } => write!(f, "#/components/schemas/{}/properties/{}", schema, property),
        }
    }
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ReferenceOr<T> {
    Reference {
        #[serde(rename = "$ref")]
        reference: String,
    },
    Item(T),
}

impl<T> ReferenceOr<T> {
    pub fn ref_(r: &str) -> Self {
        ReferenceOr::Reference {
            reference: r.to_owned(),
        }
    }
    pub fn item(item: T) -> Self {
        ReferenceOr::Item(item)
    }
    pub fn reference(r: &str) -> Self {
        ReferenceOr::Reference {
            reference: r.to_owned(),
        }
    }

    pub fn boxed_item(item: T) -> ReferenceOr<Box<T>> {
        ReferenceOr::Item(Box::new(item))
    }

    /// Converts this [ReferenceOr] to the item inside, if it exists.
    ///
    /// The return value will be [Option::Some] if this was a [ReferenceOr::Item] or [Option::None] if this was a [ReferenceOr::Reference].
    ///
    /// # Examples
    ///
    /// ```
    /// # use openapiv3::ReferenceOr;
    ///
    /// let i = ReferenceOr::Item(1);
    /// assert_eq!(i.into_item(), Some(1));
    ///
    /// let j: ReferenceOr<u8> = ReferenceOr::Reference { reference: String::new() };
    /// assert_eq!(j.into_item(), None);
    /// ```
    pub fn into_item(self) -> Option<T> {
        match self {
            ReferenceOr::Reference { .. } => None,
            ReferenceOr::Item(i) => Some(i),
        }
    }

    /// Returns a reference to to the item inside this [ReferenceOr], if it exists.
    ///
    /// The return value will be [Option::Some] if this was a [ReferenceOr::Item] or [Option::None] if this was a [ReferenceOr::Reference].
    ///
    /// # Examples
    ///
    /// ```
    /// # use openapiv3::ReferenceOr;
    ///
    /// let i = ReferenceOr::Item(1);
    /// assert_eq!(i.as_item(), Some(&1));
    ///
    /// let j: ReferenceOr<u8> = ReferenceOr::Reference { reference: String::new() };
    /// assert_eq!(j.as_item(), None);
    /// ```
    // TODO i believe this should be called as_ref() ?
    pub fn as_item(&self) -> Option<&T> {
        match self {
            ReferenceOr::Reference { .. } => None,
            ReferenceOr::Item(i) => Some(i),
        }
    }

    pub fn as_ref_str(&self) -> Option<&str> {
        match self {
            ReferenceOr::Reference { reference } => Some(reference),
            ReferenceOr::Item(_) => None,
        }
    }

    pub fn as_mut(&mut self) -> Option<&mut T> {
        match self {
            ReferenceOr::Reference { .. } => None,
            ReferenceOr::Item(i) => Some(i),
        }
    }
}

impl<T: 'static> ReferenceOr<T> {
    pub fn as_ref(&self) -> ReferenceOr<&T> {
        match self {
            ReferenceOr::Reference { reference } => ReferenceOr::Reference { reference: reference.clone() },
            ReferenceOr::Item(i) => ReferenceOr::Item(i),
        }
    }
}


impl ReferenceOr<Schema> {
    pub fn resolve<'a>(&'a self, spec: &'a OpenAPI) -> &'a Schema {
        match self {
            ReferenceOr::Reference { reference } => {
                let reference = SchemaReference::from_str(&reference);
                match &reference {
                    SchemaReference::Schema { ref schema } => {
                        let schema_ref = spec.schemas().get(schema)
                            .expect(&format!("Schema {} not found in OpenAPI spec.", schema));
                        // In theory both this as_item and the one below could have continue to be references
                        // but assum
                        schema_ref.as_item()
                            .expect(&format!("The schema {} was used in a reference, but that schema is itself a reference to another schema.", schema))
                    }
                    SchemaReference::Property { schema: ref schema_name, ref property } => {
                        let schema = spec.schemas().get(schema_name)
                            .expect(&format!("Schema {} not found in OpenAPI spec.", schema_name))
                            .as_item()
                            .expect(&format!("The schema {} was used in a reference, but that schema is itself a reference to another schema.", schema_name));
                        let prop_schema = schema
                            .properties()
                            .expect(&format!("Tried to resolve reference {}, but {} is not an object with properties.", reference, schema_name))
                            .get(property)
                            .expect(&format!("Schema {} does not have property {}.", schema_name, property));
                        prop_schema.resolve(spec)
                    }
                }
            }
            ReferenceOr::Item(schema) => schema,
        }
    }
}

impl ReferenceOr<Box<Schema>> {
    pub fn unbox(&self) -> ReferenceOr<Schema> {
        match self {
            ReferenceOr::Reference { reference } => ReferenceOr::Reference { reference: reference.clone() },
            ReferenceOr::Item(boxed) => ReferenceOr::Item(*boxed.clone()),
        }
    }
}


impl ReferenceOr<Parameter> {
    pub fn resolve<'a>(&'a self, spec: &'a OpenAPI) -> Result<&'a Parameter> {
        match self {
            ReferenceOr::Reference { reference } => {
                let name = get_parameter_name(&reference)?;
                let components = spec.components.as_ref().unwrap();
                components.parameters.get(name)
                    .ok_or(anyhow!("{} not found in OpenAPI spec.", reference))?
                    .as_item()
                    .ok_or(anyhow!("{} is circular.", reference))
            }
            ReferenceOr::Item(parameter) => Ok(parameter),
        }
    }
}


fn parse_reference<'a>(reference: &'a str, group: &str) -> Result<&'a str> {
    let mut parts = reference.rsplitn(2, '/');
    let name = parts.next();
    name.filter(|_| matches!(parts.next(), Some(x) if format!("#/components/{group}") == x))
        .ok_or(anyhow!("Invalid {} reference: {}", group, reference))
}


fn get_response_name(reference: &str) -> Result<&str> {
    parse_reference(reference, "responses")
}


fn get_request_body_name(reference: &str) -> Result<&str> {
    parse_reference(reference, "requestBodies")
}

fn get_parameter_name(reference: &str) -> Result<&str> {
    parse_reference(reference, "parameters")
}

impl ReferenceOr<Response> {
    pub fn resolve<'a>(&'a self, spec: &'a OpenAPI) -> Result<&'a Response> {
        match self {
            ReferenceOr::Reference { reference } => {
                let name = get_response_name(&reference)?;
                let components = spec.components.as_ref().unwrap();
                components.responses.get(name)
                    .ok_or(anyhow!("{} not found in OpenAPI spec.", reference))?
                    .as_item()
                    .ok_or(anyhow!("{} is circular.", reference))
            }
            ReferenceOr::Item(response) => Ok(response),
        }
    }
}


impl ReferenceOr<RequestBody> {
    pub fn resolve<'a>(&'a self, spec: &'a OpenAPI) -> Result<&'a RequestBody> {
        match self {
            ReferenceOr::Reference { reference } => {
                let name = get_request_body_name(&reference)?;
                let components = spec.components.as_ref().unwrap();
                components.request_bodies.get(name)
                    .ok_or(anyhow!("{} not found in OpenAPI spec.", reference))?
                    .as_item()
                    .ok_or(anyhow!("{} is circular.", reference))
            }
            ReferenceOr::Item(request_body) => Ok(request_body),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_request_body_name() {
        assert!(matches!(get_request_body_name("#/components/requestBodies/Foo"), Ok("Foo")));
        assert!(get_request_body_name("#/components/schemas/Foo").is_err());
    }
}