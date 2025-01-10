use axum::extract::{Path, State};
use axum::{response::IntoResponse, Json};
use serde_json::json;
use sqlx::Row;

use crate::models::state::AppState;
use crate::models::user::User;

// handler to get visit count
pub async fn get_visit_counts(State(state): State<AppState>) -> String {
    
    if *state.visit.lock().await == 0  {
        *state.visit.lock().await += 1;
        format!("Welcome.. You've visited for the first time")
    } else {
        *state.visit.lock().await += 1;
        format!("{}. Visit count: {}", state.health_check, *state.visit.lock().await)
    }
}

pub async fn fetch_users(State(state): State<AppState>) -> impl IntoResponse {
    let pool = state.db;
    let data = match sqlx::query("SELECT * FROM users")
    .fetch_all(&pool)
    .await {
        Ok(rows) => rows,
        Err(_) => {
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error!!",
            ).into_response()
        }
    };

    let users = data
        .into_iter()
        .map(|row| {
            json!({
                "id": row.try_get::<i32, _>("id").unwrap_or_default(),
                "name": row.try_get::<String, _>("name").unwrap_or_default(),
                "email": row.try_get::<String, _>("email").unwrap_or_default(),
            })
        })
        .collect::<serde_json::Value>();

    (axum::http::StatusCode::OK, Json(users)).into_response()

}

pub async fn fetch_single_user(State(state): State<AppState>, Path(id): Path<i32>) -> impl IntoResponse {
    let query = format!("SELECT * FROM users WHERE id={}",id);
    let pool = state.db;
    let data = match sqlx::query(&query)
        .fetch_all(&pool)
        .await {
            Ok(rows) => rows,
            Err(_) => {
                return (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error!!",
                ).into_response()
            }
        };
    // println!("{:?}",data);
    let ret_size = data.len();
    let user = data
            .into_iter()
            .map(|row| {
                json!({
                    "id": row.try_get::<i32, _>("id").unwrap_or_default(),
                    "name": row.try_get::<String, _>("name").unwrap_or_default(),
                    "email": row.try_get::<String, _>("email").unwrap_or_default(),
                })
            })
            .collect::<serde_json::Value>();
    if ret_size <= 0 {
        (axum::http::StatusCode::NOT_FOUND, Json(user)).into_response()        
    } else {
        (axum::http::StatusCode::OK, Json(user)).into_response()
    }
}

pub async fn create_user(State(state): State<AppState>, Json(payload): Json<User>) -> impl IntoResponse {

    let pool = state.db;
        
    let max_id_query = "SELECT MAX(id) FROM users";
    let max_id_row = sqlx::query(&max_id_query).fetch_one(&pool).await.unwrap();
    // let m_id = m
    let m_value = max_id_row.try_get::<i32,_>("max").unwrap() + 1;
    // println!("{:?}",m_value);
    let name = payload.name;
    let email = payload.email;
    let query = format!("INSERT INTO users (id,name,email) VALUES ({},'{}','{}')", m_value, name, email);
    // println!("{}",query);
    match sqlx::query(&query)
            .execute(&pool).await
    {
        Ok(row) => {
            // Ok(())
            let res = format!("User inserted into DB.");
            (axum::http::StatusCode::OK, res).into_response()
        },

        Err(_) => {
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Not able to insert the data into DB.").into_response()
        }
    }
}