use serde::Deserialize;
use uuid::Uuid;

use crate::fields::{email::Email, text::Text};

pub struct CreateBlogData {
    pub title: Text,
    pub description: Text,
    pub content: String,
    pub image: String,
    pub image_id: String,
    pub created_by: Uuid,
    pub created_by_name: Text,
    pub created_by_email: Email,
}

#[derive(Debug)]
pub struct UpdatedBlogData {
    pub blog_id: Uuid,
    pub description: Option<Text>,
    pub content: Option<String>,
    pub image: Option<String>,
    pub image_id: Option<String>,
    pub edited_by: Uuid,
    pub edited_by_name: Text,
    pub edited_by_email: Email,
}

pub struct ValidatedCreateBlogData {
    pub title: String,
    pub description: String,
    pub content: String,
    pub image: String,
}

#[derive(Deserialize)]
pub struct UpdateBlogRequest {
    pub description: Option<String>,
    pub content: Option<String>,
    pub image: Option<String>,
}
