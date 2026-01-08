use axum::Router;
use tokio::sync::mpsc;

use crate::{
    api::app_state::{self},
    db::pool::create_pool,
    indexer::listener::connect_rpc,
};

mod api;
mod db;
mod indexer;

#[tokio::main]
async fn main() {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();

    let pool = create_pool().await.expect("Error while creating pool");

    let (tx, rx) = mpsc::channel(10);

    tokio::spawn(async move {
        let url = "wss://api.devnet.solana.com";
        if let Err(e) = connect_rpc(url, tx).await {
            eprintln!("WebSocket error: {}", e);
        }
    });
    let value = pool.clone();
    tokio::spawn(async move {
        db::writer::run_writer(rx, pool).await;
    });

    let state = app_state::AppState { db: value };

    let app = Router::new()
        .merge(api::routes::routers())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Error on tcp listener bind");
    axum::serve(listener, app).await.unwrap();
}
