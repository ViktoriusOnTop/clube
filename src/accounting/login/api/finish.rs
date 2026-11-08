use std::sync::Arc;
use axum::{Extension, Json};
use axum::http::StatusCode;
use opaque_ke::{CredentialFinalization, ServerLoginParameters};
use serde::Deserialize;
use crate::accounting::account::DefaultCipherSuite;
use crate::AppState;

#[derive(Deserialize)]
pub struct LoginFinishRequest {
    pub login_id: String,
    pub ke3_bytes: Vec<u8>,
}

pub async fn login_finish(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<LoginFinishRequest>,
) ->  anyhow::Result<Json<String>, (StatusCode, String)>{
    let login_state = state.ongoing_logins.lock().unwrap().remove(&payload.login_id)
        .ok_or((StatusCode::UNAUTHORIZED, "Session expired or invalid transaction ID".to_string()))?;
    let credential_finalization = CredentialFinalization::<DefaultCipherSuite>::deserialize(&payload.ke3_bytes)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid KE3 packet signature structure: {}", e)))?;
    let server_login_finish_result = login_state.finish(
        credential_finalization,
        ServerLoginParameters::default()
    ).map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    let _session_key = server_login_finish_result.session_key;
    Ok(Json("Login successful".to_string()))
}