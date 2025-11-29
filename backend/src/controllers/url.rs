use axum::{extract::Path, response::Response};
pub struct UrlController;
use crate::services::UrlService;

impl UrlController {
    pub async fn shorten_url(url: String) -> Response {
        UrlService::add_url(url).await
    }
    pub async fn get_url_redirect(Path(code): Path<String>) -> Response {
        UrlService::get_url_response(code.to_string()).await
    }
}
