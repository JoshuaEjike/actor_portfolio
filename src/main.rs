mod api;
mod auth;
mod config;
mod core;
mod errors;
mod extractor;
mod fields;
mod payload_handler;
mod response;
mod stack;
mod state;

use std::net::SocketAddr;

use sqlx::postgres::PgPoolOptions;
use tokio::{net::TcpListener, sync::mpsc};

use auth::{actor::AuthActor, messages::AuthMessage};

use crate::{
    api::app_apis,
    config::Config,
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

    tokio::spawn(
        AuthActor::new(
            pool.clone(),
            config.jwt_secret.clone(),
            config.jwt_expiry_hour,
        )
        .run(auth_rx),
    );

    tokio::spawn(StackActor::new(pool.clone()).run(stack_rx));

    let app_state = AppState {
        auth_tx,
        stack_tx,
        jwt_secret: config.jwt_secret.clone(),
    };

    let app = app_apis(app_state);

    let port = config.port;

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let listener = TcpListener::bind(addr).await.unwrap();

    println!("🚀 Server runnings at http://{addr}");

    axum::serve(listener, app).await.unwrap();
}
