use tokio::sync::{mpsc::Sender, oneshot};

use crate::{
    errors::api_errors::ApiErrors,
    image::messages::{ImageMessage, ImageUploadResult},
};

pub async fn base64_image_uploader_core(
    base64: String,
    image_tx: &Sender<ImageMessage>,
) -> Result<ImageUploadResult, ApiErrors> {
    let (tx, rx) = oneshot::channel();

    image_tx
        .send(ImageMessage::UploadBase64 {
            base64,
            respond_to: tx,
        })
        .await
        .map_err(|_| ApiErrors::InternalServerError("Image Service unavailable".to_string()))?;

    let result = rx
        .await
        .map_err(|_| ApiErrors::InternalServerError("Image Upload failed".to_string()))??;

    Ok(result)
}

// pub async fn file_image_uploader_core(
//     base64: String,
//     image_tx: &Sender<ImageMessage>,
// ) -> Result<ImageUploadResult, ApiErrors> {
//     let (tx, rx) = oneshot::channel();

//     image_tx
//         .send(ImageMessage::UploadBase64 {
//             base64,
//             respond_to: tx,
//         })
//         .await
//         .map_err(|_| ApiErrors::InternalServerError("Service unavailable".to_string()))?;

//     let result = rx
//         .await
//         .map_err(|_| ApiErrors::InternalServerError("Upload failed".to_string()))??;

//     Ok(result)
// }
