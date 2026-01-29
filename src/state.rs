use tokio::sync::mpsc::Sender;

use crate::auth::messages::AuthMessage;

#[derive(Clone)]
pub struct AppState {
    pub auth_tx: Sender<AuthMessage>,
    pub jwt_secret: String,
}
