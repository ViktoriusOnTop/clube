use std::sync::Arc;
use axum::{Extension, Json};
use axum::http::StatusCode;
use opaque_ke::{RegistrationUpload, ServerRegistration};
use serde::Deserialize;
use crate::accounting::account::{AccountCreation, DefaultCipherSuite};
use crate::AppState;

#[derive(Deserialize)]
pub struct SignupFinishRequest {
    pub display_name: String,
    pub username: String,
    pub email: String,
    pub upload_bytes: Vec<u8>,
}

pub async fn signup_finish(
    Extension(state): Extension<Arc<AppState>>,
   Json(request): Json<SignupFinishRequest>,
) -> anyhow::Result<Json<String>, (StatusCode, String)>{
    if let Ok(Some(_)) = state.username_db.find_by_username(&request.username) {
        return Err((StatusCode::CONFLICT, "Username already taken".to_string()));
    }
    if let Ok(Some(_)) = state.email_db.find_by_email(&request.email) {
        return Err((StatusCode::CONFLICT, "Email already registered".to_string()));
    }
    let reg_upload = RegistrationUpload::<DefaultCipherSuite>::deserialize(&request.upload_bytes)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid registration upload bytes: {}", e)))?;

    let server_registration = ServerRegistration::<DefaultCipherSuite>::finish(reg_upload);

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let new_account = AccountCreation {
        last_login_at: None,
        display_name: request.display_name,
        username: request.username.clone(),
        server_registration,
        email: request.email.clone(),
        is_email_verified: false,
        phone_number: None,
        is_phone_verified: None,
        last_active: Some(now),
        recent_failed_login_attempts: 0,
        failed_login_attempts_lockout_time: 0,
        joined_clubs_ids: std::collections::HashSet::new(),
        owned_clubs_ids: std::collections::HashSet::new(),
    };

    let generated_id = state.accounts_db.create_account(new_account)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create account: {}", e)))?;

    state.email_db.insert_emailid(&request.email, generated_id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to index email: {}", e)))?;

    state.username_db.insert_usernameid(&request.username, generated_id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to index username: {}", e)))?;

    Ok(Json("Signup successful".to_string()))
}