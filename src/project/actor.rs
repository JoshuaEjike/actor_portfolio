use sqlx::{PgPool, Postgres, QueryBuilder};
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::{
    errors::api_errors::ApiErrors,
    project::{
        dispatcher::project_dispatcher,
        dto::{CreateProjectData, ProjectQuery, UpdatedProjectData},
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
            "INSERT INTO project (id, title, description, company, role, start_date, end_date, tag, link, stack, content, word_count, image, image_id, created_by, created_by_name, created_by_email, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19)",
            id,
            project.title,
            project.description,
            project.company,
            project.role,
            project.start_date,
            project.end_date,
            project.tag,
            project.link,
            project.stack,
            project.content,
            project.word_count,
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
        let project = sqlx::query!(
            "SELECT id, title, description, company, role, start_date, end_date, tag, link, stack, content, word_count, image, image_id, created_at, updated_at FROM project WHERE id = $1",
            project_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| ApiErrors::NotFound("Project not found".to_string()))?;

        Ok(ProjectResponse {
            id: project.id,
            title: project.title,
            description: project.description,
            company: project.company,
            role: project.role,
            start_date: project.start_date,
            end_date: project.end_date,
            tag: project.tag,
            link: project.link,
            stack: project.stack,
            content: project.content,
            word_count: project.word_count,
            image: project.image,
            image_id: project.image_id,
            created_at: project.created_at,
            updated_at: project.updated_at,
        })
    }

    pub async fn get_all_project(
        &self,
        query: ProjectQuery,
    ) -> Result<(Vec<ProjectResponse>, u64), ApiErrors> {
        let page = query.page.unwrap_or(1);
        let limit = query.limit.unwrap_or(10);
        let offset = (page - 1) * limit;

        // 🔹 MAIN QUERY
        let mut qb = QueryBuilder::<Postgres>::new(
            "SELECT id, title, description, company, role, start_date, end_date, tag, link, stack, content, word_count, image, image_id, created_at, updated_at FROM project",
        );

        // 🔹 COUNT QUERY (for meta)
        let mut count_qb = QueryBuilder::<Postgres>::new("SELECT COUNT(*) as count FROM project");

        // 🔍 FILTER
        if let Some(title) = query.title {
            qb.push(" WHERE title ILIKE ");
            qb.push_bind(format!("%{}%", title));

            count_qb.push(" WHERE title ILIKE ");
            count_qb.push_bind(format!("%{}%", title));
        }

        // 🔹 ORDER + PAGINATION
        qb.push(" ORDER BY created_at DESC");
        qb.push(" LIMIT ");
        qb.push_bind(limit as i64);
        qb.push(" OFFSET ");
        qb.push_bind(offset as i64);

        // 🔹 FETCH DATA
        let projects = qb
            .build_query_as::<ProjectResponse>()
            .fetch_all(&self.pool)
            .await
            .map_err(|_| ApiErrors::InternalServerError("Failed to fetch project".into()))?;

        // 🔹 FETCH COUNT
        let total: (i64,) = count_qb
            .build_query_as()
            .fetch_one(&self.pool)
            .await
            .map_err(|_| ApiErrors::InternalServerError("Failed to count project".into()))?;

        Ok((projects, total.0 as u64))
    }

    pub async fn update_project(&self, project: UpdatedProjectData) -> Result<bool, ApiErrors> {
        let result = sqlx::query!(r#"UPDATE project SET description = COALESCE($1, description), company = COALESCE($2, company), role = COALESCE($3, role), start_date = COALESCE($4, start_date), end_date = COALESCE($5, end_date), tag = COALESCE($6, tag), link = COALESCE($7, link), stack = COALESCE($8, stack), content = COALESCE($9, content), word_count = COALESCE($10, word_count), image = COALESCE($11, image), image_id = COALESCE($12, image_id), edited_by = $13, edited_by_name = $14, edited_by_email = $15 WHERE id = $16"#, 
                project.description,
                project.company,
                project.role,
                project.start_date,
                project.end_date,
                project.tag,
                project.link,
                project.stack,
                project.content,
                project.word_count,
                project.image,
                project.image_id,
                project.edited_by,
                project.edited_by_name.as_str(),
                project.edited_by_email.as_str(),
                project.project_id,
            )
            .execute(&self.pool)
            .await
            .map_err(|e| ApiErrors::InternalServerError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(ApiErrors::NotFound("Project not found".to_string()));
        }

        Ok(result.rows_affected() > 0)
    }

    pub async fn get_total_project_count(&self) -> Result<u64, ApiErrors> {
        let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM project")
            .fetch_one(&self.pool)
            .await
            .map_err(|_| ApiErrors::InternalServerError("Failed to count project".into()))?;

        Ok(total as u64)
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
