use tokio::sync::mpsc::Sender;

use crate::{auth::messages::AuthMessage, stack::messages::StackMessage};

#[derive(Clone)]
pub struct AppState {
    pub auth_tx: Sender<AuthMessage>,
    pub stack_tx: Sender<StackMessage>,
    pub jwt_secret: String,
}
