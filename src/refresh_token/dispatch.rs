use tokio::sync::mpsc;

use crate::refresh_token::{
    actor::RefreshTokenActor, messages::RefreshTokenMessage, repo::RefreshTokenRepository,
};

pub async fn refresh_token_dispatcher<R>(
    actor: &RefreshTokenActor<R>,
    mut rx: mpsc::Receiver<RefreshTokenMessage>,
) where
    R: RefreshTokenRepository + Send + Sync + 'static,
{
    while let Some(msg) = rx.recv().await {
        match msg {
            RefreshTokenMessage::Login {
                user_id,
                respond_to,
            } => {
                let res = actor.handle_login(user_id).await;
                let _ = respond_to.send(res);
            }

            RefreshTokenMessage::Refresh {
                refresh_token,
                respond_to,
            } => {
                let res = actor.handle_refresh(refresh_token).await;
                let _ = respond_to.send(res);
            }

            RefreshTokenMessage::Logout {
                refresh_token,
                respond_to,
            } => {
                let res = actor.handle_logout(refresh_token).await;
                let _ = respond_to.send(res);
            }
        }
    }
}
