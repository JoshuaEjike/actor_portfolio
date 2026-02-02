use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    stack::handlers::{create_stack, delete_stack, get_all_stack, get_single_stack, update_stack},
    state::AppState,
};

pub fn stack_api_router(state: AppState) -> Router {
    Router::new()
        .route("/create", post(create_stack))
        .route("/all", get(get_all_stack))
        .route(
            "/detail/{id}",
            get(get_single_stack)
                .patch(update_stack)
                .delete(delete_stack),
        )
        .with_state(state)
}
