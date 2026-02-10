mod api;
mod auth;
mod blog;
mod config;
mod core;
mod errors;
mod extractor;
mod fields;
mod image;
mod payload_handler;
mod project;
mod refresh_token;
mod response;
mod stack;
mod state;
mod utils;

use std::net::SocketAddr;

use sqlx::postgres::PgPoolOptions;
use tokio::{net::TcpListener, sync::mpsc};

use auth::{actor::AuthActor, messages::AuthMessage};

use crate::{
    api::app_apis,
    blog::{actor::BlogActor, messages::BlogMessage},
    config::Config,
    image::{actor::ImageActor, messages::ImageMessage},
    project::{actor::ProjectActor, messages::ProjectMessage},
    refresh_token::{
        actor::RefreshTokenActor, messages::RefreshTokenMessage, repo_sqlx::RefreshTokenRepoSqlx,
    },
    stack::{actor::StackActor, messages::StackMessage},
    state::AppState,
};

#[tokio::main]
async fn main() {
    let config = Config::from_env();

    let pool = PgPoolOptions::new()
        .max_connections(config.db_pool_max_connections.unwrap_or(12))
        .connect(&config.database_url)
        .await
        .unwrap();

    let (auth_tx, auth_rx) = mpsc::channel::<AuthMessage>(32);

    let (stack_tx, stack_rx) = mpsc::channel::<StackMessage>(32);

    let (image_tx, image_rx) = mpsc::channel::<ImageMessage>(32);

    let (blog_tx, blog_rx) = mpsc::channel::<BlogMessage>(32);

    let (project_tx, project_rx) = mpsc::channel::<ProjectMessage>(32);

    let (refresh_token_tx, refresh_token_rx) = mpsc::channel::<RefreshTokenMessage>(32);

    tokio::spawn(
        AuthActor::new(
            pool.clone(),
        )
        .run(auth_rx),
    );

    tokio::spawn(StackActor::new(pool.clone()).run(stack_rx));

    tokio::spawn(
        ImageActor::new(
            config.cloud_name,
            config.cloud_api_key,
            config.cloud_api_secret,
        )
        .run(image_rx),
    );

    tokio::spawn(BlogActor::new(pool.clone()).run(blog_rx));

    tokio::spawn(ProjectActor::new(pool.clone()).run(project_rx));

    let refresh_token_repo = RefreshTokenRepoSqlx { pool: pool.clone() };

    tokio::spawn(
        RefreshTokenActor::new(
            refresh_token_repo,
            config.jwt_secret.clone(),
            config.jwt_expiry_hour,
        )
        .run(refresh_token_rx),
    );

    let app_state = AppState {
        auth_tx,
        stack_tx,
        image_tx,
        blog_tx,
        project_tx,
        refresh_token_tx,
        jwt_secret: config.jwt_secret.clone(),
    };

    let app = app_apis(app_state);

    let port = config.port;

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let listener = TcpListener::bind(addr).await.unwrap();

    println!("🚀 Server runnings at http://{addr}");

    axum::serve(listener, app).await.unwrap();
}
