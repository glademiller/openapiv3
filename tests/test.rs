use openapiv3::*;
use serde_yaml;
use std::collections::BTreeMap;

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
        let mut m = BTreeMap::new();
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
            license: Some(License { name: "MIT".to_owned(), url: None }),
            version: "1.0.0".to_owned(),
            ..Default::default()
        },
        servers: vec![
            Server {
                url: "http://petstore.swagger.io/v1".to_owned(),
                ..Default::default()
            }
        ],
        components: Some(Components {
            schemas: map!{
                "Cat".to_owned() => ReferenceOr::Item(Schema::AllOf{
                    all_of: vec![
                        ReferenceOr::Reference { reference: "#/components/schemas/Pet".to_owned() },
                        ReferenceOr::Item(Schema::Schema(Box::new(SchemaVariant::Object {
                            properties: map!{
                                "huntingSkill".to_owned() => ReferenceOr::Item(Box::new(Schema::Schema(Box::new(SchemaVariant::String {
                                    format: VariantOrUnknownOrEmpty::Empty,
                                    pattern: None,
                                    schema_data: SchemaData {
                                        description: Some("The measured skill for hunting".to_owned()),
                                        ..Default::default()
                                    },
                                    enumeration: vec![
                                        "clueless".to_owned(),
                                        "lazy".to_owned(),
                                        "adventurous".to_owned(),
                                        "aggressive".to_owned(),
                                    ]
                                })))),
                            },
                            required: vec!["huntingSkill".to_owned()],
                            additional_properties: false,
                            min_properties: None,
                            max_properties: None,
                            schema_data: Default::default(),
                        }))),
                    ]
                }),
                "Dog".to_owned() => ReferenceOr::Item(Schema::AllOf{
                    all_of: vec![
                        ReferenceOr::Reference { reference: "#/components/schemas/Pet".to_owned() },
                        ReferenceOr::Item(Schema::Schema(Box::new(SchemaVariant::Object {
                            properties: map!{
                                "packSize".to_owned() => ReferenceOr::Item(Box::new(Schema::Schema(Box::new(SchemaVariant::Integer {
                                    format: VariantOrUnknownOrEmpty::Item(IntegerFormat::Int32),
                                    schema_data: SchemaData {
                                        description: Some("the size of the pack the dog is from".to_owned()),
                                        ..Default::default()
                                    },
                                    multiple_of: None,
                                    exclusive_minimum: false,
                                    exclusive_maximum: false,
                                    minimum: Some(0),
                                    maximum: None,
                                    enumeration: Vec::new(),
                                })))),
                            },
                            required: vec!["packSize".to_owned()],
                            additional_properties: false,
                            min_properties: None,
                            max_properties: None,
                            schema_data: Default::default(),
                        }))),
                    ]
                }),
                "Pet".to_owned() => ReferenceOr::Item(Schema::Schema(Box::new(SchemaVariant::Object {
                    properties: map!{
                        "name".to_owned() => ReferenceOr::Item(Box::new(Schema::Schema(Box::new(SchemaVariant::String {
                            format: VariantOrUnknownOrEmpty::Empty,
                            pattern: None,
                            schema_data: Default::default(),
                            enumeration: Vec::new(),
                        })))),
                        "petType".to_owned() => ReferenceOr::Item(Box::new(Schema::Schema(Box::new(SchemaVariant::String {
                            format: VariantOrUnknownOrEmpty::Empty,
                            pattern: None,
                            schema_data: Default::default(),
                            enumeration: Vec::new(),
                        })))),
                    },
                    required: vec!["name".to_owned(), "petType".to_owned()],
                    additional_properties: false,
                    min_properties: None,
                    max_properties: None,
                    schema_data: SchemaData {
                        discriminator: Some(Discriminator {
                            property_name: "petType".to_owned(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                }))),
            },
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(
        serde_yaml::to_string(&api).unwrap(),
        include_str!("../fixtures/petstore-discriminated.yaml")
    );
}
