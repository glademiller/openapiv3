use std::convert::TryInto;
use indexmap::IndexMap;
use crate as v3;
use crate::StatusCode;
use super::schema as v2;

trait TryRemove<T> {
    fn try_remove(&mut self, i: usize) -> Option<T>;
}

impl<T> TryRemove<T> for Vec<T> {
    fn try_remove(&mut self, i: usize) -> Option<T> {
        self.get(i)?;
        Some(self.remove(i))
    }
}

impl Into<v3::OpenAPI> for v2::OpenAPI {
    fn into(self) -> v3::OpenAPI {
        let v2::OpenAPI {
            swagger: _,
            info,
            host,
            base_path,
            schemes,
            consumes: _,
            produces: _,
            paths,
            definitions,
            parameters,
            responses,
            security_definitions,
            security,
            tags,
            external_docs,
        } = self;
        let mut components = v3::Components::default();

        components.schemas = definitions
            .unwrap_or_default()
            .into_iter()
            .map(|(k, v)| (k, v3::ReferenceOr::Item(v.into())))
            .collect();

        components.parameters = parameters
            .unwrap_or_default()
            .into_iter()
            .filter_map(|(k, v)| {
                let v: v3::ReferenceOr<v3::Parameter> = v.try_into().ok()?;
                Some((k, v))
            })
            .collect();

        components.responses = responses
            .unwrap_or_default()
            .into_iter()
            .map(|(k, v)| (k, v.into()))
            .collect();

        components.security_schemes = security_definitions
            .unwrap_or_default()
            .into_iter()
            .map(|(k, v)| (k, v.into()))
            .collect();

        v3::OpenAPI {
            openapi: "3.0.3".to_string(),
            info: info.into(),
            servers: host
                .map(|h| {
                    let scheme = schemes.and_then(|mut s| if s.len() >= 1 {
                        Some(s.remove(0))
                    } else {
                        None
                    })
                        .map(|s| s.as_str())
                        .unwrap_or("http");
                    let url = format!("{}://{}{}", scheme, h, base_path.unwrap_or_default());
                    vec![v3::Server {
                        url,
                        ..v3::Server::default()
                    }]
                }).unwrap_or_default(),
            paths: paths.into(),
            components: Some(components),
            security,
            tags: tags.unwrap_or_default()
                .into_iter()
                .map(|t| t.into())
                .collect(),
            external_docs: external_docs
                .and_then(|mut e| e.try_remove(0))
                .map(|e| e.into()),
            extensions: Default::default(),
        }
    }
}

impl Into<v3::Paths> for IndexMap<String, v2::PathItem> {
    fn into(self) -> v3::Paths {
        v3::Paths {
            paths: self.into_iter().map(|(k, v)| (k, v.into())).collect(),
            extensions: Default::default(),
        }
    }
}

impl Into<v3::ReferenceOr<v3::PathItem>> for v2::PathItem {
    fn into(self) -> v3::ReferenceOr<v3::PathItem> {
        let v2::PathItem {
            get,
            put,
            post,
            delete,
            options,
            head,
            patch,
            parameters,
        } = self;
        v3::ReferenceOr::Item(v3::PathItem {
            summary: None,
            description: None,
            get: get.map(|op| op.into()),
            put: put.map(|op| op.into()),
            post: post.map(|op| op.into()),
            delete: delete.map(|op| op.into()),
            options: options.map(|op| op.into()),
            head: head.map(|op| op.into()),
            patch: patch.map(|op| op.into()),
            trace: None,
            servers: vec![],
            parameters: parameters
                .unwrap_or_default()
                .into_iter()
                .flat_map(|p| p.try_into().ok())
                .collect(),
            extensions: Default::default(),
        })
    }
}

/// Change something like "#/definitions/User" to "#/components/schemas/User"
fn rewrite_ref(s: &str) -> String {
    s.replace("#/definitions/", "#/components/schemas/")
}

