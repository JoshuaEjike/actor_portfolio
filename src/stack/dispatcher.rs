use tokio::sync::mpsc;

use crate::stack::{actor::StackActor, messages::StackMessage};

pub async fn stack_dispatcher(actor: &StackActor, mut rx: mpsc::Receiver<StackMessage>) {
    while let Some(msg) = rx.recv().await {
        match msg {
            StackMessage::Create { stack, respond_to } => {
                let _ = respond_to.send(actor.create_stack(stack).await);
            }
            StackMessage::GetSingleStack {
                stack_id,
                respond_to,
            } => {
                let _ = respond_to.send(actor.get_single_stack(stack_id).await);
            }

            StackMessage::GetAllStack { respond_to } => {
                let _ = respond_to.send(actor.get_all_stack().await);
            }

            StackMessage::UpdateStack { stack, respond_to } => {
                let _ = respond_to.send(actor.update_stack(stack).await);
            }

            StackMessage::DeleteStack {
                stack_id,
                respond_to,
            } => {
                let _ = respond_to.send(actor.delete_stack(stack_id).await);
            }
        }
    }
}
