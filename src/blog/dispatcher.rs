use tokio::sync::mpsc;

use crate::blog::{actor::BlogActor, messages::BlogMessage};

pub async fn blog_dispatcher(actor: &BlogActor, mut rx: mpsc::Receiver<BlogMessage>) {
    while let Some(msg) = rx.recv().await {
        match msg {
            BlogMessage::Create { blog, respond_to } => {
                let _ = respond_to.send(actor.create_blog(blog).await);
            }
            BlogMessage::GetSingleBlog {
                blog_id,
                respond_to,
            } => {
                let _ = respond_to.send(actor.get_single_blog(blog_id).await);
            }

            BlogMessage::GetAllBlog { respond_to } => {
                let _ = respond_to.send(actor.get_all_blog().await);
            }

            BlogMessage::UpdateBlog { blog, respond_to } => {
                let _ = respond_to.send(actor.update_blog(blog).await);
            }

            BlogMessage::DeleteBlog {
                blog_id,
                respond_to,
            } => {
                let _ = respond_to.send(actor.delete_blog(blog_id).await);
            }
        }
    }
}
