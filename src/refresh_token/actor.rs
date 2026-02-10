use tokio::sync::mpsc;

use crate::{
    core::jwt::{generate_refresh_token, generate_token},
    errors::api_errors::ApiErrors,
    refresh_token::{
        dispatch::refresh_token_dispatcher,
        messages::{RefreshTokenMessage, TokenPair},
        repo::RefreshTokenRepository,
    },
};

pub struct RefreshTokenActor<R>
where
    R: RefreshTokenRepository + Send + Sync + 'static,
{
    pub repo: R,
    pub jwt_secret: String,
    pub jwt_expiry_hour: i64,
}

impl<R> RefreshTokenActor<R>
where
    R: RefreshTokenRepository + Send + Sync + 'static,
{
    pub fn new(repo: R, jwt_secret: String, jwt_expiry_hour: i64) -> Self {
        Self {
            repo,
            jwt_secret,
            jwt_expiry_hour,
        }
    }

    pub async fn run(self, rx: mpsc::Receiver<RefreshTokenMessage>) {
        refresh_token_dispatcher(&self, rx).await;
    }

    pub async fn handle_login(&self, user_id: uuid::Uuid) -> Result<TokenPair, ApiErrors> {
        let access_token = generate_token(user_id, &self.jwt_secret, self.jwt_expiry_hour)?;

        let refresh_token = generate_refresh_token();

        self.repo
            .store_refresh_token(user_id, &refresh_token)
            .await?;

        Ok(TokenPair {
            access_token,
            refresh_token,
        })
    }

    pub async fn handle_refresh(&self, token: String) -> Result<TokenPair, ApiErrors> {
        let record = self
            .repo
            .find_refresh_token(&token)
            .await?
            .ok_or(ApiErrors::Unauthorized("Invalid refresh token".into()))?;

        if record.revoked.unwrap_or(false) {
            return Err(ApiErrors::Unauthorized("Token revoked".into()));
        }

        if record.expires_at < chrono::Utc::now().naive_utc() {
            return Err(ApiErrors::Unauthorized("Token expired".into()));
        }

        // 🔁 ROTATION
        self.repo.revoke_refresh_token(record.id).await?;

        let new_refresh = generate_refresh_token();

        self.repo
            .store_refresh_token(record.user_id, &new_refresh)
            .await?;

        let access = generate_token(record.user_id, &self.jwt_secret, self.jwt_expiry_hour)?;

        Ok(TokenPair {
            access_token: access,
            refresh_token: new_refresh,
        })
    }

    pub async fn handle_logout(&self, token: String) -> Result<(), ApiErrors> {
        self.repo.revoke_refresh_token_by_value(&token).await?;

        Ok(())
    }
}
