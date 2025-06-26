use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::{Value};
use ed25519_dalek::{VerifyingKey, Signature, Verifier};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use std::collections::HashMap;
use utoipa::ToSchema;
use tracing::{info};

#[derive(Deserialize, ToSchema)]
pub struct VerifyRequest {
    pub document: Value,
}

#[derive(Serialize, ToSchema, Debug)]
#[serde(rename_all = "lowercase")]
pub enum VerificationStatus {
    True,
    False,
    NA,
}

#[derive(Serialize, ToSchema)]
pub struct VerifyResult {
    pub results: HashMap<String, VerificationStatus>,
}

#[utoipa::path(
    post,
    path = "/verify",
    request_body = VerifyRequest,
    responses((status = 200, body = VerifyResult))
)]
pub async fn verify_handler(Json(payload): Json<VerifyRequest>) -> Json<VerifyResult> {
    info!("Calling verify_handler method to manage /verify endpoint");

    let mut results = HashMap::new();

    const PUBLIC_KEY_B64: &str = "REPLACE_WITH_YOUR_PUBLIC_KEY_BASE64";
    
    let public_key_bytes = match STANDARD.decode(PUBLIC_KEY_B64) {
        Ok(bytes) => bytes,
        Err(_) => return Json(VerifyResult { results }),
    };

    let public_key_array: [u8; 32] = match public_key_bytes.try_into() {
        Ok(arr) => arr,
        Err(_) => return Json(VerifyResult { results }),
    };
    
    let verifying_key = match VerifyingKey::from_bytes(&public_key_array) {
        Ok(k) => k,
        Err(_) => return Json(VerifyResult { results }),
    };

    let obj = match payload.document.as_object() {
        Some(obj) => obj,
        None => return Json(VerifyResult { results }),
    };

    for (key, value) in obj {
        if !value.is_object() {
            continue;
        }

        let status = verify_field(value, &verifying_key);
        results.insert(key.clone(), status);
    }

    Json(VerifyResult { results })
}

fn verify_field(value: &Value, verifying_key: &VerifyingKey) -> VerificationStatus {
    let field_obj = match value.as_object() {
        Some(obj) => obj,
        None => return VerificationStatus::NA,
    };

    let proof_obj = match field_obj.get("ngsildproof") {
        Some(Value::Object(p)) => p,
        _ => return VerificationStatus::NA,
    };

    let proof_value_b64 = match proof_obj.get("proof")
        .and_then(|p| p.get("proofValue"))
        .and_then(Value::as_str) {
            Some(val) => val,
            None => return VerificationStatus::NA,
    };

    let signature_bytes = match STANDARD.decode(proof_value_b64) {
        Ok(bytes) => bytes,
        Err(_) => return VerificationStatus::False,
    };

    // Convert Vec<u8> to [u8; 64] for signature
    let signature_array: [u8; 64] = match signature_bytes.try_into() {
        Ok(arr) => arr,
        Err(_) => return VerificationStatus::False,
    };

    let signature = Signature::from_bytes(&signature_array);

    let signed_bytes = match serde_json::to_vec(&field_obj_without_proof(field_obj)) {
        Ok(data) => data,
        Err(_) => return VerificationStatus::False,
    };

    match verifying_key.verify(&signed_bytes, &signature) {
        Ok(_) => VerificationStatus::True,
        Err(_) => VerificationStatus::False,
    }
}

// Helper to remove ngsildproof before signing
fn field_obj_without_proof(field: &serde_json::Map<String, Value>) -> Value {
    let mut cleaned = field.clone();
    cleaned.remove("ngsildproof");
    Value::Object(cleaned)
}
