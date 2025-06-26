use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::RwLock;
use once_cell::sync::Lazy;
use utoipa::ToSchema;
use tracing::{info};

#[derive(Deserialize, ToSchema)]
pub struct ConfigRequest {
    pub entity_type: String,
    pub properties_to_sign: Vec<String>,
}

#[derive(Serialize, Clone)]
pub struct ConfigEntry {
    pub entity_type: String,
    pub properties_to_sign: Vec<String>,
}

// Global config store: entity_type -> ConfigEntry
pub static CONFIG_STORE: Lazy<RwLock<HashMap<String, ConfigEntry>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

#[utoipa::path(
    post,
    path = "/config",
    request_body = ConfigRequest,
    responses((status = 200, description = "Config stored"))
)]
pub async fn config_handler(Json(config): Json<ConfigRequest>) -> StatusCode {
    info!("Calling config_handler method to manage /config endpoint");

    let mut store = CONFIG_STORE.write().unwrap();
    store.insert(
        config.entity_type.clone(),
        ConfigEntry {
            entity_type: config.entity_type,
            properties_to_sign: config.properties_to_sign,
        },
    );
    StatusCode::OK
}
