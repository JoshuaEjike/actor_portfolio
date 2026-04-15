use axum::{
    Json,
    extract::{FromRequest, Request},
};

use crate::{
    blog::dto::UpdateBlogRequest, core::image_core::base64_image_uploader_core,
    errors::api_errors::ApiErrors, payload_handler::blog_payload_handler::BlogCreateRequest,
    state::AppState,
};

#[derive(Debug)]
pub struct BlogCreateInput {
    pub title: String,
    pub description: String,
    pub content: String,
    pub word_count: i32,
    pub image: String,
    pub image_id: String,
}

impl FromRequest<AppState> for BlogCreateInput {
    type Rejection = ApiErrors;

    async fn from_request(req: Request, state: &AppState) -> Result<Self, Self::Rejection> {
        let Json(payload) = Json::<BlogCreateRequest>::from_request(req, state)
            .await
            .map_err(|_| ApiErrors::BadRequest("Invalid request body".into()))?;

        let payload_data = payload.validate()?;

        let image = base64_image_uploader_core(payload_data.image, &state.image_tx).await?;

        Ok(BlogCreateInput {
            title: payload_data.title,
            description: payload_data.description,
            content: payload_data.content,
            word_count: payload_data.word_count,
            image: image.url,
            image_id: image.public_id,
        })
    }
}

pub struct BlogUpateInput {
    pub title: Option<String>,
    pub description: Option<String>,
    pub content: Option<String>,
    pub word_count: Option<i32>,
    pub image: Option<String>,
    pub image_id: Option<String>,
}

impl FromRequest<AppState> for BlogUpateInput {
    type Rejection = ApiErrors;

    async fn from_request(req: Request, state: &AppState) -> Result<Self, Self::Rejection> {
        let Json(payload) = Json::<UpdateBlogRequest>::from_request(req, state)
            .await
            .map_err(|_| ApiErrors::BadRequest("Invalid request body".into()))?;

        let image_data = if let Some(base64) = payload.image {
            Some(base64_image_uploader_core(base64, &state.image_tx).await?)
        } else {
            None
        };

        let (image, image_id) = match image_data {
            Some(upload) => (Some(upload.url), Some(upload.public_id)),
            None => (None, None),
        };

        Ok(BlogUpateInput {
            title: payload.title,
            description: payload.description,
            content: payload.content,
            word_count: payload.word_count,
            image,
            image_id,
        })
    }
}
