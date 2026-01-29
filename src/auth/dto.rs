use serde::Deserialize;
use uuid::Uuid;

use crate::fields::{
    email::Email, password::Password, phone_number::PhoneNumber, roles::Roles, text::Text,
};

pub struct RegisteredData {
    pub email: Email,
    pub password: Password,
    pub name: Text,
    pub phone_number: Option<PhoneNumber>,
    pub roles: Roles,
    pub created_by: Uuid,
    pub created_by_name: Text,
    pub created_by_email: Email,
}

pub struct UpdatedData {
    pub user_id: Uuid,
    pub name: Option<Text>,
    pub phone_number: Option<PhoneNumber>,
    pub roles: Option<Roles>,
    pub edited_by: Uuid,
    pub edited_by_name: Text,
    pub edited_by_email: Email,
}

pub struct ValidatedRegister {
    pub email: String,
    pub password: String,
    pub name: String,
    pub phone_number: Option<String>,
    pub roles: String,
}

pub struct ValidatedLogin {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub roles: Option<String>,
}
