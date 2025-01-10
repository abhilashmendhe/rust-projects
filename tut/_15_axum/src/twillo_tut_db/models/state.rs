use std::sync::Arc;

use sqlx::PgPool;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct AppState {
    pub health_check: String,
    pub visit: Arc<Mutex<u32>>,
    pub db: PgPool
}