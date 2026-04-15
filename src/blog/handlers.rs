use axum::{
    Json,
    extract::{Query, State},
};
use tokio::sync::oneshot;
use uuid::Uuid;

use crate::{
    blog::{
        dto::{BlogQuery, CreateBlogData, UpdatedBlogData},
        messages::BlogMessage,
    },
    errors::api_errors::ApiErrors,
    extractor::{
        auth_extractor::AuthUser,
        blog_extractor::{BlogCreateInput, BlogUpateInput},
        path_id_extractor::PathParam,
    },
    state::AppState,
};

pub async fn create_blog(
    AuthUser {
        id, email, name, ..
    }: AuthUser,
    State(state): State<AppState>,
    payload: BlogCreateInput,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let (tx, rx) = oneshot::channel();

    let blog = CreateBlogData {
        title: payload.title,
        description: payload.description,
        content: payload.content,
        word_count: payload.word_count,
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

    Ok(Json(serde_json::json!({
        "message": "success".to_string(),
        "data": { "blog_id": blog_id }
    })))
}

pub async fn get_single_blog(
    State(state): State<AppState>,
    PathParam(blog_id): PathParam<Uuid>,
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

    Ok(Json(serde_json::json!( {
        "message": "success".to_string(),
        "data": blog,
    })))
}

pub async fn get_all_blog(
    State(state): State<AppState>,
    Query(query): Query<BlogQuery>,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let (tx, rx) = oneshot::channel();

    state
        .blog_tx
        .send(BlogMessage::GetAllBlog {
            query,
            respond_to: tx,
        })
        .await
        .map_err(|_| ApiErrors::InternalServerError("Service unavailable".to_string()))?;

    let blogs = rx
        .await
        .map_err(|_| ApiErrors::InternalServerError("Failed".to_string()))??;

    Ok(Json(serde_json::json!( {
        "message": "success".to_string(),
        "data": {"blogs": blogs.0, "total": blogs.1},
    })))
}

pub async fn get_total_blog_count(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let (tx, rx) = oneshot::channel();

    state
        .blog_tx
        .send(BlogMessage::GetTotalBlogCount { respond_to: tx })
        .await
        .map_err(|_| ApiErrors::InternalServerError("Service unavailable".to_string()))?;

    let blogs = rx
        .await
        .map_err(|_| ApiErrors::InternalServerError("Failed".to_string()))??;

    Ok(Json(serde_json::json!( {
        "message": "success".to_string(),
        "data": {"total": blogs},
    })))
}

pub async fn delete_blog(
    _: AuthUser,
    State(state): State<AppState>,
    PathParam(blog_id): PathParam<Uuid>,
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

    Ok(Json(serde_json::json!({"message": "success".to_string(),})))
}

pub async fn update_blog(
    AuthUser {
        id, email, name, ..
    }: AuthUser,
    State(state): State<AppState>,
    PathParam(blog_id): PathParam<Uuid>,
    // TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    payload: BlogUpateInput,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let (tx, rx) = oneshot::channel();

    let blog = UpdatedBlogData {
        blog_id,
        title: payload.title,
        description: payload.description,
        content: payload.content,
        word_count: payload.word_count,
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

    Ok(Json(serde_json::json!({"message": "success".to_string(),})))
}
