use serde::Deserialize;

use crate::{errors::api_errors::ApiErrors, project::dto::ValidatedCreateProjectData};

#[derive(Deserialize)]
pub struct ProjectCreateRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub stack: Option<String>,
    pub content: Option<String>,
    pub image: Option<String>,
}

impl ProjectCreateRequest {
    pub fn validate(self) -> Result<ValidatedCreateProjectData, ApiErrors> {
        let title = self
            .title
            .ok_or_else(|| ApiErrors::BadRequest("Title is required".to_string()))?;

        let description = self
            .description
            .ok_or_else(|| ApiErrors::BadRequest("Description is required".to_string()))?;

        let stack = self
            .stack
            .ok_or_else(|| ApiErrors::BadRequest("Stack is required".to_string()))?;

        let content = self
            .content
            .ok_or_else(|| ApiErrors::BadRequest("Content is required".to_string()))?;

        let image = self
            .image
            .ok_or_else(|| ApiErrors::BadRequest("Image is required".to_string()))?;

        Ok(ValidatedCreateProjectData {
            title,
            description,
            stack,
            content,
            image,
        })
    }
}
