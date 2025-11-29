use crate::{db::DBSERVICE, models::UrlModel};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Redirect, Response},
};
use base64::Engine;
use xxhash_rust::xxh64::xxh64;
pub struct UrlService;

impl UrlService {
    pub async fn add_url(url_to_shorten: String) -> Response {
        if let Some(existing_url) = UrlService::get_url(url_to_shorten.clone()).await {
            return (StatusCode::OK, Json(existing_url)).into_response();
        }
        let is_valid_res = UrlService::url_is_valid(url_to_shorten.clone()).await;
        let is_valid = match is_valid_res {
            Ok(is_valid_) => is_valid_,
            Err(_) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json("could not find url, try again"),
                )
                    .into_response();
            }
        };
        if !is_valid {
            return (StatusCode::BAD_REQUEST, Json("Invalid url")).into_response();
        }

        let code = UrlService::encode(&url_to_shorten);
        let url_model = UrlModel {
            code: code.clone(),
            url: url_to_shorten,
        };
        let add_to_db = DBSERVICE.add_url(&url_model.code, &url_model.url).await;
        match add_to_db {
            Ok(url_result) => (
                StatusCode::OK,
                Json(UrlModel {
                    code: url_result.0,
                    url: url_result.1,
                }),
            )
                .into_response(),
            Err(_) => (
                StatusCode::BAD_REQUEST,
                Json("Problem with your request, try again"),
            )
                .into_response(),
        }
    }
    pub async fn get_url_response(code: String) -> Response {
        let url_res = UrlService::get_url_from_code(code).await;
        match url_res {
            Some(url) => (Redirect::temporary(&url.url)).into_response(),
            None => (
                StatusCode::BAD_REQUEST,
                Json("Could not find url with code"),
            )
                .into_response(),
        }
    }
    fn encode(url: &str) -> String {
        use base64::engine::general_purpose::STANDARD;
        let hash = xxh64(url.as_bytes(), 5000 as u64);
        let generated_code = STANDARD.encode(hash.to_le_bytes());
        generated_code
    }
    pub async fn get_url(url: String) -> Option<UrlModel> {
        let url_result = DBSERVICE.get_url(&url).await;
        match url_result {
            Ok(url_res) => Some(UrlModel {
                code: url_res.0,
                url,
            }),
            Err(_) => None,
        }
    }

    pub async fn get_url_from_code(code: String) -> Option<UrlModel> {
        let url_res = DBSERVICE.get_url_from_code(&code).await;
        match url_res {
            Ok(url_result) => Some(UrlModel {
                code: url_result.0,
                url: url_result.1,
            }),
            Err(_) => None,
        }
    }
    pub async fn url_is_valid(url: String) -> Result<bool, std::io::Error> {
        let response_result = reqwest::get(&url).await;
        let resp = match response_result {
            Ok(resp_) => resp_,
            Err(_) => return Ok(false),
        };
        let status = resp.status();
        return Ok(status.is_success());
        // let body_res = resp.text().await;
        // let body = match body_res {
        //     Ok(body_) => body_,
        //     Err(_) => return Ok(false),
        // };
        // println!("{}", body);
        // return Ok(true);
    }
}
