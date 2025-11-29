use crate::db::DBSERVICE;
use axum::http::{
    HeaderValue, Method,
    header::{ACCEPT, ACCESS_CONTROL_ALLOW_CREDENTIALS, AUTHORIZATION, CONTENT_TYPE, COOKIE},
};
use axum::{
    Router,
    response::{IntoResponse, Json, Response},
    routing::{get, post},
};
use reqwest::StatusCode;
use tower_http::cors::CorsLayer;
mod controllers;
mod db;
mod models;
mod secrets;
mod services;
use controllers::UrlController;

use crate::secrets::SECRET_MANAGER;
#[tokio::main]
async fn main() {
    let port = SECRET_MANAGER.get("PORT");
    let frontend_url = SECRET_MANAGER.get("FRONTEND_URL");
    let allowed_origin: HeaderValue = frontend_url.parse().expect("Invalid FRONTEND_URL for CORS");
    let cors = CorsLayer::new()
        .allow_origin(allowed_origin)
        .allow_credentials(true)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([
            AUTHORIZATION,
            CONTENT_TYPE,
            ACCEPT,
            COOKIE,
            ACCESS_CONTROL_ALLOW_CREDENTIALS,
        ]);
    let app = Router::new()
        .route("/", get(root))
        .route("/url", post(UrlController::shorten_url))
        .route("/health", get(get_health))
        .route("/{code}", get(UrlController::get_url_redirect))
        .layer(cors);
    println!("running on {}", port);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
async fn root() -> Response {
    (
        StatusCode::OK,
        Json(String::from("Welcome to url shortener")),
    )
        .into_response()
}
async fn get_health() -> Response {
    let mut result = std::collections::HashMap::new();
    let is_db_healthy = DBSERVICE.check_db_health().await;
    let is_api_healthy = root().await.status().is_success();
    result.insert("is_db_healthy", is_db_healthy);
    result.insert("is_api_healthy", is_api_healthy);
    let is_healthy = is_db_healthy && true;
    if is_healthy {
        return (StatusCode::OK, Json(result)).into_response();
    }
    return (StatusCode::INTERNAL_SERVER_ERROR, Json(result)).into_response();
}
