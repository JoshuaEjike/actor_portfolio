use tokio::sync::mpsc::Sender;

use crate::{
    auth::messages::AuthMessage, image::messages::ImageMessage, stack::messages::StackMessage,
};

#[derive(Clone)]
pub struct AppState {
    pub auth_tx: Sender<AuthMessage>,
    pub stack_tx: Sender<StackMessage>,
    pub image_tx: Sender<ImageMessage>,
    pub jwt_secret: String,
}
