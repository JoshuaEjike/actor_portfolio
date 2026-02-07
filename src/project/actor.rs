use sqlx::PgPool;
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::{
    errors::api_errors::ApiErrors,
    fields::text::Text,
    project::{
        dispatcher::project_dispatcher,
        dto::{CreateProjectData, UpdatedProjectData},
        messages::{ProjectMessage, ProjectResponse},
    },
};

pub struct ProjectActor {
    pool: PgPool,
}

impl ProjectActor {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn run(self, rx: mpsc::Receiver<ProjectMessage>) {
        project_dispatcher(&self, rx).await;
    }

    pub async fn create_project(&self, project: CreateProjectData) -> Result<Uuid, ApiErrors> {
        let id = Uuid::new_v4();

        let created_at = chrono::Utc::now().naive_utc();

        sqlx::query!(
            "INSERT INTO project (id, title, description, stack, content, image, image_id, created_by, created_by_name, created_by_email, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",
            id,
            project.title.as_str(),
            project.description.as_str(),
            project.stack.as_str(),
            project.content,
            project.image,
            project.image_id,
            project.created_by,
            project.created_by_name.as_str(),
            project.created_by_email.as_str(),
            created_at,
            created_at,
        )
        .execute(&self.pool)
        .await
        .map_err(|_| ApiErrors::Conflict("Project already exists".to_string()))?;

        Ok(id)
    }

    pub async fn get_single_project(&self, project_id: Uuid) -> Result<ProjectResponse, ApiErrors> {
        let blog = sqlx::query!(
            "SELECT id, title, description, stack, content, image, image_id, created_at, updated_at FROM project WHERE id = $1",
            project_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| ApiErrors::NotFound("Project not found".to_string()))?;

        Ok(ProjectResponse {
            id: blog.id,
            title: Text(blog.title),
            description: Text(blog.description),
            stack: Text(blog.stack),
            content: blog.content,
            image: blog.image,
            image_id: blog.image_id,
            created_at: blog.created_at,
            updated_at: blog.updated_at,
        })
    }

    pub async fn get_all_project(&self) -> Result<Vec<ProjectResponse>, ApiErrors> {
        let blog = sqlx::query!(
            "SELECT id, title, description, stack, content, image, image_id, created_at, updated_at FROM project ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|_| ApiErrors::InternalServerError("Failed to fetch project".to_string()))?;

        Ok(blog
            .into_iter()
            .map(|blg| ProjectResponse {
                id: blg.id,
                title: Text(blg.title),
                description: Text(blg.description),
                stack: Text(blg.stack),
                content: blg.content,
                image: blg.image,
                image_id: blg.image_id,
                created_at: blg.created_at,
                updated_at: blg.updated_at,
            })
            .collect())
    }

    pub async fn update_project(&self, blog: UpdatedProjectData) -> Result<bool, ApiErrors> {
        let result = sqlx::query!(r#"UPDATE project SET description = COALESCE($1, description), stack = COALESCE($2, stack), content = COALESCE($3, content), image = COALESCE($4, image), image_id = COALESCE($5, image_id), edited_by = $6, edited_by_name = $7, edited_by_email = $8 WHERE id = $9"#, 
                blog.description.as_ref().map(|s| s.as_str()),
                blog.stack.as_ref().map(|s| s.as_str()),
                blog.content,
                blog.image,
                blog.image_id,
                blog.edited_by,
                blog.edited_by_name.as_str(),
                blog.edited_by_email.as_str(),
                blog.project_id,
            )
            .execute(&self.pool)
            .await
            .map_err(|e| ApiErrors::InternalServerError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(ApiErrors::NotFound("Project not found".to_string()));
        }

        Ok(result.rows_affected() > 0)
    }

    pub async fn delete_project(&self, project_id: Uuid) -> Result<bool, ApiErrors> {
        let result = sqlx::query!("DELETE FROM project WHERE id = $1", project_id)
            .execute(&self.pool)
            .await
            .map_err(|_| ApiErrors::InternalServerError("Project Delete failed".to_string()))?;

        if result.rows_affected() == 0 {
            return Err(ApiErrors::NotFound("Project not found".to_string()));
        }

        Ok(result.rows_affected() > 0)
    }
}
