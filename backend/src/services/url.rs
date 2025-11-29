use axum::{ http::StatusCode, response::{Response, Json, IntoResponse, Redirect } };
use base64::Engine;
use xxhash_rust::xxh64::xxh64;
use crate::{ db::DBSERVICE, models::{ UrlModel } };
pub struct UrlService;

impl UrlService {
    pub fn add_url(url_to_shorten: String ) -> Response {
        if let Some(existing_url) = UrlService::get_url(url_to_shorten.clone()) {
            return (StatusCode::OK, Json(existing_url)).into_response();
        }

        let code = UrlService::encode(&url_to_shorten);
        let url_model = UrlModel { code: code.clone(), url: url_to_shorten };
        let add_to_db = {
            let mut db = DBSERVICE.lock().unwrap();
            db.add_url(url_model.code.clone(), url_model.url.clone())
        };
        match add_to_db {
            Ok(url_result) => (StatusCode::OK, Json(UrlModel { code: url_result.0, url: url_result.1 })).into_response(),
            Err(_) => (StatusCode::BAD_REQUEST, Json("Error adding to db")).into_response()
        }
    }
    pub fn get_url_response(code: String) -> Response {
        let url_res = UrlService::get_url_from_code(code);
        match url_res {
            Some(url) => (Redirect::temporary(&url.url)).into_response(),
            None => (StatusCode::BAD_REQUEST, Json("Could not find url with code")).into_response()
        }
    }
    fn encode(url: &str) -> String {
        use base64::engine::general_purpose::STANDARD;
        let hash = xxh64(url.as_bytes(), 5000 as u64);
        let generated_code = STANDARD.encode(hash.to_le_bytes());
        generated_code
    }
    pub fn get_url(url: String) -> Option< UrlModel> {
        let url_result = {
            let mut db = DBSERVICE.lock().unwrap();
            db.get_url(url)
        };
        match url_result {
            Ok(url_res) => Some(UrlModel { code: url_res.0, url: url_res.1 }),
            Err(_) => None
        }
    }

    pub fn get_url_from_code(code: String) -> Option<UrlModel> {
        let url_res = {
            let mut db = DBSERVICE.lock().unwrap();
            db.get_url_from_code(code)
        };
        match url_res {
            Ok(url_result) => Some(UrlModel { code: url_result.0, url: url_result.1 }), 
            Err(_) => None,
        }
    }
}
