use tokio::sync::{mpsc::Sender, oneshot};
use tower_cookies::Cookies;
use uuid::Uuid;

use crate::{
    errors::api_errors::ApiErrors,
    refresh_token::messages::{RefreshTokenMessage, TokenPair},
    utils::cookies::set_refresh_cookie,
};

pub async fn login_token_core(
    refresh_token_tx: &Sender<RefreshTokenMessage>,
    cookies: Cookies,
    user_id: Uuid,
) -> Result<TokenPair, ApiErrors> {
    let (tx, rx) = oneshot::channel();

    refresh_token_tx
        .send(RefreshTokenMessage::Login {
            user_id,
            respond_to: tx,
        })
        .await
        .map_err(|_| ApiErrors::InternalServerError("Service unavailable".to_string()))?;

    let tokens = rx
        .await
        .map_err(|_| ApiErrors::InternalServerError("Failed".to_string()))??;

    cookies.add(set_refresh_cookie(tokens.refresh_token.clone()));

    Ok(tokens)
}
