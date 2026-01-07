use tokio::sync::mpsc;
use tracing::info;

use crate::indexer::listener::connect_rpc;

mod indexer;

#[tokio::main]
async fn main() {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();

    let (tx, mut rx) = mpsc::channel(10);

    tokio::spawn(async move {
        let url = "wss://api.devnet.solana.com";
        if let Err(e) = connect_rpc(url, tx).await {
            eprintln!("WebSocket error: {}", e);
        }
    });

    while let Some(event) = rx.recv().await {
        info!(
            "TX: {} | slot: {} | logs: {}",
            event.signature,
            event.slot,
            event.logs.len()
        );
    }
}