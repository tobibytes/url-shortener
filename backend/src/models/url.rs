use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrlModel {
    pub code: String,
    pub url: String,
}
