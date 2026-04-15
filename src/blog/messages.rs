use chrono::NaiveDateTime;
use sqlx::prelude::FromRow;
use tokio::sync::oneshot;
use uuid::Uuid;

use crate::{
    blog::dto::{BlogQuery, CreateBlogData, UpdatedBlogData},
    errors::api_errors::ApiErrors,
};

#[derive(Debug, serde::Serialize, FromRow)]
pub struct BlogResponse {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub content: String,
    pub word_count: i32,
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

    GetTotalBlogCount {
        respond_to: oneshot::Sender<Result<u64, ApiErrors>>,
    },

    GetAllBlog {
        query: BlogQuery,
        respond_to: oneshot::Sender<Result<(Vec<BlogResponse>, u64), ApiErrors>>,
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
