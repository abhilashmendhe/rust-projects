
use axum::{extract::{Path, State}, response::IntoResponse, Json};
use crate::models::{state::AppState, user::{UpdateUser, User}};

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

// handler for create user
pub async fn create_user(State(appstate): State<AppState>, Json(payload): Json<User>) -> String {

    let exist_user = appstate
            .users
            .lock()
            .await
            .clone()
            .into_iter()
            .find(|u| (*u).id == payload.id);
    
    if let Some(_) = exist_user {
        
        let tuser = User {
            id: *appstate.max_id.lock().await,
            name: payload.name.clone(),
            email: payload.email.clone()
        };
        *appstate.max_id.lock().await += 1;
        appstate.users.lock().await.push(tuser.clone());
        format!("Created a POST request for:\n{:?}", tuser)

    } else {
        appstate.users.lock().await.push(payload.clone());
        *appstate.max_id.lock().await = payload.id + 1;
        format!("Created a POST request for:\n{:?}", payload)
    }

}

pub async fn update_user(Path(id): Path<u64>, State(appstate): State<AppState>, Json(payload): Json<UpdateUser>) -> impl IntoResponse {
    println!("Called update user!");
    let index = appstate
                    .users
                    .lock()
                    .await
                    .clone()
                    .into_iter()
                    .position(|u| u.id == id);

    println!("Index: {:?}, id: {}", index, id);
    println!("{:?}", payload);
    if let Some(index) = index {
        
        appstate.users.lock().await.remove(index);
        let tuser = User {
            id: id,
            name: payload.name.clone(),
            email: payload.email.clone()
        };
        appstate.users.lock().await.push(tuser.clone());
        (axum::http::StatusCode::OK, Json(tuser)).into_response()
    } else {
        (axum::http::StatusCode::NOT_FOUND, "User not found to be updated!").into_response()
    }

}

// delete a single user by id
pub async fn delete_user(Path(id): Path<u64>, State(appstate): State<AppState>) ->  impl IntoResponse {

    let index = appstate
                    .users
                    .lock()
                    .await
                    .clone()
                    .into_iter()
                    .position(|u| u.id == id);

    if let Some(ind) = index {
        let u = appstate.users.lock().await.remove(ind);
        (axum::http::StatusCode::OK, Json(u)).into_response()
    } else {
        (axum::http::StatusCode::NOT_FOUND, format!("User not found to be deleted!!")).into_response()
    }
}

// get single user by id
pub async fn get_user(Path(id): Path<u64>, State(state): State<AppState>) ->  impl IntoResponse {
    
    match state
            .users
            .lock()
            .await
            .clone()
            .into_iter()
            .find(|u| (*u).id == id) {
                Some(user) => (axum::http::StatusCode::OK, Json(user)).into_response(),
                None=>{
                    (axum::http::StatusCode::NOT_FOUND, "User not found!!").into_response()
                }
            }
}

// get all users
pub async fn list_users(State(state): State<AppState>) -> impl IntoResponse {
    let users = state.users.lock().await.to_vec();
    let len = users.len();
    if len <= 0 {
        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "No users found!").into_response()
    } else {
        (axum::http::StatusCode::OK, Json(users)).into_response()
    }
}