use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use rand::rngs::OsRng;
use rand::RngCore;
use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, SecretKey};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use chrono::Utc;

#[derive(Deserialize)]
pub struct SignRequest {
    pub document: Value,
    #[serde(rename = "keysToSign")]
    pub keys_to_sign: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct NgsildProof {
    #[serde(rename = "type")]
    type_field: String,
    #[serde(rename = "entityIdSealed")]
    entity_id_sealed: String,
    #[serde(rename = "entityTypeSealed")]
    entity_type_sealed: String,
    proof: ProofContent,
}

#[derive(Serialize, Deserialize, Debug)]
struct ProofContent {
    #[serde(rename = "type")]
    type_field: String,
    created: String,
    #[serde(rename = "verificationMethod")]
    verification_method: String,
    cryptosuite: String,
    #[serde(rename = "proofPurpose")]
    proof_purpose: String,
    #[serde(rename = "proofValue")]
    proof_value: String,
}

pub async fn sign_handler(Json(request): Json<SignRequest>) -> Json<Value> {
    let mut doc = request.document;
    let keys_to_sign = request.keys_to_sign;

    // Step 1: Generate signing key (temporary per request; should be stored in prod)
    let mut secret_bytes = [0u8; 32];
    OsRng.fill_bytes(&mut secret_bytes);
    let secret_key = SecretKey::from(secret_bytes);
    let signing_key = SigningKey::from_bytes(&secret_key);
    let _verifying_key: VerifyingKey = signing_key.verifying_key();

    let entity_id = doc.get("id").and_then(Value::as_str).unwrap_or_default().to_string();
    let entity_type = doc.get("type").and_then(Value::as_str).unwrap_or_default().to_string();
    let verification_method = "https://example.edu/issuers/565049#z6MkwXG2WjeQnN....Hc6SaVWoT";

    // Step 2: Loop through keys to sign
    for key in keys_to_sign {
        // Extract the sub-object to be signed
        if let Some(parent) = doc.as_object_mut() {
            if let Some(target) = parent.get(&key).and_then(Value::as_object) {
                // Serialize the target field (e.g., "address")
                let to_sign = serde_json::to_vec(target).unwrap();
                let signature: Signature = signing_key.sign(&to_sign);
                let signature_b64 = STANDARD.encode(signature.to_bytes());

                let proof = NgsildProof {
                    type_field: "Property".to_string(),
                    entity_id_sealed: entity_id.clone(),
                    entity_type_sealed: entity_type.clone(),
                    proof: ProofContent {
                        type_field: "DataIntegrityProof".to_string(),
                        created: Utc::now().to_rfc3339(),
                        verification_method: verification_method.to_string(),
                        cryptosuite: "eddsa-rdfc-2022".to_string(),
                        proof_purpose: "assertionMethod".to_string(),
                        proof_value: signature_b64,
                    },
                };

                if let Some(Value::Object(signed_section)) = parent.get_mut(&key) {
                    signed_section.insert(
                        "ngsildproof".to_string(), 
                        serde_json::to_value(proof).unwrap()
                    );
                }

            }
        }
    }

    Json(doc)
}
