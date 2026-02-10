use axum::{
    Router,
    routing::{get, post},
};

use tower_cookies::CookieManagerLayer;

use crate::{
    auth::handlers::{delete_user, get_all_users, get_user, login, register, update_user},
    state::AppState,
};

pub fn user_api_router(state: AppState) -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/users", get(get_all_users))
        .route(
            "/users/{id}",
            get(get_user).patch(update_user).delete(delete_user),
        )
        .layer(CookieManagerLayer::new())
        .with_state(state)
}
