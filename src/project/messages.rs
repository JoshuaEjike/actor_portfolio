use chrono::NaiveDateTime;
use serde::Serialize;
use tokio::sync::oneshot;
use uuid::Uuid;

use crate::{
    errors::api_errors::ApiErrors,
    fields::text::Text,
    project::dto::{CreateProjectData, UpdatedProjectData},
};

#[derive(Serialize)]
pub struct ProjectResponse {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub stack: Text,
    pub content: String,
    pub word_count: i32,
    pub image: String,
    pub image_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub enum ProjectMessage {
    Create {
        project: CreateProjectData,
        respond_to: oneshot::Sender<Result<Uuid, ApiErrors>>,
    },

    GetSingleProject {
        project_id: Uuid,
        respond_to: oneshot::Sender<Result<ProjectResponse, ApiErrors>>,
    },

    GetAllProject {
        respond_to: oneshot::Sender<Result<Vec<ProjectResponse>, ApiErrors>>,
    },

    UpdateProject {
        project: UpdatedProjectData,
        respond_to: oneshot::Sender<Result<bool, ApiErrors>>,
    },

    DeleteProject {
        project_id: Uuid,
        respond_to: oneshot::Sender<Result<bool, ApiErrors>>,
    },
}