fn build_schema_kind(type_: &str, format: Option<String>) -> v3::SchemaKind {
    match type_ {
        "string" => v3::SchemaKind::Type(v3::Type::String(v3::StringType {
            format: {
                let s = serde_json::to_string(&format).unwrap();
                serde_json::from_str(&s).unwrap()
            },
            ..v3::StringType::default()
        })),
        "number" => v3::SchemaKind::Type(v3::Type::Number(v3::NumberType {
            format: {
                let s = serde_json::to_string(&format).unwrap();
                serde_json::from_str(&s).unwrap()
            },
            ..v3::NumberType::default()
        })),
        "integer" => v3::SchemaKind::Type(v3::Type::Integer(v3::IntegerType {
            format: {
                let s = serde_json::to_string(&format).unwrap();
                serde_json::from_str(&s).unwrap()
            },
            ..v3::IntegerType::default()
        })),
        "boolean" => v3::SchemaKind::Type(v3::Type::Boolean {}),
        "array" => v3::SchemaKind::Type(v3::Type::Array(v3::ArrayType {
            ..v3::ArrayType::default()
        })),
        "object" => {
            let object_type = v3::ObjectType::default();
            v3::SchemaKind::Type(v3::Type::Object(object_type))
        }
        _ => panic!("Unknown schema type: {}", type_),
    }
}

impl Into<v3::Schema> for v2::Schema {
    fn into(self) -> v3::Schema {
        let v2::Schema {
            description,
            schema_type,
            format,
            enum_values,
            required,
            items,
            properties,
            all_of,
            other,
        } = self;

        let schema_data = v3::SchemaData {
            description,
            extensions: other,
            ..v3::SchemaData::default()
        };

        if let Some(all_of) = all_of {
            return v3::Schema {
                schema_data,
                schema_kind: v3::SchemaKind::AllOf {
                    all_of: all_of
                        .into_iter()
                        .map(|s| s.into())
                        .collect()
                },
            }
        }

        let schema_type = schema_type.unwrap_or_else(|| "object".to_string());
        let mut schema_kind = build_schema_kind(&schema_type, format);

        match &mut schema_kind {
            v3::SchemaKind::Type(v3::Type::String(ref mut s)) => {
                s.enumeration = enum_values.unwrap_or_default();
            }
            v3::SchemaKind::Type(v3::Type::Object(ref mut o)) => {
                if let Some(properties) = properties {
                    o.properties = properties
                        .into_iter()
                        .map(|(k, v)| (k, v.into()))
                        .collect();
                }
                o.required = required.unwrap_or_default();
            }
            v3::SchemaKind::Type(v3::Type::Array(ref mut a)) => {
                a.items = Some({
                    let item = items.unwrap();
                    let item = *item;
                    let item: v3::ReferenceOr<v3::Schema> = item.into();
                    item.boxed()
                });
            }
            _ => {}
        }

        v3::Schema {
            schema_data,
            schema_kind,
        }
    }
}

impl TryInto<v3::ReferenceOr<v3::Parameter>> for v2::Parameter {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<v3::ReferenceOr<v3::Parameter>, Self::Error> {
        if !self.valid_v3_location() {
            return Err(anyhow::anyhow!("Invalid location: {}", serde_json::to_string(&self.location).unwrap()));
        }
        let v2::Parameter {
            name,
            location,
            description,
            required,
            schema: _,
            type_,
            format,
            items,
            default,
            unique_items,
            collection_format,
        } = self;
        let type_ = type_.unwrap();

        let mut schema_kind = build_schema_kind(&type_, format);
        let mut schema_data = v3::SchemaData::default();

        match &mut schema_kind {
            v3::SchemaKind::Type(v3::Type::Array(ref mut a)) => {
                a.items = items.map(|item| {
                    let item: v3::ReferenceOr<v3::Schema> = item.into();
                    item.boxed()
                });
                a.unique_items = unique_items.unwrap_or_default();
            }
            _ => {}
        }
        schema_data.default = default;

        let mut explode = None;
        if let Some(collection_format) = collection_format {
            match collection_format.as_str() {
                "multi" => explode = Some(true),
                "csv" => explode = Some(false),
                _ => {}
            }
        }

