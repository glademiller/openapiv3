use indexmap::IndexMap;
use openapiv3::*;
use serde_yaml;

enum FileType {
    YAML,
    JSON,
}

static TEST_CASES: &[(FileType, &str, &str)] = &[
    (
        FileType::YAML,
        "quayio.yaml",
        include_str!("../fixtures/quayio.yaml"),
    ),
    (
        FileType::JSON,
        "quayio.json",
        include_str!("../fixtures/quayio.json"),
    ),
    (
        FileType::YAML,
        "petstore.yaml",
        include_str!("../fixtures/petstore.yaml"),
    ),
    (
        FileType::YAML,
        "petstore-discriminated.yaml",
        include_str!("../fixtures/petstore-discriminated.yaml"),
    ),
    (
        FileType::YAML,
        "api-with-examples.yaml",
        include_str!("../fixtures/api-with-examples.yaml"),
    ),
    (
        FileType::YAML,
        "link-example.yaml",
        include_str!("../fixtures/link-example.yaml"),
    ),
    (
        FileType::YAML,
        "callback-example.yaml",
        include_str!("../fixtures/callback-example.yaml"),
    ),
    (
        FileType::YAML,
        "docker.yaml",
        include_str!("../fixtures/docker.yaml"),
    ),
    (
        FileType::YAML,
        "forge.yaml",
        include_str!("../fixtures/forge.yaml"),
    ),
    (
        FileType::YAML,
        "adobe_aem.yaml",
        include_str!("../fixtures/adobe_aem.yaml"),
    ),
    (
        FileType::YAML,
        "azure_advisor.yaml",
        include_str!("../fixtures/azure_advisor.yaml"),
    ),
    (
        FileType::JSON,
        "polygon.json",
        include_str!("../fixtures/polygon.json"),
    ),
    (
        FileType::JSON,
        "slack.json",
        include_str!("../fixtures/slack.json"),
    ),
    (
        FileType::JSON,
        "swagger_generator.json",
        include_str!("../fixtures/swagger_generator.json"),
    ),
    (
        FileType::JSON,
        "twilio.json",
        include_str!("../fixtures/twilio.json"),
    ),
    (
        FileType::JSON,
        "fitbit.json",
        include_str!("../fixtures/fitbit.json"),
    ),
    (
        FileType::JSON,
        "walmart.json",
        include_str!("../fixtures/walmart.json"),
    ),
    (
        FileType::JSON,
        "xkcd.json",
        include_str!("../fixtures/xkcd.json"),
    ),
    (
        FileType::YAML,
        "authentiq.yaml",
        include_str!("../fixtures/authentiq.yaml"),
    ),
    (
        FileType::YAML,
        "stripe.yaml",
        include_str!("../fixtures/stripe.yaml"),
    ),
];

#[test]
fn run_tests() {
    for (file_type, name, contents) in TEST_CASES {
        println!("{}", name);
        let openapi: OpenAPI = match file_type {
            FileType::YAML => serde_yaml::from_str(contents)
                .expect(&format!("Could not deserialize file {}", name)),
            FileType::JSON => serde_json::from_str(contents)
                .expect(&format!("Could not deserialize file {}", name)),
        };
        let _yaml =
            serde_yaml::to_string(&openapi).expect(&format!("Could not serialize YAML {}", name));
        let _json =
            serde_json::to_string(&openapi).expect(&format!("Could not serialize JSON {}", name));
    }
}

macro_rules! map {
    ( $( $key:expr => $value:expr ),* $(,)? ) => {{
        #[allow(unused_mut)]
        let mut m = IndexMap::new();
        $(m.insert($key, $value);)*
        m
    }};
}

