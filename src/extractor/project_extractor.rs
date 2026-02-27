use axum::{
    Json,
    extract::{FromRequest, Request},
};

use crate::{
    core::{image_core::base64_image_uploader_core, stack_identifier_core::ensure_stack_exists},
    errors::api_errors::ApiErrors,
    fields::text::Text,
    payload_handler::project_payload_handler::ProjectCreateRequest,
    project::dto::UpdateProjectRequest,
    state::AppState,
};

pub struct ProjectCreateInput {
    pub title: String,
    pub description: String,
    pub stack: String,
    pub content: String,
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
            stack: payload_data.stack,
            content: payload_data.content,
            image: image.url,
            image_id: image.public_id,
        })
    }
}

pub struct ProjectUpateInput {
    pub description: Option<Text>,
    pub stack: Option<Text>,
    pub content: Option<String>,
    pub image: Option<String>,
    pub image_id: Option<String>,
}

impl FromRequest<AppState> for ProjectUpateInput {
    type Rejection = ApiErrors;

    async fn from_request(req: Request, state: &AppState) -> Result<Self, Self::Rejection> {
        let Json(payload) = Json::<UpdateProjectRequest>::from_request(req, state)
            .await
            .map_err(|_| ApiErrors::BadRequest("Invalid request body".into()))?;

        let description = payload.description.as_deref().map(Text::new).transpose()?;

        let stack = payload.stack.as_deref().map(Text::new).transpose()?;

        if let Some(title) = payload.stack {
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
            description,
            stack,
            content: payload.content,
            image,
            image_id,
        })
    }
}
