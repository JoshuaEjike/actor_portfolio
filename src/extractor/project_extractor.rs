use axum::{
    Json,
    extract::{FromRequest, Request},
};

use chrono::NaiveDate;

use crate::{
    core::{image_core::base64_image_uploader_core, stack_identifier_core::ensure_stack_exists},
    errors::api_errors::ApiErrors,
    payload_handler::project_payload_handler::ProjectCreateRequest,
    project::dto::UpdateProjectRequest,
    state::AppState,
};

pub struct ProjectCreateInput {
    pub title: String,
    pub description: String,
    pub company: String,
    pub role: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub tag: String,
    pub link: String,
    pub stack: String,
    pub content: String,
    pub word_count: i32,
    pub image: String,
    pub image_id: String,
}

impl FromRequest<AppState> for ProjectCreateInput {
    type Rejection = ApiErrors;

    async fn from_request(req: Request, state: &AppState) -> Result<Self, Self::Rejection> {
        let Json(payload) = Json::<ProjectCreateRequest>::from_request(req, state)
            .await
            .map_err(|_| ApiErrors::BadRequest("Invalid request body".into()))?;

        let payload_data = payload.validate()?;

        ensure_stack_exists(payload_data.stack.clone(), &state.stack_tx).await?;

        let image = base64_image_uploader_core(payload_data.image, &state.image_tx).await?;

        Ok(ProjectCreateInput {
            title: payload_data.title,
            description: payload_data.description,
            company: payload_data.company,
            role: payload_data.role,
            start_date: payload_data.start_date,
            end_date: payload_data.end_date,
            tag: payload_data.tag,
            link: payload_data.link,
            stack: payload_data.stack,
            content: payload_data.content,
            word_count: payload_data.word_count,
            image: image.url,
            image_id: image.public_id,
        })
    }
}

#[derive(Clone)]
pub struct ProjectUpateInput {
    pub description: Option<String>,
    pub company: Option<String>,
    pub role: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub tag: Option<String>,
    pub link: Option<String>,
    pub stack: Option<String>,
    pub content: Option<String>,
    pub word_count: Option<i32>,
    pub image: Option<String>,
    pub image_id: Option<String>,
}

impl FromRequest<AppState> for ProjectUpateInput {
    type Rejection = ApiErrors;

    async fn from_request(req: Request, state: &AppState) -> Result<Self, Self::Rejection> {
        let Json(payload) = Json::<UpdateProjectRequest>::from_request(req, state)
            .await
            .map_err(|_| ApiErrors::BadRequest("Invalid request body".into()))?;

        if let Some(title) = payload.stack.clone() {
            ensure_stack_exists(title, &state.stack_tx).await?;
        }

        let image_data = if let Some(base64) = payload.image {
            Some(base64_image_uploader_core(base64, &state.image_tx).await?)
        } else {
            None
        };

        let (image, image_id) = match image_data {
            Some(upload) => (Some(upload.url), Some(upload.public_id)),
            None => (None, None),
        };

        Ok(ProjectUpateInput {
            description: payload.description,
            company: payload.company,
            role: payload.role,
            start_date: payload.start_date,
            end_date: payload.end_date,
            tag: payload.tag,
            link: payload.link,
            stack: payload.stack,
            content: payload.content,
            word_count: payload.word_count,
            image,
            image_id,
        })
    }
}
