use sqlx::PgPool;
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::{
    blog::{
        dispatcher::blog_dispatcher,
        dto::{CreateBlogData, UpdatedBlogData},
        messages::{BlogMessage, BlogResponse},
    },
    errors::api_errors::ApiErrors,
    fields::text::Text,
};

pub struct BlogActor {
    pool: PgPool,
}

impl BlogActor {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn run(self, rx: mpsc::Receiver<BlogMessage>) {
        blog_dispatcher(&self, rx).await;
    }

    pub async fn create_blog(&self, blog: CreateBlogData) -> Result<Uuid, ApiErrors> {
        let id = Uuid::new_v4();

        let created_at = chrono::Utc::now().naive_utc();

        sqlx::query!(
            "INSERT INTO blog (id, title, description, content, image, image_id, created_by, created_by_name, created_by_email, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
            id,
            blog.title.as_str(),
            blog.description.as_str(),
            blog.content,
            blog.image,
            blog.image_id,
            blog.created_by,
            blog.created_by_name.as_str(),
            blog.created_by_email.as_str(),
            created_at,
            created_at,
        )
        .execute(&self.pool)
        .await
        .map_err(|_| ApiErrors::Conflict("Blog already exists".to_string()))?;

        Ok(id)
    }

    pub async fn get_single_blog(&self, blog_id: Uuid) -> Result<BlogResponse, ApiErrors> {
        let blog = sqlx::query!(
            "SELECT id, title, description, content, image, image_id, created_at, updated_at FROM blog WHERE id = $1",
            blog_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| ApiErrors::NotFound("Blog not found".to_string()))?;

        Ok(BlogResponse {
            id: blog.id,
            title: Text(blog.title),
            description: Text(blog.description),
            content: blog.content,
            image: blog.image,
            image_id: blog.image_id,
            created_at: blog.created_at,
            updated_at: blog.updated_at,
        })
    }

    pub async fn get_all_blog(&self) -> Result<Vec<BlogResponse>, ApiErrors> {
        let blog = sqlx::query!(
            "SELECT id, title, description, content, image, image_id, created_at, updated_at FROM blog ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|_| ApiErrors::InternalServerError("Failed to fetch project".to_string()))?;

        Ok(blog
            .into_iter()
            .map(|blg| BlogResponse {
                id: blg.id,
                title: Text(blg.title),
                description: Text(blg.description),
                content: blg.content,
                image: blg.image,
                image_id: blg.image_id,
                created_at: blg.created_at,
                updated_at: blg.updated_at,
            })
            .collect())
    }

    pub async fn update_blog(&self, blog: UpdatedBlogData) -> Result<bool, ApiErrors> {
        let result = sqlx::query!(r#"UPDATE blog SET description = COALESCE($1, description), content = COALESCE($2, content), image = COALESCE($3, image), image_id = COALESCE($4, image_id), edited_by = $5, edited_by_name = $6, edited_by_email = $7 WHERE id = $8"#, 
                blog.description.as_ref().map(|s| s.as_str()),
                blog.content,
                blog.image,
                blog.image_id,
                blog.edited_by,
                blog.edited_by_name.as_str(),
                blog.edited_by_email.as_str(),
                blog.blog_id,
            )
            .execute(&self.pool)
            .await
            .map_err(|e| ApiErrors::InternalServerError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(ApiErrors::NotFound("Blog not found".to_string()));
        }

        Ok(result.rows_affected() > 0)
    }

    pub async fn delete_blog(&self, blog_id: Uuid) -> Result<bool, ApiErrors> {
        let result = sqlx::query!("DELETE FROM blog WHERE id = $1", blog_id)
            .execute(&self.pool)
            .await
            .map_err(|_| ApiErrors::InternalServerError("Blog Delete failed".to_string()))?;

        if result.rows_affected() == 0 {
            return Err(ApiErrors::NotFound("Blog not found".to_string()));
        }

        Ok(result.rows_affected() > 0)
    }
}