        let parameter_data = v3::ParameterData {
            name,
            description,
            required: required.unwrap_or_default(),
            deprecated: None,
            format: v3::ParameterSchemaOrContent::Schema(v3::ReferenceOr::Item(v3::Schema {
                schema_data,
                schema_kind,
            })),
            example: None,
            examples: Default::default(),
            explode,
            extensions: Default::default(),
        };
        let parameter = match location {
            v2::ParameterLocation::Query => {
                v3::Parameter::Query {
                    parameter_data,
                    allow_reserved: false,
                    style: Default::default(),
                    allow_empty_value: None,
                }
            }
            v2::ParameterLocation::Header => {
                v3::Parameter::Header {
                    parameter_data,
                    style: Default::default(),
                }
            }
            v2::ParameterLocation::Path => {
                v3::Parameter::Path {
                    parameter_data,
                    style: Default::default(),
                }
            }
            v2::ParameterLocation::FormData | v2::ParameterLocation::Body => unreachable!(),
        };
        Ok(v3::ReferenceOr::Item(parameter))
    }
}

fn split_params_into_params_and_body(params: Option<Vec<v2::Parameter>>) -> (Vec<v2::Parameter>, Vec<v2::Parameter>) {
    params
        .unwrap_or_default()
        .into_iter()
        .partition(|p| p.valid_v3_location())
}

impl Into<v3::Operation> for v2::Operation {
    fn into(self) -> v3::Operation {
        let v2::Operation {
            consumes: _,
            produces: _,
            schemes: _,
            tags,
            summary,
            description,
            operation_id,
            parameters,
            mut responses,
            security,
        } = self;
        let (parameters, body) = split_params_into_params_and_body(parameters);
        let body = body.into();

        let responses = {
            let mut r = v3::Responses::default();
            r.default = responses.swap_remove("default").map(|r| r.into());
            r.responses = responses
                .into_iter()
                .map(|(k, v)| (
                    StatusCode::Code(k.parse::<u16>().expect(&format!("Invalid status code: {}", k))),
                    v.into()
                ))
                .collect();
            r
        };
        v3::Operation {
            tags: tags.unwrap_or_default(),
            summary,
            description,
            external_docs: None,
            operation_id,
            parameters: parameters
                .into_iter()
                .flat_map(|p| p.try_into().ok())
                .collect(),
            request_body: Some(v3::ReferenceOr::Item(body)),
            responses,
            deprecated: false,
            security,
            servers: vec![],
            extensions: Default::default(),
        }
    }
}

impl Into<v3::ReferenceOr<v3::Schema>> for v2::ReferenceOrSchema {
    fn into(self) -> v3::ReferenceOr<v3::Schema> {
        match self {
            v2::ReferenceOrSchema::Item(s) => v3::ReferenceOr::Item(s.into()),
            v2::ReferenceOrSchema::Reference { reference } => v3::ReferenceOr::Reference {
                reference: rewrite_ref(&reference)
            }
        }
    }
}

impl Into<v3::RequestBody> for Vec<v2::Parameter> {
    fn into(self) -> v3::RequestBody {
        let mut object = v3::ObjectType::default();
        for param in self {
            let v2::Parameter {
                name,
                location,
                description: _,
                required,
                schema,
                type_: _,
                format: _,
                items: _,
                default: _,
                unique_items: _,
                collection_format: _,
            } = param;
            assert!(location == v2::ParameterLocation::Body);
            if required.unwrap_or_default() {
                object.required.push(name.clone());
            }
            let schema = match schema {
                Some(s) => s.into(),
                None => v3::ReferenceOr::Item(v3::Schema::new_any()),
            };
            object.properties.insert(name, schema);
        }

        let mut content = IndexMap::new();
        content.insert(
            "application/json".to_string(),
            v3::MediaType {
                schema: Some(v3::ReferenceOr::Item(v3::Schema {
                    schema_data: v3::SchemaData::default(),
                    schema_kind: v3::SchemaKind::Type(v3::Type::Object(object)),
                })),
                ..v3::MediaType::default()
            },
        );
        v3::RequestBody {
            description: None,
            content,
            required: true,
            extensions: Default::default(),
        }
    }
}

impl Into<v3::ExternalDocumentation> for v2::ExternalDoc {
    fn into(self) -> v3::ExternalDocumentation {
        let v2::ExternalDoc {
            description,
            url,
        } = self;
        v3::ExternalDocumentation {
            description,
            url,
            ..v3::ExternalDocumentation::default()
        }
    }
}

