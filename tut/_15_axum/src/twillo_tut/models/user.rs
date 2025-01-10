use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct UpdateUser {
    pub name: String,
    pub email: String,
}


pub fn fill_appstate_users() -> (Arc<Mutex<Vec<User>>>, Arc<Mutex<u64>>) {
    let users = vec![
        User {
            id: 1, 
            name: "Abhilash".to_string(),
            email: "abhi@gmail.com".to_string(),
            
        },
        User {
            id: 2,
            name: "Suchit".to_string(),
            email: "suchit@gmail.com".to_string(),
           
        },
        User {
            id: 3,
            name: "Kapil".to_string(),
            email: "kapil@gmail.com".to_string(),
            
        },
        User {
            id: 4,
            name: "Dikshant".to_string(),
            email: "dikshu@gmail.com".to_string(),
           
        }
    ];
    let max_val = users.iter().max_by(|a, b| a.id.partial_cmp(&b.id).unwrap()).unwrap().clone();
    (Arc::new(Mutex::new(users)), Arc::new(Mutex::new(max_val.id+1)))
}