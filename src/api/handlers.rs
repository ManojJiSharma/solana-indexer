use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use serde::Deserialize;
use serde_json::json;

use crate::api::app_state::AppState;

pub async fn health() -> Json<serde_json::Value> {
    Json(json!({"Status":"ok"}))
}

#[derive(serde::Serialize, sqlx::FromRow)]
pub struct TransactionResponse {
    signature: String,
    slot: i64,
    logs: Vec<String>,
}

#[derive(Deserialize)]
pub struct Pagination {
    pub limit: Option<i64>,
}

pub async fn get_tx(
    Path(signature): Path<String>,
    State(pool): State<AppState>,
) -> Result<Json<TransactionResponse>, StatusCode> {
    let tx = sqlx::query_as::<_, TransactionResponse>(
        r#"
        SELECT signature, slot, logs
        FROM transactions
        WHERE signature = $1
        "#,
    )
    .bind(&signature)
    .fetch_optional(&pool.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match tx {
        Some(tx) => Ok(Json(tx)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn get_slot_txs(
    Path(slot): Path<i64>,
    Query(pagination): Query<Pagination>,
    State(state): State<AppState>,
) -> Result<Json<Vec<TransactionResponse>>, StatusCode> {
    let limit = pagination.limit.unwrap_or(50);
    let txs = sqlx::query_as::<_, TransactionResponse>(
        r#"
        SELECT signature, slot, logs
        FROM transactions
        WHERE slot = $1
        ORDER BY created_at DESC
        LIMIT $2
        "#
    )
    .bind(slot)
    .bind(limit)
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(txs))
}