impl Into<v3::Tag> for v2::Tag {
    fn into(self) -> v3::Tag {
        let v2::Tag {
            name,
            description,
            external_docs,
        } = self;
        v3::Tag {
            name,
            description,
            external_docs: external_docs
                .and_then(|mut e| e.try_remove(0))
                .map(|e| e.into()),
            extensions: Default::default(),
        }
    }
}

impl Into<v3::Info> for v2::Info {
    fn into(self) -> v3::Info {
        let v2::Info {
            title,
            description,
            terms_of_service,
            contact,
            license,
            version,
        } = self;
        v3::Info {
            title: title.unwrap_or_default(),
            description,
            terms_of_service,
            contact: contact.map(|c| c.into()),
            license: license.map(|l| l.into()),
            version: version.unwrap_or_else(|| "0.1.0".to_string()),
            extensions: Default::default(),
        }
    }
}

impl Into<v3::Contact> for v2::Contact {
    fn into(self) -> v3::Contact {
        let v2::Contact {
            name,
            url,
            email,
        } = self;
        v3::Contact {
            name,
            url,
            email,
            extensions: Default::default(),
        }
    }
}

impl Into<v3::License> for v2::License {
    fn into(self) -> v3::License {
        let v2::License {
            name,
            url,
        } = self;
        v3::License {
            name: name.unwrap_or_default(),
            url,
            extensions: Default::default(),
        }
    }
}

impl Into<v3::ReferenceOr<v3::SecurityScheme>> for v2::Security {
    fn into(self) -> v3::ReferenceOr<v3::SecurityScheme> {
        match self {
            v2::Security::ApiKey { name, location, description } => {
                let location = match location {
                    v2::ApiKeyLocation::Query => v3::APIKeyLocation::Query,
                    v2::ApiKeyLocation::Header => v3::APIKeyLocation::Header,
                };
                v3::ReferenceOr::Item(v3::SecurityScheme::APIKey {
                    location,
                    name,
                    description,
                })
            }
            v2::Security::Basic { description } => {
                v3::ReferenceOr::Item(v3::SecurityScheme::HTTP {
                    scheme: "basic".to_string(),
                    bearer_format: None,
                    description,
                })
            }
            v2::Security::Oauth2 { flow, authorization_url, token_url, scopes, description } => {
                let mut implicit = None;
                let mut password = None;
                let mut client_credentials = None;
                let mut authorization_code = None;
                match flow {
                    v2::Flow::AccessCode => {
                        authorization_code = Some(v3::OAuth2Flow::AuthorizationCode {
                            authorization_url,
                            token_url: token_url.unwrap(),
                            refresh_url: None,
                            scopes,
                        });
                    }
                    v2::Flow::Application => {
                        client_credentials = Some(v3::OAuth2Flow::ClientCredentials {
                            token_url: token_url.unwrap(),
                            refresh_url: None,
                            scopes,
                        });
                    }
                    v2::Flow::Implicit => {
                        implicit = Some(v3::OAuth2Flow::Implicit {
                            authorization_url,
                            refresh_url: None,
                            scopes,
                        });
                    }
                    v2::Flow::Password => {
                        password = Some(v3::OAuth2Flow::Password {
                            token_url: token_url.unwrap(),
                            refresh_url: None,
                            scopes,
                        });
                    }
                }
                let flows = v3::OAuth2Flows {
                    implicit,
                    password,
                    client_credentials,
                    authorization_code,
                };
                v3::ReferenceOr::Item(v3::SecurityScheme::OAuth2 {
                    flows,
                    description,
                })
            }
        }
    }
}

impl Into<v3::ReferenceOr<v3::Response>> for v2::Response {
    fn into(self) -> v3::ReferenceOr<v3::Response> {
        let v2::Response {
            description,
            schema,
        } = self;
        let Some(schema) = schema else {
            return v3::ReferenceOr::Item(v3::Response {
                description,
                ..v3::Response::default()
            });
        };
        v3::ReferenceOr::Item(v3::Response {
            description,
            content: {
                let mut map = IndexMap::new();
                map.insert("application/json".to_string(), v3::MediaType {
                    schema: Some(schema.into()),
                    ..v3::MediaType::default()
                });
                map
            },
            ..v3::Response::default()
        })
    }
}