use tokio::sync::mpsc::Sender;

use crate::{
    auth::messages::AuthMessage, blog::messages::BlogMessage, image::messages::ImageMessage,
    project::messages::ProjectMessage, refresh_token::messages::RefreshTokenMessage,
    stack::messages::StackMessage,
};

#[derive(Clone)]
pub struct AppState {
    pub auth_tx: Sender<AuthMessage>,
    pub stack_tx: Sender<StackMessage>,
    pub image_tx: Sender<ImageMessage>,
    pub blog_tx: Sender<BlogMessage>,
    pub project_tx: Sender<ProjectMessage>,
    pub refresh_token_tx: Sender<RefreshTokenMessage>,
    pub jwt_secret: String,
}
