use serde::Deserialize;

use crate::{blog::dto::ValidatedCreateBlogData, errors::api_errors::ApiErrors};

#[derive(Deserialize, Debug)]
pub struct BlogCreateRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub content: Option<String>,
    pub word_count: Option<i32>,
    pub image: Option<String>,
}

impl BlogCreateRequest {
    pub fn validate(self) -> Result<ValidatedCreateBlogData, ApiErrors> {
        println!("{self:?}");

        let title = self
            .title
            .ok_or_else(|| ApiErrors::BadRequest("Title is required".to_string()))?;

        let description = self
            .description
            .ok_or_else(|| ApiErrors::BadRequest("Description is required".to_string()))?;

        let content = self
            .content
            .ok_or_else(|| ApiErrors::BadRequest("Content is required".to_string()))?;

        let word_count = self
            .word_count
            .ok_or_else(|| ApiErrors::BadRequest("Word Count is required".to_string()))?;

        let image = self
            .image
            .ok_or_else(|| ApiErrors::BadRequest("Image is required".to_string()))?;

        Ok(ValidatedCreateBlogData {
            title,
            description,
            content,
            word_count,
            image,
        })
    }
}
