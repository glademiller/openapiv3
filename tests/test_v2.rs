#![cfg(feature = "v2")]

const PETSTORE_EXAMPLE: &str = include_str!("../fixtures/petstore-extended-swagger2-0.json");

#[test]
fn load_swagger_20_and_upgrade() {
    let v2: openapiv3::v2::OpenAPI = serde_json::from_str(PETSTORE_EXAMPLE).unwrap();

    let versioned: openapiv3::VersionedOpenAPI = serde_json::from_str(PETSTORE_EXAMPLE).unwrap();
    assert!(matches!(versioned, openapiv3::VersionedOpenAPI::V2(_)));
    let v3: openapiv3::OpenAPI = versioned.upgrade();

    // schemas
    assert!(v3.openapi.starts_with("3.0"));
    assert!(v2.definitions.as_ref().unwrap().contains_key("Pet"));
    assert!(v2.definitions.as_ref().unwrap().contains_key("NewPet"));
    assert!(v2.definitions.as_ref().unwrap().contains_key("Error"));
    assert!(v3.schemas().contains_key("Pet"));
    assert!(v3.schemas().contains_key("NewPet"));
    assert!(v3.schemas().contains_key("Error"));

    // paths
    assert!(v2.paths.contains_key("/pets"));
    assert!(v2.paths.contains_key("/pets/{id}"));

    assert!(v3.paths.paths.contains_key("/pets"));
    assert!(v3.paths.paths.contains_key("/pets/{id}"));

}