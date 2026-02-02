use serde::Deserialize;
use uuid::Uuid;

use crate::fields::{email::Email, text::Text};

pub struct CreateStackData {
    pub title: Text,
    pub slug: Text,
    pub created_by: Uuid,
    pub created_by_name: Text,
    pub created_by_email: Email,
}

pub struct ValidatedCreateStackData {
    pub title: String,
    pub slug: String,
}

pub struct UpdatedStackData {
    pub stack_id: Uuid,
    pub slug: Option<Text>,
    pub edited_by: Uuid,
    pub edited_by_name: Text,
    pub edited_by_email: Email,
}

#[derive(Deserialize)]
pub struct UpdateStackRequest {
    pub slug: Option<String>,
}
