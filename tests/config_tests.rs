use axum::Json;
use serde_json::{json, Value};
use jsonld_signer::handlers::{config::config_handler, sign::sign_handler};
use jsonld_signer::handlers::config::ConfigRequest;

#[tokio::test]
async fn test_sign_without_config_returns_405() {
    // Prepare document
    let doc = json!({
        "id": "urn:ngsi-ld:Store:002",
        "type": "Store",
        "address": { "type": "Property", "value": { "foo": "bar" } }
    });

    // Call sign endpoint without config
    let response = sign_handler(Json(doc.clone())).await;
    match response {
        Err(status) => assert_eq!(status.as_u16(), 405),
        Ok(_) => panic!("Expected 405, got OK"),
    }
}

#[tokio::test]
async fn test_config_all_properties_signs_everything() {
    // Save config with empty properties_to_sign => signs all object fields
    let cfg = ConfigRequest {
        entity_type: "Store".to_string(),
        properties_to_sign: vec![],
    };
    let status = config_handler(Json(cfg)).await;
    assert_eq!(status.as_u16(), 200);

    let doc = json!({
        "id": "urn:ngsi-ld:Store:002",
        "type": "Store",
        "address": { "type": "Property", "value": { "foo": "bar" } },
        "location": { "type": "GeoProperty", "value": { "lat": 1, "lon": 2 } }
    });

    let signed = sign_handler(Json(doc.clone())).await.unwrap().0;
    let address = &signed["address"];
    let location = &signed["location"];
    assert!(address.get("ngsildproof").is_some(), "address not signed");
    assert!(location.get("ngsildproof").is_some(), "location not signed");
}

#[tokio::test]
async fn test_config_selective_signing() {
    // Save config that only signs "address"
    let cfg = ConfigRequest {
        entity_type: "Store".to_string(),
        properties_to_sign: vec!["address".to_string()],
    };
    let status = config_handler(Json(cfg)).await;
    assert_eq!(status.as_u16(), 200);

    let doc = json!({
        "id": "urn:ngsi-ld:Store:002",
        "type": "Store",
        "address": { "type": "Property", "value": { "foo": "bar" } },
        "location": { "type": "GeoProperty", "value": { "lat": 1, "lon": 2 } }
    });

    let signed = sign_handler(Json(doc.clone())).await.unwrap().0;
    let address = &signed["address"];
    let location = &signed["location"];
    assert!(address.get("ngsildproof").is_some(), "address not signed");
    assert!(location.get("ngsildproof").is_none(), "location should not be signed");
}
