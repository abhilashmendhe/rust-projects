use std::{env, sync::Arc};

use axum::{routing::{get, post}, Router};
use dotenv::dotenv;
use routes::{create_user, fetch_single_user, fetch_users, get_visit_counts};
use models::state::AppState;
use sqlx::PgPool;
use tokio::sync::Mutex;

mod models;
mod routes;

#[tokio::main]
async fn main() {

    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file!!!");
    let db_pool = PgPool::connect(&database_url).await.unwrap();

    let shared_data = AppState {
        health_check: "Hey... You've already visited me...".to_string(),
        visit: Arc::new(Mutex::new(0)),
        db: db_pool.clone()
    };
    
    let server_url = env::var("HOST_PORT").expect("HOST_PORT is not set in .env file!!!");
    println!("Running on {}", server_url);
    // let f = fetch(db_pool).await;
    let app = Router::new()
        .route("/", get(|| async {"Hello, from Rust with DB conneciton!"}))
        .route("/health", get(get_visit_counts))
        .route("/user/{:id}", get(fetch_single_user))
        .route("/users", get(fetch_users))
        .route("/create-user", post(create_user))
        // .layer(Extension(db_pool))
        .with_state(shared_data);

    let listener = tokio::net::TcpListener::bind(server_url).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    // let hello_handler = || async {"Hello, Rust!"};

    // let app = Router::new()
    //         .route("/", get(hello_handler))
    //         .route("/health", get(get_visit_counts))
    //         .route("/create-user", post(create_user))
    //         .route("/delete-user/{:id}", delete(delete_user))
    //         .route("/user/{:id}", get(get_user))
    //         .route("/users", get(list_users))
    //         .with_state(shared_data);

    // println!("Running on http://localhost:8080");

    // let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    // axum::serve(listener, app).await.unwrap();

}

