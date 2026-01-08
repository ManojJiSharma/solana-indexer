use crate::indexer::listener::LogEvent;
use sqlx::{Pool, Postgres, QueryBuilder};
use std::time::Duration;
use tokio::sync::mpsc::Receiver;
use tokio::time::interval;
use tracing::{error, info};

pub async fn run_writer(mut rx: Receiver<LogEvent>, pool: Pool<Postgres>) {
    let mut buffer = Vec::new();
    let mut interval = interval(Duration::from_secs(10));

    loop {
        tokio::select! {
            Some(event) = rx.recv() => {
                buffer.push(event);

                if buffer.len() >= 50 {
                    info!("Pushing {} events (buffer trigger)", buffer.len());
                    if let Err(e) = insert_batch(&pool, &buffer).await {
                        error!("DB insert failed {:?}",e);
                    }
                    buffer.clear();
                }
            }

            _ = interval.tick() => {
                if !buffer.is_empty() {
                    info!("Pushing {} events (time trigger)", buffer.len());
                    if let Err(e) = insert_batch(&pool, &buffer).await {
                        error!("DB insert failed {:?}",e);
                    }
                    buffer.clear();
                }
            }
        }
    }
}

async fn insert_batch(pool: &Pool<Postgres>, buffer: &[LogEvent]) -> Result<(), sqlx::Error> {
    let mut qb = QueryBuilder::new("INSERT INTO transactions (signature, slot, logs)");

    qb.push_values(buffer.iter(), |mut b, event| {
        b.push_bind(&event.signature)
            .push_bind(event.slot as i64)
            .push_bind(&event.logs);
    });
    qb.push(" ON CONFLICT (signature) DO NOTHING");

    qb.build().execute(pool).await?;
    Ok(())
}
