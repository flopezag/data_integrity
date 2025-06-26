use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use rand::rngs::OsRng;
use rand::RngCore;
use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, SecretKey};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use chrono::Utc;
//use utoipa::ToSchema;
use crate::handlers::config::CONFIG_STORE;
use tracing::{info, error};


/*#[derive(Deserialize, ToSchema)]
pub struct SignRequest {
    pub document: Value,
    #[serde(rename = "keysToSign")]
    pub keys_to_sign: Vec<String>,
}
*/
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

#[utoipa::path(
    post,
    path = "/sign",
    request_body = Value,
    responses((status = 200, body = Value), (status = 405, description = "No configuration found"))
)]
pub async fn sign_handler(Json(mut doc): Json<Value>) -> impl IntoResponse {
    info!("Calling sign_handler method to manage /sign endpoint");

    // Check if "data" is present and is an array
    let data_array = match doc.get_mut("data").and_then(Value::as_array_mut) {
        Some(arr) if !arr.is_empty() => arr,
            _ => {
                error!("'data' field must be a non-empty array of entities to sign");

                let response = Json(serde_json::json!({
                    "error": "'data' field must be a non-empty array of entities to sign"
                }));
        
            return Err((StatusCode::BAD_REQUEST, response).into_response());
        }
    };

    info!("Signing {} entities", data_array.len());

    for entity in data_array.iter_mut() {
        let entity_id = entity.get("id").and_then(Value::as_str).unwrap_or_default().to_string();
        let entity_type = entity.get("type").and_then(Value::as_str).unwrap_or_default().to_string();

        info!("Reading config store...");

        let config = {
            let store = CONFIG_STORE.read().unwrap();
            store.get(&entity_type).cloned()
        };

        info!("Got config: {:?}", config.as_ref().map(|c| &c.properties_to_sign));


        let keys_to_sign: Vec<String> = match config {
            None => {
                error!("No signing configuration found for entity type '{}'. \
                    You must POST to /config first to set the configuration.",
                    entity_type);

                let msg = format!(
                    "No signing configuration found for entity type '{}'. \
                    You must POST to /config first to set the configuration.",
                    entity_type
                );

                let response = Json(serde_json::json!({ "error": msg }));
                return Err((StatusCode::PRECONDITION_REQUIRED, response).into_response());
            }

            Some(cfg) if cfg.properties_to_sign.is_empty() => {
                // Sign all top-level object fields
                let obj = match entity.as_object() {
                    Some(o) => o,
                    None => {
                        error!("Each item in 'data' must be a JSON object.");

                        let response = Json(serde_json::json!({
                            "error": "Each item in 'data' must be a JSON object."
                    }));
        
                        return Err((StatusCode::BAD_REQUEST, response).into_response());
                    }
                };

                obj.iter()
                    .filter(|(_, v)| v.is_object())
                    .map(|(k, _)| k.clone())
                    .collect()
            }
            Some(cfg) => cfg.properties_to_sign.clone(),
        };

        info!("Signing {} properties for entity type '{}'", keys_to_sign.len(), entity_type);

        // Generate ephemeral signing key (temporary)
        let mut secret_bytes = [0u8; 32];
        OsRng.fill_bytes(&mut secret_bytes);
        let secret_key = SecretKey::from(secret_bytes);
        let signing_key = SigningKey::from_bytes(&secret_key);
        let _verifying_key: VerifyingKey = signing_key.verifying_key();

        for key in keys_to_sign {
            if let Some(parent) = entity.as_object_mut() {
                if let Some(target) = parent.get(&key).and_then(Value::as_object) {
                    let to_sign = serde_json::to_vec(target).unwrap();
                    let signature = signing_key.sign(&to_sign);
                    let proof = build_proof(&entity_id, &entity_type, &signature);

                    if let Some(Value::Object(signed_section)) = parent.get_mut(&key) {
                        signed_section.insert("ngsildproof".into(), proof);
                    }
                }
            }
        }

    }

    Ok(Json(doc))

    /* 
    // Step 0: Extract EntityId, EntityType, and properties to sign
    let entity_id = doc.get("id").and_then(Value::as_str).unwrap_or_default().to_string();
    let entity_type = doc.get("type").and_then(Value::as_str).unwrap_or_default().to_string();

    let config = {
        let store = CONFIG_STORE.read().unwrap();
        store.get(&entity_type).cloned()
    };

    let keys_to_sign: Vec<String> = match config {
        None => return Err(StatusCode::METHOD_NOT_ALLOWED),
        Some(cfg) if cfg.properties_to_sign.is_empty() => {
            // Sign all top-level object fields
            doc.as_object()
                .unwrap()
                .iter()
                .filter(|(_, v)| v.is_object())
                .map(|(k, _)| k.clone())
                .collect()
        }
        Some(cfg) => cfg.properties_to_sign.clone(),
    };

    // Step 1: Generate signing key (temporary per request; should be stored in prod)
    let mut secret_bytes = [0u8; 32];
    OsRng.fill_bytes(&mut secret_bytes);
    let secret_key = SecretKey::from(secret_bytes);
    let signing_key = SigningKey::from_bytes(&secret_key);
    let _verifying_key: VerifyingKey = signing_key.verifying_key();

    // Step 2: Loop through keys to sign
    for key in keys_to_sign {
        if let Some(parent) = doc.as_object_mut() {
            if let Some(target) = parent.get(&key).and_then(Value::as_object) {
                let to_sign = serde_json::to_vec(target).unwrap();
                let signature = signing_key.sign(&to_sign);
                let proof = build_proof(&entity_id, &entity_type, &signature);

                if let Some(Value::Object(signed_section)) = parent.get_mut(&key) {
                    signed_section.insert("ngsildproof".into(), proof);
                }
            }
        }
    }

    Ok(Json(doc))
    */
}

fn build_proof(entity_id: &str, entity_type: &str, signature: &Signature) -> Value {
    let verification_method = "https://example.edu/issuers/565049#key-1";

    let proof = serde_json::json!({
        "type": "Property",
        "entityIdSealed": entity_id,
        "entityTypeSealed": entity_type,
        "proof": {
            "type": "DataIntegrityProof",
            "created": Utc::now().to_rfc3339(),
            "verificationMethod": verification_method,
            "cryptosuite": "eddsa-rdfc-2022",
            "proofPurpose": "assertionMethod",
            "proofValue": STANDARD.encode(signature.to_bytes())
        }
    });

    proof
}
