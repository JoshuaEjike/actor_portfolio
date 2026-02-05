use serde::Deserialize;

use crate::{blog::dto::ValidatedCreateBlogData, errors::api_errors::ApiErrors};

#[derive(Deserialize)]
pub struct BlogCreateRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub content: Option<String>,
    pub image: Option<String>,
}

impl BlogCreateRequest {
    pub fn validate(self) -> Result<ValidatedCreateBlogData, ApiErrors> {
        let title = self
            .title
            .ok_or_else(|| ApiErrors::BadRequest("Title is required".to_string()))?;

        let description = self
            .description
            .ok_or_else(|| ApiErrors::BadRequest("Description is required".to_string()))?;

        let content = self
            .content
            .ok_or_else(|| ApiErrors::BadRequest("Content is required".to_string()))?;

        let image = self
            .image
            .ok_or_else(|| ApiErrors::BadRequest("Image is required".to_string()))?;

        Ok(ValidatedCreateBlogData {
            title,
            description,
            content,
            image,
        })
    }
}
