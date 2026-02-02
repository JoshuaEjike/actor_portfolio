use axum::{
    Json,
    extract::{Path, State},
};
use tokio::sync::oneshot;
use uuid::Uuid;

use crate::{
    errors::api_errors::ApiErrors,
    extractor::auth_extractor::AuthUser,
    fields::text::Text,
    payload_handler::stack_payload_handler::StackCreateRequest,
    response::general_response::ResponseMessage,
    stack::{
        dto::{CreateStackData, UpdateStackRequest, UpdatedStackData},
        messages::StackMessage,
    },
    state::AppState,
};

pub async fn create_stack(
    AuthUser {
        id,
        email,
        name,
        roles: _,
    }: AuthUser,
    State(state): State<AppState>,
    Json(payload): Json<StackCreateRequest>,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let payload_data = payload.validate()?;

    let (tx, rx) = oneshot::channel();

    let title = Text::new(&payload_data.title)?;

    let slug = Text::new(&payload_data.slug)?;

    let stack = CreateStackData {
        title,
        slug,
        created_by: id,
        created_by_name: name,
        created_by_email: email,
    };

    state
        .stack_tx
        .send(StackMessage::Create {
            stack,
            respond_to: tx,
        })
        .await
        .map_err(|_| ApiErrors::InternalServerError("Stack service unavailable".to_string()))?;

    let stack_id = rx
        .await
        .map_err(|_| ApiErrors::InternalServerError("Stack failed".to_string()))??;

    let response = ResponseMessage {
        message: format!("Stack created: {stack_id}"),
    };

    Ok(Json(serde_json::json!(response)))
}

pub async fn get_single_stack(
    State(state): State<AppState>,
    Path(stack_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let (tx, rx) = oneshot::channel();

    state
        .stack_tx
        .send(StackMessage::GetSingleStack {
            stack_id,
            respond_to: tx,
        })
        .await
        .map_err(|_| ApiErrors::InternalServerError("Service unavailable".to_string()))?;

    let stack = rx
        .await
        .map_err(|_| ApiErrors::InternalServerError("Failed".to_string()))??;

    Ok(Json(serde_json::json!(stack)))
}

pub async fn get_all_stack(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let (tx, rx) = oneshot::channel();

    state
        .stack_tx
        .send(StackMessage::GetAllStack { respond_to: tx })
        .await
        .map_err(|_| ApiErrors::InternalServerError("Service unavailable".to_string()))?;

    let stack = rx
        .await
        .map_err(|_| ApiErrors::InternalServerError("Failed".to_string()))??;

    Ok(Json(serde_json::json!(stack)))
}

pub async fn update_stack(
    AuthUser {
        id,
        email,
        name,
        roles: _,
    }: AuthUser,
    State(state): State<AppState>,
    Path(stack_id): Path<Uuid>,
    // TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    Json(payload): Json<UpdateStackRequest>,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let (tx, rx) = oneshot::channel();

    let title = payload.title.as_deref().map(Text::new).transpose()?;

    let slug = payload.slug.as_deref().map(Text::new).transpose()?;

    let stack = UpdatedStackData {
        stack_id,
        title,
        slug,
        edited_by: id,
        edited_by_name: name,
        edited_by_email: email,
    };

    state
        .stack_tx
        .send(StackMessage::UpdateStack {
            stack,
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

pub async fn delete_stack(
    State(state): State<AppState>,
    Path(stack_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let (tx, rx) = oneshot::channel();

    state
        .stack_tx
        .send(StackMessage::DeleteStack {
            stack_id,
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
