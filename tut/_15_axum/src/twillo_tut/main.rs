use std::sync::Arc;

use tokio::sync::Mutex;

use axum::{routing::{delete, get, post, put}, Router};
use models::{state::AppState, user::fill_appstate_users};
use routes::{create_user, delete_user, get_user, get_visit_counts, list_users, update_user};

mod models;
mod routes;

#[tokio::main]
async fn main() {

    let (users, max_id) = fill_appstate_users();
   
    let shared_data = AppState {
        health_check: "Hey... You've already visited me...".to_string(),
        visit: Arc::new(Mutex::new(0)),
        users,
        max_id
    };
    
    let hello_handler = || async {"Hello, Rust!"};

    let app = Router::new()
            .route("/", get(hello_handler))
            .route("/health", get(get_visit_counts))
            .route("/create-user", post(create_user))
            .route("/delete-user/{:id}", delete(delete_user))
            .route("/user/{:id}", get(get_user))
            .route("/users", get(list_users))
            .route("/update-user/{:id}", put(update_user))
            .with_state(shared_data);

    println!("Running on http://localhost:8080");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

