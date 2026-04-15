use axum::{
    Json,
    extract::{Query, State},
};
use tokio::sync::oneshot;
use uuid::Uuid;

use crate::{
    errors::api_errors::ApiErrors,
    extractor::{
        auth_extractor::AuthUser,
        path_id_extractor::PathParam,
        project_extractor::{ProjectCreateInput, ProjectUpateInput},
    },
    project::{
        dto::{CreateProjectData, ProjectQuery, UpdatedProjectData},
        messages::ProjectMessage,
    },
    state::AppState,
};

pub async fn create_project(
    AuthUser {
        id, email, name, ..
    }: AuthUser,
    State(state): State<AppState>,
    payload: ProjectCreateInput,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let (tx, rx) = oneshot::channel();

    let project = CreateProjectData {
        title: payload.title,
        description: payload.description,
        company: payload.company,
        role: payload.role,
        start_date: payload.start_date,
        end_date: payload.end_date,
        tag: payload.tag,
        link: payload.link,
        stack: payload.stack,
        content: payload.content,
        word_count: payload.word_count,
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

    let project_id = rx
        .await
        .map_err(|_| ApiErrors::InternalServerError("Project failed".to_string()))??;

    Ok(Json(serde_json::json!({
        "message": "success".to_string(),
        "data": { "id": project_id }
    })))
}

pub async fn get_single_project(
    State(state): State<AppState>,
    PathParam(project_id): PathParam<Uuid>,
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

    let project = rx
        .await
        .map_err(|_| ApiErrors::InternalServerError("Failed".to_string()))??;

    Ok(Json(serde_json::json!({
        "message": "success".to_string(),
        "data": project
    })))
}

pub async fn get_all_project(
    State(state): State<AppState>,
    Query(query): Query<ProjectQuery>,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let (tx, rx) = oneshot::channel();

    state
        .project_tx
        .send(ProjectMessage::GetAllProject {
            query,
            respond_to: tx,
        })
        .await
        .map_err(|_| ApiErrors::InternalServerError("Service unavailable".to_string()))?;

    let projects = rx
        .await
        .map_err(|_| ApiErrors::InternalServerError("Failed".to_string()))??;

    Ok(Json(serde_json::json!({
        "message": "success".to_string(),
        "data": projects
    })))
}


pub async fn get_total_project_count(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let (tx, rx) = oneshot::channel();

    state
        .project_tx
        .send(ProjectMessage::GetTotalProjectCount { respond_to: tx })
        .await
        .map_err(|_| ApiErrors::InternalServerError("Service unavailable".to_string()))?;

    let project = rx
        .await
        .map_err(|_| ApiErrors::InternalServerError("Failed".to_string()))??;

    Ok(Json(serde_json::json!( {
        "message": "success".to_string(),
        "data": {"total": project},
    })))
}

pub async fn delete_project(
    _: AuthUser,
    State(state): State<AppState>,
    PathParam(project_id): PathParam<Uuid>,
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

    Ok(Json(serde_json::json!({"message": "success".to_string(),})))
}

pub async fn update_project(
    AuthUser {
        id, email, name, ..
    }: AuthUser,
    State(state): State<AppState>,
    PathParam(project_id): PathParam<Uuid>,
    payload: ProjectUpateInput,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let (tx, rx) = oneshot::channel();

    let project = UpdatedProjectData {
        project_id,
        description: payload.description,
        company: payload.company,
        role: payload.role,
        start_date: payload.start_date,
        end_date: payload.end_date,
        tag: payload.tag,
        link: payload.link,
        stack: payload.stack,
        content: payload.content,
        word_count: payload.word_count,
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

    Ok(Json(
        serde_json::json!({   "message": "success".to_string(),}),
    ))
}
