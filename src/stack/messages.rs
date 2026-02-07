use chrono::NaiveDateTime;
use tokio::sync::oneshot;
use uuid::Uuid;

use crate::{
    errors::api_errors::ApiErrors,
    fields::text::Text,
    stack::dto::{CreateStackData, UpdatedStackData},
};

#[derive(serde::Serialize)]
pub struct StackResponse {
    pub id: Uuid,
    pub title: Text,
    pub slug: Text,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub enum StackMessage {
    Create {
        stack: CreateStackData,
        respond_to: oneshot::Sender<Result<Uuid, ApiErrors>>,
    },

    GetSingleStack {
        stack_id: Uuid,
        respond_to: oneshot::Sender<Result<StackResponse, ApiErrors>>,
    },

    GetSingleStackByTitle {
        stack_title: String,
        respond_to: oneshot::Sender<Result<StackResponse, ApiErrors>>,
    },

    GetAllStack {
        respond_to: oneshot::Sender<Result<Vec<StackResponse>, ApiErrors>>,
    },

    UpdateStack {
        stack: UpdatedStackData,
        respond_to: oneshot::Sender<Result<bool, ApiErrors>>,
    },

    DeleteStack {
        stack_id: Uuid,
        respond_to: oneshot::Sender<Result<bool, ApiErrors>>,
    },
}
