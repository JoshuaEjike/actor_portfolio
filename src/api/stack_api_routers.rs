use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    stack::handlers::{
        create_stack, delete_stack, get_all_stack, get_single_stack, get_single_stack_by_title,
        update_stack,
    },
    state::AppState,
};

pub fn stack_api_router(state: AppState) -> Router {
    Router::new()
        .route("/create", post(create_stack))
        .route("/all", get(get_all_stack))
        .route("/by/{stack_title}", get(get_single_stack_by_title))
        .route(
            "/detail/{id}",
            get(get_single_stack)
                .patch(update_stack)
                .delete(delete_stack),
        )
        .with_state(state)
}
