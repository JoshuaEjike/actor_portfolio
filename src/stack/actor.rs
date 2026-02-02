use sqlx::PgPool;
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::{
    errors::api_errors::ApiErrors,
    fields::text::Text,
    stack::{
        dispatcher::stack_dispatcher,
        dto::{CreateStackData, UpdatedStackData},
        messages::{StackMessage, StackResponse},
    },
};

pub struct StackActor {
    pool: PgPool,
}

impl StackActor {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn run(self, rx: mpsc::Receiver<StackMessage>) {
        stack_dispatcher(&self, rx).await;
    }

    pub async fn create_stack(&self, stack: CreateStackData) -> Result<Uuid, ApiErrors> {
        let id = Uuid::new_v4();

        let created_at = chrono::Utc::now().naive_utc();

        sqlx::query!(
            "INSERT INTO stack (id, title, slug, created_by, created_by_name, created_by_email, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
            id,
            stack.title.as_str(),
            stack.slug.as_str(),
            stack.created_by,
            stack.created_by_name.as_str(),
            stack.created_by_email.as_str(),
            created_at,
            created_at,
        )
        .execute(&self.pool)
        .await
        .map_err(|_| ApiErrors::Conflict("Stack already exists".to_string()))?;

        Ok(id)
    }

    pub async fn get_single_stack(&self, stack_id: Uuid) -> Result<StackResponse, ApiErrors> {
        let stack = sqlx::query!(
            "SELECT id, title, slug, created_at, updated_at FROM stack WHERE id = $1",
            stack_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| ApiErrors::NotFound("Stack not found".to_string()))?;

        Ok(StackResponse {
            id: stack.id,
            title: Text(stack.title),
            slug: Text(stack.slug),
            created_at: stack.created_at,
            updated_at: stack.updated_at,
        })
    }

    pub async fn get_all_stack(&self) -> Result<Vec<StackResponse>, ApiErrors> {
        let stack = sqlx::query!(
            "SELECT id, title, slug, created_at, updated_at FROM stack ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|_| ApiErrors::InternalServerError("Failed to fetch stack".to_string()))?;

        Ok(stack
            .into_iter()
            .map(|sck| StackResponse {
                id: sck.id,
                title: Text(sck.title),
                slug: Text(sck.slug),
                created_at: sck.created_at,
                updated_at: sck.updated_at,
            })
            .collect())
    }

    pub async fn update_stack(&self, stack: UpdatedStackData) -> Result<bool, ApiErrors> {
        let result = sqlx::query!(r#"UPDATE stack SET slug = $1, edited_by = $2, edited_by_name = $3, edited_by_email = $4 WHERE id = $5"#, 
                stack.slug.as_ref().map(|s| s.as_str()),
                stack.edited_by,
                stack.edited_by_name.as_str(),
                stack.edited_by_email.as_str(),
                stack.stack_id,
            )
            .execute(&self.pool)
            .await
            .map_err(|_| ApiErrors::InternalServerError("Update failed".to_string()))?;

        if result.rows_affected() == 0 {
            return Err(ApiErrors::NotFound("Stack not found".to_string()));
        }

        Ok(result.rows_affected() > 0)
    }

    pub async fn delete_stack(&self, stack_id: Uuid) -> Result<bool, ApiErrors> {
        let result = sqlx::query!("DELETE FROM stack WHERE id = $1", stack_id)
            .execute(&self.pool)
            .await
            .map_err(|_| ApiErrors::InternalServerError("Delete failed".to_string()))?;

        if result.rows_affected() == 0 {
            return Err(ApiErrors::NotFound("Stack not found".to_string()));
        }

        Ok(result.rows_affected() > 0)
    }
}
