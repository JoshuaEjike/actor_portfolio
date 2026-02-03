pub mod image_api_routers;
pub mod stack_api_routers;
pub mod user_api_routers;

use axum::Router;

use crate::{
    api::{image_api_routers::image_api_router, stack_api_routers::stack_api_router},
    state::AppState,
};

use user_api_routers::user_api_router;

pub fn app_apis(state: AppState) -> Router {
    Router::new().nest(
        "/api/v1",
        Router::new()
            .nest("/auth", user_api_router(state.clone()))
            .nest("/stack", stack_api_router(state.clone()))
            .nest("/image", image_api_router(state.clone())),
    )
}
