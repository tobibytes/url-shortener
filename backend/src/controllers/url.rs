use axum::{response::{ Response } };
pub struct UrlController;
use crate::{services::UrlService};

impl UrlController {
    pub async fn shorten_url(url: String) -> Response {
        UrlService::add_url(url)
    }
    pub async fn get_url_redirect(code: String) -> Response {
        UrlService::get_url_response(code)
    }
}
