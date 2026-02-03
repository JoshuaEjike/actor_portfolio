use serde::Serialize;
use tokio::sync::oneshot;

use crate::errors::api_errors::ApiErrors;

#[derive(Debug, Serialize)]
pub struct ImageUploadResult {
    pub url: String,
    pub public_id: String,
}

pub enum ImageMessage {
    UploadBase64 {
        base64: String,
        respond_to: oneshot::Sender<Result<ImageUploadResult, ApiErrors>>,
    },
    UploadBytes {
        bytes: Vec<u8>,
        respond_to: oneshot::Sender<Result<ImageUploadResult, ApiErrors>>,
    },
}
