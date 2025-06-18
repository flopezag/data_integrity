use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, SecretKey};
use rand::rngs::OsRng;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use chrono::Utc;
use axum::{
    routing::{get, post},
    Json, Router,
};
use tokio::net::TcpListener;
use std::{net::SocketAddr, time::Instant};
use once_cell::sync::Lazy;


// Constants
static START_TIME: Lazy<Instant> = Lazy::new(Instant::now);
static GITHUB_REPO: &str = "https://github.com/your-user/your-repo";
static VERSION: &str = "0.1.0";

#[derive(Serialize)]
struct ServiceInfo {
    version: String,
    repository: String,
    uptime_seconds: u64,
}


#[derive(Serialize, Deserialize, Debug)]
struct Proof {
    #[serde(rename = "type")]
    type_field: String,
    created: String,
    proofPurpose: String,
    verificationMethod: String,
    signatureValue: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(service_info))
        .route("/sign", post(sign_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app)
        .await
        .unwrap();
}

async fn service_info() -> Json<ServiceInfo> {
    Json(ServiceInfo {
        version: VERSION.to_string(),
        repository: GITHUB_REPO.to_string(),
        uptime_seconds: START_TIME.elapsed().as_secs(),
    })
}


async fn sign_handler(Json(_ngsilddoc): Json<Value>) -> Json<Value> {
    // Step 1: Create the NGSI-LD document
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

    // Step 2: Generate a valid Ed25519 signing key
    let mut secret_bytes = [0u8; 32];
    OsRng.fill_bytes(&mut secret_bytes);
    let secret_key = SecretKey::from(secret_bytes);
    let signing_key = SigningKey::from_bytes(&secret_key);
    let verifying_key: VerifyingKey = signing_key.verifying_key();

    // Step 3: Serialize document
    let doc_bytes = serde_json::to_vec(&ngsilddoc).expect("Serialization failed");

    // Step 4: Sign the document
    let signature: Signature = signing_key.sign(&doc_bytes);
    let signature_b64 = STANDARD.encode(signature.to_bytes());

    // Step 5: Create proof
    let proof = Proof {
        type_field: "Ed25519Signature2020".to_string(),
        created: Utc::now().to_rfc3339(),
        proofPurpose: "assertionMethod".to_string(),
        verificationMethod: "did:example:123#key-1".to_string(),
        signatureValue: signature_b64,
    };

    // Step 6: Attach proof
    if let Some(obj) = ngsilddoc.as_object_mut() {
        obj.insert("proof".to_string(), serde_json::to_value(proof).unwrap());
    }

    // Step 7: Output
    println!("{}", serde_json::to_string_pretty(&ngsilddoc).unwrap());
    println!("\nPublic Key (base64): {}", STANDARD.encode(verifying_key.to_bytes()));
    
    Json(ngsilddoc)
}
