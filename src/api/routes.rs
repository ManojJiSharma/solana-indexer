use axum::Router;
use axum::routing::get;

use crate::api::{app_state::AppState, handlers};

pub fn routers() -> Router<AppState>{
    Router::new()
        .route("/health", get(handlers::health))
        .route("/tx/{signature}", get(handlers::get_tx))
        .route("/slot/{slot}/txs", get(handlers::get_slot_txs))
}
