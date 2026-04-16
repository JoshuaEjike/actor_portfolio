use chrono::NaiveDate;
use serde::Deserialize;
use url::Url;

use crate::{errors::api_errors::ApiErrors, project::dto::ValidatedCreateProjectData};

#[derive(Deserialize)]
pub struct ProjectCreateRequest {
    pub title: Option<String>,
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
}

impl ProjectCreateRequest {
    pub fn validate(self) -> Result<ValidatedCreateProjectData, ApiErrors> {
        let title = self
            .title
            .ok_or_else(|| ApiErrors::BadRequest("Title is required".to_string()))?;

        let description = self
            .description
            .ok_or_else(|| ApiErrors::BadRequest("Description is required".to_string()))?;

        let company = self
            .company
            .ok_or_else(|| ApiErrors::BadRequest("Company is required".to_string()))?;

        let role = self
            .role
            .ok_or_else(|| ApiErrors::BadRequest("Role is required".to_string()))?;

        let start_date = self
            .start_date
            .ok_or_else(|| ApiErrors::BadRequest("Start date is required".to_string()))?;

        let tag = self
            .tag
            .ok_or_else(|| ApiErrors::BadRequest("Tag is required".to_string()))?;

        let link = self
            .link
            .ok_or_else(|| ApiErrors::BadRequest("Link is required".to_string()))?;

        let stack = self
            .stack
            .ok_or_else(|| ApiErrors::BadRequest("Stack is required".to_string()))?;

        let content = self
            .content
            .ok_or_else(|| ApiErrors::BadRequest("Content is required".to_string()))?;

        let word_count = self
            .word_count
            .ok_or_else(|| ApiErrors::BadRequest("Word Count is required".to_string()))?;

        let image = self
            .image
            .ok_or_else(|| ApiErrors::BadRequest("Image is required".to_string()))?;

        Url::parse(&link).map_err(|_| {
            ApiErrors::BadRequest("Invalid link format. Must be a valid URL".to_string())
        })?;

        if let Some(end_date) = self.end_date
            && end_date < start_date
        {
            return Err(ApiErrors::BadRequest(
                "End date cannot be before start date".to_string(),
            ));
        }

        Ok(ValidatedCreateProjectData {
            title,
            description,
            company,
            role,
            tag,
            link,
            start_date,
            end_date: self.end_date,
            stack,
            content,
            word_count,
            image,
        })
    }
}
