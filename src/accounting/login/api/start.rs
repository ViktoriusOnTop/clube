use std::sync::Arc;
use axum::{Extension, Json};
use axum::http::StatusCode;
use opaque_ke::CipherSuite;
use opaque_ke::{CredentialRequest, ServerLogin, ServerLoginParameters};
use opaque_ke::argon2::password_hash::rand_core::OsRng;
use crate::accounting::account::DefaultCipherSuite;
use serde::{Deserialize, Serialize};
use crate::AppState;

#[derive(Deserialize)]
pub struct LoginStartRequest {
    pub identifier: String, //username or email
    pub ke1_bytes: Vec<u8>,
}


#[derive(Serialize)]
pub struct LoginStartResponse {
    pub login_id: String,
    pub ke2_bytes: Vec<u8>,
}

pub fn login_start(
    Extension(state): Extension<Arc<AppState>>,
    Json(request): Json<LoginStartRequest>,
) -> anyhow::Result<Json<LoginStartResponse>, (StatusCode, String)> {
    let mut rng = OsRng;
    let user_id = if let Ok(Some(id)) = state.username_db.find_by_username(&request.identifier) {
        id
    } else if let Ok(Some(id)) = state.email_db.find_by_email(&request.identifier) {
        id
    } else {
        return Err((StatusCode::NOT_FOUND, "User profile not found".to_string()));
    };
    let account = state.accounts_db.find_account(user_id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB read failed: {}", e)))?
        .ok_or((StatusCode::NOT_FOUND, "Account profile data missing".to_string()))?;
    let credential_request = CredentialRequest::<DefaultCipherSuite>::deserialize(&request.ke1_bytes)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid KE1 packet: {}", e)))?;
    let server_login_start_result = ServerLogin::start(
        &mut rng,
        &state.setup,
        Some(account.server_registration),
        credential_request,
        request.identifier.as_bytes(),
        ServerLoginParameters::default(),
    ).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("OPAQUE login init failed: {}", e)))?;
    let login_id = rand::random::<u64>().to_string();
    state.ongoing_logins.lock().unwrap().insert(login_id.clone(), server_login_start_result.state);

    Ok(Json(LoginStartResponse {
        login_id,
        ke2_bytes: server_login_start_result.message.serialize().to_vec(),
    }))
}