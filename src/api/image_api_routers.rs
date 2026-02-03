use axum::{Router, routing::post};

use crate::{
    image::handlers::{upload_base64, upload_form},
    state::AppState,
};

pub fn image_api_router(state: AppState) -> Router {
    Router::new()
        .route("/base64", post(upload_base64))
        .route("/file", post(upload_form))
        .with_state(state)
}
