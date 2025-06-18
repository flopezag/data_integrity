use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use rand::rngs::OsRng;
use rand::RngCore;
use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, SecretKey};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use chrono::Utc;

#[derive(Serialize, Deserialize, Debug)]
struct Proof {
    #[serde(rename = "type")]
    type_field: String,
    created: String,
    #[serde(rename = "proofPurpose")]
    proof_purpose: String,
    #[serde(rename = "verificationMethod")]
    verification_method: String,
    #[serde(rename = "signatureValue")]
    signature_value: String,
}

pub async fn sign_handler(Json(_ngsilddoc): Json<Value>) -> Json<Value> {
    let mut ngsilddoc = json!({
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
        },
        "@context": "https://uri.etsi.org/ngsi-ld/primer/store-context.jsonld"
    });

    /*
    "ngsildproof": {
      "type": "Property",
      "entityIdSealed": "urn:ngsi-ld:Car123",
      "entityTypeSealed": "Car", 
      "value": {
        "proof": {…}
      }
    }
    */
    
    let mut secret_bytes = [0u8; 32];
    OsRng.fill_bytes(&mut secret_bytes);
    let secret_key = SecretKey::from(secret_bytes);
    let signing_key = SigningKey::from_bytes(&secret_key);
    let verifying_key: VerifyingKey = signing_key.verifying_key();

    let doc_bytes = serde_json::to_vec(&ngsilddoc).expect("Serialization failed");

    let signature: Signature = signing_key.sign(&doc_bytes);
    let signature_b64 = STANDARD.encode(signature.to_bytes());

    let proof = Proof {
        type_field: "Ed25519Signature2020".to_string(),
        created: Utc::now().to_rfc3339(),
        proof_purpose: "assertionMethod".to_string(),
        verification_method: "did:example:123#key-1".to_string(),
        signature_value: signature_b64,
    };

    if let Some(obj) = ngsilddoc.as_object_mut() {
        obj.insert("proof".to_string(), serde_json::to_value(proof).unwrap());
    }

    println!("{}", serde_json::to_string_pretty(&ngsilddoc).unwrap());
    println!("\nPublic Key (base64): {}", STANDARD.encode(verifying_key.to_bytes()));

    Json(ngsilddoc)
}
