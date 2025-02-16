use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}
