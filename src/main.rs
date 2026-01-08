use std::future::pending;

use tokio::sync::mpsc;

use crate::{db::pool::create_pool, indexer::listener::connect_rpc};

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

    tokio::spawn(async move {
        db::writer::run_writer(rx, pool).await;
    });

    pending::<()>().await;
}
