use axum::{
    Json,
    extract::{Path, State},
};
use tokio::sync::oneshot;
use uuid::Uuid;

use crate::{
    blog::{
        dto::{CreateBlogData, UpdatedBlogData},
        messages::BlogMessage,
    },
    errors::api_errors::ApiErrors,
    extractor::{
        auth_extractor::AuthUser,
        image_extractor::{BlogCreateInput, BlogUpateInput},
    },
    fields::text::Text,
    response::general_response::ResponseMessage,
    state::AppState,
};

pub async fn create_blog(
    AuthUser {
        id,
        email,
        name,
        roles: _,
    }: AuthUser,
    State(state): State<AppState>,
    payload: BlogCreateInput,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let (tx, rx) = oneshot::channel();

    let title = Text::new(&payload.title)?;

    let description = Text::new(&payload.description)?;

    let blog = CreateBlogData {
        title,
        description,
        content: payload.content,
        image: payload.image,
        image_id: payload.image_id,
        created_by: id,
        created_by_name: name,
        created_by_email: email,
    };

    state
        .blog_tx
        .send(BlogMessage::Create {
            blog,
            respond_to: tx,
        })
        .await
        .map_err(|_| ApiErrors::InternalServerError("Stack service unavailable".to_string()))?;

    let blog_id = rx
        .await
        .map_err(|_| ApiErrors::InternalServerError("Stack failed".to_string()))??;

    let response = ResponseMessage {
        message: format!("Blog created: {blog_id}"),
    };

    Ok(Json(serde_json::json!(response)))
}

pub async fn get_single_blog(
    State(state): State<AppState>,
    Path(blog_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let (tx, rx) = oneshot::channel();

    state
        .blog_tx
        .send(BlogMessage::GetSingleBlog {
            blog_id,
            respond_to: tx,
        })
        .await
        .map_err(|_| ApiErrors::InternalServerError("Service unavailable".to_string()))?;

    let blog = rx
        .await
        .map_err(|_| ApiErrors::InternalServerError("Failed".to_string()))??;

    Ok(Json(serde_json::json!(blog)))
}

pub async fn get_all_blog(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let (tx, rx) = oneshot::channel();

    state
        .blog_tx
        .send(BlogMessage::GetAllBlog { respond_to: tx })
        .await
        .map_err(|_| ApiErrors::InternalServerError("Service unavailable".to_string()))?;

    let stack = rx
        .await
        .map_err(|_| ApiErrors::InternalServerError("Failed".to_string()))??;

    Ok(Json(serde_json::json!(stack)))
}

pub async fn delete_blog(
    AuthUser {
        id: _,
        email: _,
        name: _,
        roles: _,
    }: AuthUser,
    State(state): State<AppState>,
    Path(blog_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let (tx, rx) = oneshot::channel();

    state
        .blog_tx
        .send(BlogMessage::DeleteBlog {
            blog_id,
            respond_to: tx,
        })
        .await
        .map_err(|_| ApiErrors::InternalServerError("Service unavailable".to_string()))?;

    rx.await
        .map_err(|_| ApiErrors::InternalServerError("Failed".to_string()))??;

    let response = ResponseMessage {
        message: "success".to_string(),
    };

    Ok(Json(serde_json::json!(response)))
}

pub async fn update_blog(
    AuthUser {
        id,
        email,
        name,
        roles: _,
    }: AuthUser,
    State(state): State<AppState>,
    Path(blog_id): Path<Uuid>,
    // TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    payload: BlogUpateInput,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let (tx, rx) = oneshot::channel();

    let blog = UpdatedBlogData {
        blog_id,
        description: payload.description,
        content: payload.content,
        image: payload.image,
        image_id: payload.image_id,
        edited_by: id,
        edited_by_name: name,
        edited_by_email: email,
    };

    state
        .blog_tx
        .send(BlogMessage::UpdateBlog {
            blog,
            respond_to: tx,
        })
        .await
        .map_err(|_| ApiErrors::InternalServerError("Service unavailable".to_string()))?;

    rx.await
        .map_err(|_| ApiErrors::InternalServerError("Failed".to_string()))??;

    let response = ResponseMessage {
        message: "success".to_string(),
    };

    Ok(Json(serde_json::json!(response)))
}
