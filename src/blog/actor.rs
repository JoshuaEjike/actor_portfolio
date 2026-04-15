use sqlx::{PgPool, Postgres, QueryBuilder};
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::{
    blog::{
        dispatcher::blog_dispatcher,
        dto::{BlogQuery, CreateBlogData, UpdatedBlogData},
        messages::{BlogMessage, BlogResponse},
    },
    errors::api_errors::ApiErrors,
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
            "INSERT INTO blog (id, title, description, content, word_count, image, image_id, created_by, created_by_name, created_by_email, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",
            id,
            blog.title,
            blog.description,
            blog.content,
            blog.word_count,
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
            "SELECT id, title, description, content, word_count, image, image_id, created_at, updated_at FROM blog WHERE id = $1",
            blog_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| ApiErrors::NotFound("Blog not found".to_string()))?;

        Ok(BlogResponse {
            id: blog.id,
            title: blog.title,
            description: blog.description,
            content: blog.content,
            word_count: blog.word_count,
            image: blog.image,
            image_id: blog.image_id,
            created_at: blog.created_at,
            updated_at: blog.updated_at,
        })
    }

    // pub async fn get_all_blog(&self, query: BlogQuery) -> Result<Vec<BlogResponse>, ApiErrors> {
    //     let page = query.page.unwrap_or(1);
    //     let limit = query.limit.unwrap_or(10);

    //     let offset = (page - 1) * limit;

    //     let mut sql = String::from(
    //         "SELECT id, title, description, content, word_count, image, image_id, created_at, updated_at FROM blog",
    //     );

    //     let mut conditions = vec![];
    //     let mut args: Vec<String> = vec![];

    //     if let Some(title) = query.title {
    //         conditions.push(format!("title ILIKE ${}", args.len() + 1));
    //         args.push(format!("%{}%", title));
    //     }

    //     if !conditions.is_empty() {
    //         sql.push_str(" WHERE ");
    //         sql.push_str(&conditions.join(" AND "));
    //     }

    //     sql.push_str(&format!(
    //         " ORDER BY created_at DESC LIMIT {} OFFSET {}",
    //         limit, offset
    //     ));

    //     let mut query_builder = sqlx::query_as::<_, BlogResponse>(&sql);

    //     for arg in args {
    //         query_builder = query_builder.bind(arg);
    //     }

    //     let blogs = query_builder
    //         .fetch_all(&self.pool)
    //         .await
    //         .map_err(|_| ApiErrors::InternalServerError("Failed to fetch blog".to_string()))?;

    //     Ok(blogs)
    // }

    pub async fn get_all_blog(
        &self,
        query: BlogQuery,
    ) -> Result<(Vec<BlogResponse>, u64), ApiErrors> {
        let page = query.page.unwrap_or(1);
        let limit = query.limit.unwrap_or(10);
        let offset = (page - 1) * limit;

        // 🔹 MAIN QUERY
        let mut qb = QueryBuilder::<Postgres>::new(
            "SELECT id, title, description, content, word_count, image, image_id, created_at, updated_at FROM blog",
        );

        // 🔹 COUNT QUERY (for meta)
        let mut count_qb = QueryBuilder::<Postgres>::new("SELECT COUNT(*) as count FROM blog");

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
        let blogs = qb
            .build_query_as::<BlogResponse>()
            .fetch_all(&self.pool)
            .await
            .map_err(|_| ApiErrors::InternalServerError("Failed to fetch blog".into()))?;

        // 🔹 FETCH COUNT
        let total: (i64,) = count_qb
            .build_query_as()
            .fetch_one(&self.pool)
            .await
            .map_err(|_| ApiErrors::InternalServerError("Failed to count blog".into()))?;

        Ok((blogs, total.0 as u64))
    }

    pub async fn get_total_blog_count(&self) -> Result<u64, ApiErrors> {
        let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM blog")
            .fetch_one(&self.pool)
            .await
            .map_err(|_| ApiErrors::InternalServerError("Failed to count blog".into()))?;

        Ok(total as u64)
    }

    pub async fn update_blog(&self, blog: UpdatedBlogData) -> Result<bool, ApiErrors> {
        let result = sqlx::query!(r#"UPDATE blog SET title = COALESCE($1, title), description = COALESCE($2, description), content = COALESCE($3, content), word_count = COALESCE($4, word_count), image = COALESCE($5, image), image_id = COALESCE($6, image_id), edited_by = $7, edited_by_name = $8, edited_by_email = $9 WHERE id = $10"#, 
                blog.title,
                blog.description,
                blog.content,
                blog.word_count,
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
