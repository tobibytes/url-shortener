use axum::{Router, routing::{get, post}, response::{ Json, IntoResponse, Response}};
mod services;
mod controllers;
mod secrets;
mod db;
mod models;
use controllers::UrlController;
#[tokio::main]
async fn main() {
    let port = "8000";
let app = Router::new()
    .route("/", get(root))
    .route("/{code}", get( UrlController::get_url_redirect))
    .route("/url", post(UrlController::shorten_url))
    .route("/health", get(get_health));
println!("running on {}", port);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await .unwrap();
    axum::serve(listener, app).await.unwrap();
}
async fn root() {}
async fn get_health() -> Response {
    Json(String::from("Ok")).into_response()
}
