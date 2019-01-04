use openapiv3::OpenAPI;
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
