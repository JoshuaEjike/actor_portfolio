pub mod blog_api_routers;
pub mod image_api_routers;
pub mod project_api_routers;
pub mod refresh_token_routers;
pub mod stack_api_routers;
pub mod user_api_routers;

use axum::Router;

use crate::{
    api::{
        blog_api_routers::blog_api_router, image_api_routers::image_api_router,
        project_api_routers::project_api_router, refresh_token_routers::refresh_token_routers,
        stack_api_routers::stack_api_router,
    },
    state::AppState,
};

use user_api_routers::user_api_router;

pub fn app_apis(state: AppState) -> Router {
    Router::new().nest(
        "/api/v1",
        Router::new()
            .nest("/auth", user_api_router(state.clone()))
            .nest("/stack", stack_api_router(state.clone()))
            .nest("/image", image_api_router(state.clone()))
            .nest("/blog", blog_api_router(state.clone()))
            .nest("/project", project_api_router(state.clone()))
            .nest("/token", refresh_token_routers(state.clone())),
    )
}
