use base64::{Engine as _, engine::general_purpose};
use tokio::sync::mpsc;

use crate::image::{actor::ImageActor, messages::ImageMessage};

pub async fn image_dispatcher(actor: &ImageActor, mut rx: mpsc::Receiver<ImageMessage>) {
    while let Some(msg) = rx.recv().await {
        match msg {
            ImageMessage::UploadBase64 { base64, respond_to } => {
                let res = actor.upload(base64).await;
                let _ = respond_to.send(res);
            }

            ImageMessage::UploadBytes { bytes, respond_to } => {
                let encoded = general_purpose::STANDARD.encode(&bytes);
                let base64 = format!("data:image/png;base64,{encoded}");

                let res = actor.upload(base64).await;
                let _ = respond_to.send(res);
            }
        }
    }
}
