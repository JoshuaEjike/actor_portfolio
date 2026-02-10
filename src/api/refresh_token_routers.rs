use crate::{refresh_token::handlers::{logout, refresh}, state::AppState};
use axum::{Router, routing::post};
use tower_cookies::CookieManagerLayer;

pub fn refresh_token_routers(state: AppState) -> Router {
    Router::new()
        .route("/refresh", post(refresh))
        .route("/logout", post(logout))
        .layer(CookieManagerLayer::new())
        .with_state(state)
}
