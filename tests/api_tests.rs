use serde_json::json;
use jsonld_signer::handlers::{sign::sign_handler, verify::verify_handler, verify::VerificationStatus};
use axum::Json;
use serde_json::Value;

#[tokio::test]
async fn test_sign_and_verify_ok() {
    let document = json!({
        "id": "urn:ngsi-ld:Store:002",
        "type": "Store",
        "address": {
            "type": "Property",
            "value": {
                "streetAddress": ["Tiger Street 4", "al"],
                "addressRegion": "Metropolis",
                "addressLocality": "Cat City",
                "postalCode": "42420"
            }
        }
    });

    let keys = vec!["address"];

    let sign_req = json!({
        "document": document.clone(),
        "keysToSign": keys
    });

    // Sign the document
    let signed = sign_handler(Json(serde_json::from_value(sign_req).unwrap())).await;

    let signed_value: Value = signed.0.clone();
    assert!(signed_value["address"]["ngsildproof"].is_object());

    // Now verify
    let verify_req = json!({ "document": signed_value });

    let result = verify_handler(Json(serde_json::from_value(verify_req).unwrap())).await;
    let result_map = result.0.results;

    // Debug output
    println!("Signed document: {}", serde_json::to_string_pretty(&signed_value).unwrap());
    println!("Verification results: {:?}", result_map);

    // Check if address key exists
    if let Some(status) = result_map.get("address") {
        assert!(matches!(status, VerificationStatus::True));
    } else {
        panic!("Address key not found in verification results. Available keys: {:?}", result_map.keys().collect::<Vec<_>>());
    }
}
