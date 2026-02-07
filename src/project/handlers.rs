use axum::{
    Json,
    extract::{Path, State},
};
use tokio::sync::oneshot;
use uuid::Uuid;

use crate::{
    errors::api_errors::ApiErrors,
    extractor::{
        auth_extractor::AuthUser,
        image_stack_extractor::{ProjectCreateInput, ProjectUpateInput},
    },
    fields::text::Text,
    project::{
        dto::{CreateProjectData, UpdatedProjectData},
        messages::ProjectMessage,
    },
    response::general_response::ResponseMessage,
    state::AppState,
};

pub async fn create_project(
    AuthUser {
        id,
        email,
        name,
        roles: _,
    }: AuthUser,
    State(state): State<AppState>,
    payload: ProjectCreateInput,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let (tx, rx) = oneshot::channel();

    let title = Text::new(&payload.title)?;

    let description = Text::new(&payload.description)?;

    let stack = Text::new(&payload.stack)?;

    let project = CreateProjectData {
        title,
        description,
        stack,
        content: payload.content,
        image: payload.image,
        image_id: payload.image_id,
        created_by: id,
        created_by_name: name,
        created_by_email: email,
    };

    state
        .project_tx
        .send(ProjectMessage::Create {
            project,
            respond_to: tx,
        })
        .await
        .map_err(|_| ApiErrors::InternalServerError("Project service unavailable".to_string()))?;

    let blog_id = rx
        .await
        .map_err(|_| ApiErrors::InternalServerError("Project failed".to_string()))??;

    let response = ResponseMessage {
        message: format!("Project created: {blog_id}"),
    };

    Ok(Json(serde_json::json!(response)))
}

pub async fn get_single_project(
    State(state): State<AppState>,
    Path(project_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let (tx, rx) = oneshot::channel();

    state
        .project_tx
        .send(ProjectMessage::GetSingleProject {
            project_id,
            respond_to: tx,
        })
        .await
        .map_err(|_| ApiErrors::InternalServerError("Service unavailable".to_string()))?;

    let blog = rx
        .await
        .map_err(|_| ApiErrors::InternalServerError("Failed".to_string()))??;

    Ok(Json(serde_json::json!(blog)))
}

pub async fn get_all_project(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let (tx, rx) = oneshot::channel();

    state
        .project_tx
        .send(ProjectMessage::GetAllProject { respond_to: tx })
        .await
        .map_err(|_| ApiErrors::InternalServerError("Service unavailable".to_string()))?;

    let stack = rx
        .await
        .map_err(|_| ApiErrors::InternalServerError("Failed".to_string()))??;

    Ok(Json(serde_json::json!(stack)))
}

pub async fn delete_project(
    AuthUser {
        id: _,
        email: _,
        name: _,
        roles: _,
    }: AuthUser,
    State(state): State<AppState>,
    Path(project_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let (tx, rx) = oneshot::channel();

    state
        .project_tx
        .send(ProjectMessage::DeleteProject {
            project_id,
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

pub async fn update_project(
    AuthUser {
        id,
        email,
        name,
        roles: _,
    }: AuthUser,
    State(state): State<AppState>,
    Path(project_id): Path<Uuid>,
    payload: ProjectUpateInput,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let (tx, rx) = oneshot::channel();

    let project = UpdatedProjectData {
        project_id,
        description: payload.description,
        stack: payload.stack,
        content: payload.content,
        image: payload.image,
        image_id: payload.image_id,
        edited_by: id,
        edited_by_name: name,
        edited_by_email: email,
    };

    state
        .project_tx
        .send(ProjectMessage::UpdateProject {
            project,
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
