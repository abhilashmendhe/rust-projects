use std::net::SocketAddr;
pub use self::error::{Error, Result};

use axum::body::Body;
use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse, Response};
use axum::{middleware, Router};
use axum::routing::{get, get_service};
use model::ModelController;
use serde::Deserialize;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

mod error;
mod web;
mod model;
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize model controller
    let mc = ModelController::new().await?;

    let routes_apis = web::routes_tickets::routes(mc.clone())
                    .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth::<Body>));
    let routes_all = Router::new()
                                .merge(routes_hello())
                                .merge(web::routes_login::routes())
                                .nest("/api", routes_apis)
                                .layer(middleware::map_response(main_response_mapper))
                                .layer(CookieManagerLayer::new())
                                .fallback_service(routes_static());
    

    // start server - Start
    let addr = SocketAddr::from(([127,0,0,1],8080));
    println!("->> Listening on {addr}\n");
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, routes_all.into_make_service()).await.unwrap();
    // start server - End

    Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:12} - main_response_mapper","RES_MAPPER");
    println!();
    res
}

// Static routing
fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

// Routes Hello
fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,

}
// e.g. `/hello?name=abhi`
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}","HANDLER");
    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("Hello <strong>{name}</strong>"))
}

// e.g. `/hello2/Abhi`
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello2 - {name:?}","HANDLER");
    Html(format!("Hello2 <strong>{name}</strong>"))
}
