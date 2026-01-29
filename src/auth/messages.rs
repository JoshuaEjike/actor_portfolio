use chrono::NaiveDateTime;
use tokio::sync::oneshot;
use uuid::Uuid;

use crate::{
    auth::dto::{RegisteredData, UpdatedData},
    errors::api_errors::ApiErrors,
    fields::{
        email::Email, password::Password, phone_number::PhoneNumber, roles::Roles, text::Text,
    },
};

#[derive(serde::Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: Email,
    pub name: Text,
    pub phone_number: Option<PhoneNumber>,
    pub roles: Roles,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub enum AuthMessage {
    Register {
        user: RegisteredData,
        respond_to: oneshot::Sender<Result<Uuid, ApiErrors>>,
    },
    Login {
        email: Email,
        password: Password,
        respond_to: oneshot::Sender<Result<String, ApiErrors>>,
    },

    GetUser {
        user_id: Uuid,
        respond_to: oneshot::Sender<Result<UserResponse, ApiErrors>>,
    },

    GetAllUsers {
        respond_to: oneshot::Sender<Result<Vec<UserResponse>, ApiErrors>>,
    },

    UpdateUser {
        user: UpdatedData,
        respond_to: oneshot::Sender<Result<bool, ApiErrors>>,
    },

    DeleteUser {
        user_id: Uuid,
        respond_to: oneshot::Sender<Result<bool, ApiErrors>>,
    },
}
