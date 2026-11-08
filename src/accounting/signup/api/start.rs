// POST /api/signup/start

use std::sync::Arc;
use axum::{Extension, Json};
use axum::http::StatusCode;
use opaque_ke::{RegistrationRequest, ServerRegistration};
use serde::{Deserialize, Serialize};
use crate::accounting::account::DefaultCipherSuite;
use crate::AppState;

#[derive(Deserialize)]
pub struct SignupStartRequest {
    pub username: String,
    pub request_bytes: Vec<u8>,
}

#[derive(Serialize)]
pub struct SignupStartResponse {
    pub response_bytes: Vec<u8>,
}

pub fn signup_start(
    Extension(state): Extension<Arc<AppState>>,
    Json(request): Json<SignupStartRequest>,
) -> anyhow::Result<Json<SignupStartResponse>, (StatusCode, String)> {
    if let Ok(Some(_)) = state.username_db.find_by_username(&request.username) {
        return Err((StatusCode::CONFLICT, "Username already exists".to_string()));
    }

    let reg_request = RegistrationRequest::<DefaultCipherSuite>::deserialize(&request.request_bytes)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid serialization: {}", e)))?;

    let server_start_result = ServerRegistration::<DefaultCipherSuite>::start(
        &state.setup,
        reg_request,
        request.username.as_bytes(),
    ).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("OPAQUE start failed: {}", e)))?;

    Ok(Json(SignupStartResponse {
        response_bytes: server_start_result.message.serialize().to_vec(),
    }))
}
