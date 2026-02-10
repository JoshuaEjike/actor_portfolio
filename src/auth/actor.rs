use crate::{
    auth::{
        dispatcher::auth_dispatcher,
        dto::{LoginResponse, RegisteredData, UpdatedData},
        messages::{AuthMessage, UserResponse},
    },
    core::password_core::{hash_password, verify_password},
    errors::api_errors::ApiErrors,
    fields::{
        email::Email, password::Password, phone_number::PhoneNumber, roles::Roles, text::Text,
    },
};

use sqlx::PgPool;
use tokio::sync::mpsc;
use uuid::Uuid;

pub struct AuthActor {
    pool: PgPool,
}

impl AuthActor {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
        }
    }

    pub async fn run(self, rx: mpsc::Receiver<AuthMessage>) {
        auth_dispatcher(&self, rx).await;
    }

    pub async fn register(&self, user: RegisteredData) -> Result<Uuid, ApiErrors> {
        let hash = hash_password(user.password.as_str())
            .map_err(|e| ApiErrors::PasswordFail(e.to_string()))?;

        let id = Uuid::new_v4();

        let created_at = chrono::Utc::now().naive_utc();

        sqlx::query!(
            "INSERT INTO users (id, email, password, name, phone_number, roles, created_by, created_by_name, created_by_email, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
            id,
            user.email.as_str(),
            hash,
            user.name.as_str(),
            user.phone_number.as_ref().map(|p| p.as_str()),
            user.roles.as_str(),
            user.created_by,
            user.created_by_name.as_str(),
            user.created_by_email.as_str(),
            created_at,
            created_at,
        )
        .execute(&self.pool)
        .await
        .map_err(|_| ApiErrors::Conflict("Email already exists".to_string()))?;

        Ok(id)
    }

    pub async fn login(
        &self,
        email: Email,
        password: Password,
    ) -> Result<LoginResponse, ApiErrors> {
        let record = sqlx::query!(
            "SELECT id, password FROM users WHERE email = $1",
            email.as_str()
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| ApiErrors::Unauthorized("Invalid credentials".to_string()))?;

        verify_password(password.as_str(), &record.password)
            .map_err(|e| ApiErrors::PasswordFail(e.to_string()))?;

        // let token = generate_token(record.id, &self.jwt_secret, self.jwt_expiry_hour)?;

        Ok(LoginResponse { id: record.id })
    }

    pub async fn get_user(&self, user_id: Uuid) -> Result<UserResponse, ApiErrors> {
        let user = sqlx::query!("SELECT id, email, name, phone_number, roles, created_at, updated_at FROM users WHERE id = $1", user_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| ApiErrors::NotFound("User not found".to_string()))?;

        Ok(UserResponse {
            id: user.id,
            email: Email(user.email),
            name: Text(user.name),
            phone_number: user.phone_number.map(PhoneNumber),
            roles: Roles::new(&user.roles)?,
            created_at: user.created_at,
            updated_at: user.updated_at,
        })
    }

    pub async fn get_all_users(&self) -> Result<Vec<UserResponse>, ApiErrors> {
        let users = sqlx::query!("SELECT id, email, name, phone_number, roles, created_at, updated_at FROM users ORDER BY created_at DESC")
            .fetch_all(&self.pool)
            .await
            .map_err(|_| ApiErrors::InternalServerError("Failed to fetch users".to_string()))?;

        // Ok(users
        //     .into_iter()
        //     .map(|u| UserResponse {
        //         id: u.id,
        //         email: Email(u.email),
        //         name: Text(u.name),
        //         phone_number: u.phone_number.map(PhoneNumber),
        //         roles: Roles::new(&u.roles)?,
        //         created_at: u.created_at,
        //         updated_at: u.updated_at,
        //     })
        //     .collect())

        let users = users
            .into_iter()
            .map(|u| {
                Ok(UserResponse {
                    id: u.id,
                    email: Email(u.email),
                    name: Text(u.name),
                    phone_number: u.phone_number.map(PhoneNumber),
                    roles: Roles::new(&u.roles)
                        .map_err(|e| ApiErrors::InternalServerError(e.to_string()))?,
                    created_at: u.created_at,
                    updated_at: u.updated_at,
                })
            })
            .collect::<Result<Vec<_>, ApiErrors>>()?;

        Ok(users)
    }

    pub async fn update_user(&self, user: UpdatedData) -> Result<bool, ApiErrors> {
        let result = sqlx::query!(r#"UPDATE users SET name = $1, phone_number = $2, roles = $3, edited_by = $4, edited_by_name = $5, edited_by_email = $6 WHERE id = $7"#, 
                user.name.as_ref().map(|n| n.as_str()),
                user.phone_number.as_ref().map(|p| p.as_str()),
                user.roles.as_ref().map(|p| p.as_str()),
                user.edited_by,
                user.edited_by_name.as_str(),
                user.edited_by_email.as_str(),
                user.user_id,
            )
            .execute(&self.pool)
            .await
            .map_err(|_| ApiErrors::InternalServerError("Update failed".to_string()))?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn delete_user(&self, user_id: Uuid) -> Result<bool, ApiErrors> {
        let result = sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
            .execute(&self.pool)
            .await
            .map_err(|_| ApiErrors::InternalServerError("Delete failed".to_string()))?;

        Ok(result.rows_affected() > 0)
    }
}
