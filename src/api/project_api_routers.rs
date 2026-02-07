use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    project::handlers::{
        create_project, delete_project, get_all_project, get_single_project, update_project,
    },
    state::AppState,
};

pub fn project_api_router(state: AppState) -> Router {
    Router::new()
        .route("/create", post(create_project))
        .route("/all", get(get_all_project))
        .route(
            "/detail/{id}",
            get(get_single_project)
                .patch(update_project)
                .delete(delete_project),
        )
        .with_state(state)
}
