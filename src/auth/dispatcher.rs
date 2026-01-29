use tokio::sync::mpsc;

use crate::auth::{actor::AuthActor, messages::AuthMessage};

pub async fn auth_dispatcher(actor: &AuthActor, mut rx: mpsc::Receiver<AuthMessage>) {
    while let Some(msg) = rx.recv().await {
        match msg {
            AuthMessage::Register { user, respond_to } => {
                let _ = respond_to.send(actor.register(user).await);
            }
            AuthMessage::Login {
                email,
                password,
                respond_to,
            } => {
                let _ = respond_to.send(actor.login(email, password).await);
            }
            AuthMessage::GetUser {
                user_id,
                respond_to,
            } => {
                let _ = respond_to.send(actor.get_user(user_id).await);
            }
            AuthMessage::GetAllUsers { respond_to } => {
                let _ = respond_to.send(actor.get_all_users().await);
            }
            AuthMessage::UpdateUser { user, respond_to } => {
                let _ = respond_to.send(actor.update_user(user).await);
            }
            AuthMessage::DeleteUser {
                user_id,
                respond_to,
            } => {
                let _ = respond_to.send(actor.delete_user(user_id).await);
            }
        }
    }
}
