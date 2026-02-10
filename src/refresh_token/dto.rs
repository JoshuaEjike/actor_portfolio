use chrono::NaiveDateTime;
use uuid::Uuid;

pub struct RefreshTokenRecord {
    pub id: Uuid,
    pub user_id: Uuid,
    pub expires_at: NaiveDateTime,
    pub revoked: Option<bool>,
}