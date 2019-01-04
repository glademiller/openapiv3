#![cfg(test)]
use crate::OpenAPI;
use serde_yaml;

enum FileType {
    YAML,
    JSON,
}

static TEST_CASES: &[(FileType, &str, &str)] = &[
    (
        FileType::YAML,
        "quayio.yaml",
        include_str!("../test_data/quayio.yaml"),
    ),
    (
        FileType::JSON,
        "quayio.json",
        include_str!("../test_data/quayio.json"),
    ),
    (
        FileType::YAML,
        "petstore.yaml",
        include_str!("../test_data/petstore.yaml"),
    ),
    (
        FileType::YAML,
        "api-with-examples.yaml",
        include_str!("../test_data/api-with-examples.yaml"),
    ),
    (
        FileType::YAML,
        "link-example.yaml",
        include_str!("../test_data/link-example.yaml"),
    ),
    (
        FileType::YAML,
        "callback-example.yaml",
        include_str!("../test_data/callback-example.yaml"),
    ),
    (
        FileType::YAML,
        "docker.yaml",
        include_str!("../test_data/docker.yaml"),
    ),
    (
        FileType::YAML,
        "forge.yaml",
        include_str!("../test_data/forge.yaml"),
    ),
    (
        FileType::YAML,
        "adobe_aem.yaml",
        include_str!("../test_data/adobe_aem.yaml"),
    ),
    (
        FileType::YAML,
        "azure_advisor.yaml",
        include_str!("../test_data/azure_advisor.yaml"),
    ),
    (
        FileType::JSON,
        "polygon.json",
        include_str!("../test_data/polygon.json"),
    ),
    (
        FileType::JSON,
        "slack.json",
        include_str!("../test_data/slack.json"),
    ),
    (
        FileType::JSON,
        "swagger_generator.json",
        include_str!("../test_data/swagger_generator.json"),
    ),
    (
        FileType::JSON,
        "twilio.json",
        include_str!("../test_data/twilio.json"),
    ),
    (
        FileType::JSON,
        "fitbit.json",
        include_str!("../test_data/fitbit.json"),
    ),
    (
        FileType::JSON,
        "walmart.json",
        include_str!("../test_data/walmart.json"),
    ),
    (
        FileType::JSON,
        "xkcd.json",
        include_str!("../test_data/xkcd.json"),
    ),
    (
        FileType::YAML,
        "authentiq.yaml",
        include_str!("../test_data/authentiq.yaml"),
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
