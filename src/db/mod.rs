pub mod user;
pub mod event;
pub mod invitations;
use crate::PGPool;
use log::{info, warn};
use sqlx::postgres::PgPoolOptions;

pub async fn init_db_pool(db_url: &str) -> PGPool {
    warn!("database url: {}", db_url);
    let pool: PGPool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .unwrap();
    info!("{}", "Connect with postgresql".to_string());
    pool
}