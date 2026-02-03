use axum::{
    Json,
    extract::{Multipart, State},
};
use tokio::sync::oneshot;

use crate::{
    errors::api_errors::ApiErrors,
    image::{dto::Base64Upload, messages::ImageMessage},
    state::AppState,
};

pub async fn upload_base64(
    State(state): State<AppState>,
    Json(payload): Json<Base64Upload>,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let (tx, rx) = oneshot::channel();

    state
        .image_tx
        .send(ImageMessage::UploadBase64 {
            base64: payload.image,
            respond_to: tx,
        })
        .await
        .map_err(|_| ApiErrors::InternalServerError("Service unavailable".to_string()))?;

    let result = rx
        .await
        .map_err(|_| ApiErrors::InternalServerError("Upload failed".to_string()))??;

    Ok(Json(serde_json::json!(result)))
}

pub async fn upload_form(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let field = multipart
        .next_field()
        .await
        .map_err(|_| ApiErrors::InternalServerError("Invalid form".to_string()))?
        .ok_or_else(|| ApiErrors::BadRequest("No file provided".to_string()))?;

    println!("this worked");

    let bytes = field
        .bytes()
        .await
        .map_err(|_| ApiErrors::InternalServerError("Failed to read file".to_string()))?;

    let (tx, rx) = oneshot::channel();

    state
        .image_tx
        .send(ImageMessage::UploadBytes {
            bytes: bytes.to_vec(),
            respond_to: tx,
        })
        .await
        .map_err(|_| ApiErrors::InternalServerError("Service unavailable".to_string()))?;

    let result = rx
        .await
        .map_err(|_| ApiErrors::InternalServerError("Upload failed".to_string()))??;

    Ok(Json(serde_json::json!(result)))
}
