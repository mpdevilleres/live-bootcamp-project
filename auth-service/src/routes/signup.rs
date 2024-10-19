use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{app_state::AppState, domain::User};
use crate::domain::AuthAPIError;

pub async fn post_signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = request.email;
    let password = request.password;

    if email.is_empty() || !email.contains("@") {
        return Err(AuthAPIError::InvalidCredentials)
    }

    if password.len() < 8 {
        return Err(AuthAPIError::InvalidCredentials)
    }

    let mut user_store = state.user_store.write().await;
    let user = User::new(email, password, request.requires_2fa);

    if  user_store.get_user(&user.email).is_ok() {
        return Err(AuthAPIError::UserAlreadyExists)
    }


    user_store.add_user(user).unwrap();

    let response = Json(SignupResponse {
        message: "User created successfully!".to_string(),
    });

    Ok((StatusCode::CREATED, response))
}

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct SignupResponse {
    pub message: String,
}
