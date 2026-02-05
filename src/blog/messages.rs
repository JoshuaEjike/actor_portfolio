use chrono::NaiveDateTime;
use tokio::sync::oneshot;
use uuid::Uuid;

use crate::{
    blog::dto::{CreateBlogData, UpdatedBlogData},
    errors::api_errors::ApiErrors,
    fields::text::Text,
};

#[derive(serde::Serialize)]
pub struct BlogResponse {
    pub id: Uuid,
    pub title: Text,
    pub description: Text,
    pub content: String,
    pub image: String,
    pub image_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub enum BlogMessage {
    Create {
        blog: CreateBlogData,
        respond_to: oneshot::Sender<Result<Uuid, ApiErrors>>,
    },

    GetSingleBlog {
        blog_id: Uuid,
        respond_to: oneshot::Sender<Result<BlogResponse, ApiErrors>>,
    },

    GetAllBlog {
        respond_to: oneshot::Sender<Result<Vec<BlogResponse>, ApiErrors>>,
    },

    UpdateBlog {
        blog: UpdatedBlogData,
        respond_to: oneshot::Sender<Result<bool, ApiErrors>>,
    },

    DeleteBlog {
        blog_id: Uuid,
        respond_to: oneshot::Sender<Result<bool, ApiErrors>>,
    },
}
