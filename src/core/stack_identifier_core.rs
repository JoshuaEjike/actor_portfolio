use tokio::sync::{mpsc::Sender, oneshot};

use crate::{errors::api_errors::ApiErrors, stack::messages::StackMessage};

pub async fn ensure_stack_exists(
    stack_title: String,
    stack_tx: &Sender<StackMessage>,
) -> Result<(), ApiErrors> {
    let (tx, rx) = oneshot::channel();

    stack_tx
        .send(StackMessage::GetSingleStackByTitle {
            stack_title,
            respond_to: tx,
        })
        .await
        .map_err(|_| ApiErrors::InternalServerError("Stack Service unavailable".to_string()))?;

    match rx
        .await
        .map_err(|_| ApiErrors::InternalServerError("Stack service failed".to_string()))?
    {
        Ok(_) => Ok(()),
        Err(ApiErrors::NotFound(_)) => Err(ApiErrors::NotFound("Stack not found".to_string())),
        Err(err) => Err(err),
    }
}
