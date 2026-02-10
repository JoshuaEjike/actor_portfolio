use serde::Serialize;
use tokio::sync::oneshot;
use uuid::Uuid;

use crate::errors::api_errors::ApiErrors;

#[derive(Serialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
}

pub enum RefreshTokenMessage {
    Login {
        user_id: Uuid,
        respond_to: oneshot::Sender<Result<TokenPair, ApiErrors>>,
    },

    Refresh {
        refresh_token: String,
        respond_to: oneshot::Sender<Result<TokenPair, ApiErrors>>,
    },

    Logout {
        refresh_token: String,
        respond_to: oneshot::Sender<Result<(), ApiErrors>>,
    },
}
