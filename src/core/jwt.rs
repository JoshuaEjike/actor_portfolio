use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc::Sender, oneshot};
use uuid::Uuid;

use crate::{
    auth::messages::{AuthMessage, UserResponse},
    errors::api_errors::ApiErrors,
};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
}

pub fn generate_token(
    user_id: Uuid,
    jwt_secret: &str,
    jwt_expiry_hour: i64,
) -> Result<String, &'static str> {
    let claims = Claims {
        sub: user_id,
        exp: (chrono::Utc::now() + chrono::Duration::hours(jwt_expiry_hour)).timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .map_err(|_| "Token generation failed")
}

pub fn decode_token(token: &str, jwt_secret: &str) -> Result<Claims, ApiErrors> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &validation,
    )
    .map(|data| data.claims)
    .map_err(|_| ApiErrors::Unauthorized("Invalid or expired token".to_string()))
}

pub async fn validate_user_token(
    token: &str,
    jwt_secret: &str,
    auth_tx: &Sender<AuthMessage>,
) -> Result<UserResponse, ApiErrors> {
    let (tx, rx) = oneshot::channel();

    let claim = decode_token(token, jwt_secret)?;

    auth_tx
        .send(AuthMessage::GetUser {
            user_id: claim.sub,
            respond_to: tx,
        })
        .await
        .map_err(|_| ApiErrors::InternalServerError("Service unavailable".to_string()))?;

    let user = rx
        .await
        .map_err(|_| ApiErrors::InternalServerError("Failed".to_string()))??;

    Ok(user)
}
