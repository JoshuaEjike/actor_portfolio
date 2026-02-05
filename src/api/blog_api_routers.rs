use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    blog::handlers::{create_blog, delete_blog, get_all_blog, get_single_blog, update_blog},
    state::AppState,
};

pub fn blog_api_router(state: AppState) -> Router {
    Router::new()
        .route("/create", post(create_blog))
        .route("/all", get(get_all_blog))
        .route(
            "/detail/{id}",
            get(get_single_blog).patch(update_blog).delete(delete_blog),
        )
        .with_state(state)
}
