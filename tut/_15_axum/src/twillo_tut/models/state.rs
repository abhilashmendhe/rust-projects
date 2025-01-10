use std::sync::Arc;

use tokio::sync::Mutex;

use super::user::User;

#[derive(Debug, Clone)]
pub struct AppState {
    pub health_check: String,
    pub visit: Arc<Mutex<u32>>,
    pub users: Arc<Mutex<Vec<User>>>,
    pub max_id: Arc<Mutex<u64>>
}