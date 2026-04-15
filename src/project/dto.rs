use serde::Deserialize;
use uuid::Uuid;

use chrono::NaiveDate;

use crate::fields::{email::Email, text::Text};

pub struct CreateProjectData {
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
    pub created_by: Uuid,
    pub created_by_name: Text,
    pub created_by_email: Email,
}

#[derive(Debug)]
pub struct UpdatedProjectData {
    pub project_id: Uuid,
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
    pub edited_by: Uuid,
    pub edited_by_name: Text,
    pub edited_by_email: Email,
}

pub struct ValidatedCreateProjectData {
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
}

#[derive(Deserialize)]
pub struct UpdateProjectRequest {
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

#[derive(Deserialize)]
pub struct ProjectQuery {
    pub title: Option<String>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}
