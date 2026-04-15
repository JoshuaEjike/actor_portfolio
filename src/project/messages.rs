use chrono::{NaiveDate, NaiveDateTime};
use serde::Serialize;
use sqlx::prelude::FromRow;
use tokio::sync::oneshot;
use uuid::Uuid;

use crate::{
    errors::api_errors::ApiErrors,
    project::dto::{CreateProjectData, ProjectQuery, UpdatedProjectData},
};

#[derive(Debug, Serialize, FromRow)]
pub struct ProjectResponse {
    pub id: Uuid,
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
        query: ProjectQuery,
        respond_to: oneshot::Sender<Result<(Vec<ProjectResponse>, u64), ApiErrors>>,
    },

    GetTotalProjectCount {
        respond_to: oneshot::Sender<Result<u64, ApiErrors>>,
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