#[test]
fn petstore_discriminated() {
    let api = OpenAPI {
        openapi: "3.0.0".to_owned(),
        info: Info {
            title: "Swagger Petstore".to_owned(),
            license: Some(License {
                name: "MIT".to_owned(),
                url: None,
                ..Default::default()
            }),
            version: "1.0.0".to_owned(),
            extensions: {
                let mut ext = IndexMap::new();
                ext.insert("x-hash".to_string(), serde_json::json!("abc123"));
                ext
            },
            ..Default::default()
        },
        servers: vec![Server {
            url: "http://petstore.swagger.io/v1".to_owned(),
            ..Default::default()
        }],
        components: Some(Components {
            schemas: map! {
                "Cat".to_owned() => ReferenceOr::Item(Schema {
                    schema_data: SchemaData {
                        description: Some("A representation of a cat".to_owned()),
                        ..Default::default()
                    },
                    schema_kind: SchemaKind::AllOf { all_of: vec![
                        ReferenceOr::ref_("#/components/schemas/Pet"),
                        ReferenceOr::Item(Schema {
                            schema_data: Default::default(),
                            schema_kind: SchemaKind::Type(Type::Object(ObjectType {
                                properties: map!{
                                    "huntingSkill".to_owned() => ReferenceOr::boxed_item(Schema {
                                        schema_data: SchemaData {
                                            description: Some("The measured skill for hunting".to_owned()),
                                            ..Default::default()
                                        },
                                        schema_kind: SchemaKind::Type(Type::String(StringType {
                                            enumeration: vec![
                                                "clueless".to_owned(),
                                                "lazy".to_owned(),
                                                "adventurous".to_owned(),
                                                "aggressive".to_owned(),
                                            ],
                                            ..Default::default()
                                        })),
                                    }),
                                },
                                required: vec!["huntingSkill".to_owned()],
                                ..Default::default()
                            })),
                        }),
                    ]},
                }),

                "Dog".to_owned() => ReferenceOr::Item(Schema {
                    schema_data: SchemaData {
                        description: Some("A representation of a dog".to_owned()),
                        ..Default::default()
                    },
                    schema_kind: SchemaKind::AllOf { all_of: vec![
                        ReferenceOr::ref_("#/components/schemas/Pet"),
                        ReferenceOr::Item(Schema {
                            schema_data: Default::default(),
                            schema_kind: SchemaKind::Type(Type::Object(ObjectType {
                                properties: map!{
                                    "packSize".to_owned() => ReferenceOr::boxed_item(Schema {
                                        schema_data: SchemaData {
                                            description: Some("the size of the pack the dog is from".to_owned()),
                                            ..Default::default()
                                        },
                                        schema_kind: SchemaKind::Type(Type::Integer(IntegerType {
                                            format: VariantOrUnknownOrEmpty::Item(IntegerFormat::Int32),
                                            minimum: Some(0),
                                            ..Default::default()
                                        })),
                                    }),
                                },
                                required: vec!["packSize".to_owned()],
                                ..Default::default()
                            })),
                        }),
                    ]},
                }),

                "Pet".to_owned() => ReferenceOr::Item(Schema {
                    schema_data: SchemaData {
                        discriminator: Some(Discriminator {
                            property_name: "petType".to_owned(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    schema_kind: SchemaKind::Type(Type::Object(ObjectType {
                        properties: map!{
                            "name".to_owned() => ReferenceOr::boxed_item(Schema {
                                schema_data: Default::default(),
                                schema_kind: SchemaKind::Type(Type::String(Default::default())),
                            }),
                            "petType".to_owned() => ReferenceOr::boxed_item(Schema {
                                schema_data: Default::default(),
                                schema_kind: SchemaKind::Type(Type::String(Default::default())),
                            }),
                        },
                        required: vec!["name".to_owned(), "petType".to_owned()],
                        ..Default::default()
                    })),
                }),
            },
            ..Default::default()
        }),
        ..Default::default()
    };
    let yaml = include_str!("../fixtures/petstore-discriminated.yaml");
    assert_eq!(serde_yaml::to_string(&api).unwrap(), yaml);
    assert_eq!(api, serde_yaml::from_str(yaml).unwrap());
}
