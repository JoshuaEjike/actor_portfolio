use async_trait::async_trait;
use uuid::Uuid;

use crate::{errors::api_errors::ApiErrors, refresh_token::dto::RefreshTokenRecord};

#[async_trait]
pub trait RefreshTokenRepository: Send + Sync {
    async fn store_refresh_token(&self, user_id: Uuid, token: &str) -> Result<(), ApiErrors>;

    async fn find_refresh_token(
        &self,
        token: &str,
    ) -> Result<Option<RefreshTokenRecord>, ApiErrors>;

    async fn revoke_refresh_token(&self, id: Uuid) -> Result<(), ApiErrors>;

    async fn revoke_refresh_token_by_value(&self, token: &str) -> Result<(), ApiErrors>;
}
