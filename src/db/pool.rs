use std::env;

use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub async fn create_pool() -> Result<Pool<Postgres>, sqlx::Error> {
    dotenvy::dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
}
