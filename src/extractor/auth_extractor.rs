use axum::{extract::FromRequestParts, http::request::Parts};

use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};

use uuid::Uuid;

use crate::{
    core::jwt::validate_user_token,
    errors::api_errors::ApiErrors,
    fields::{email::Email, roles::Roles, text::Text},
    state::AppState,
};

pub struct AuthUser {
    pub id: Uuid,
    pub email: Email,
    pub name: Text,
    pub roles: Roles,
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = ApiErrors;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|_| ApiErrors::Unauthorized("Missing token".into()))?;

        let user = validate_user_token(bearer.token(), &state.jwt_secret, &state.auth_tx).await?;

        Ok(AuthUser {
            id: user.id,
            email: user.email,
            name: user.name,
            roles: user.roles,
        })
    }
}
