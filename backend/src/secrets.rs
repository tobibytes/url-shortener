use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::env;
pub static SECRET_MANAGER: Lazy<SecretManager> = Lazy::new(|| SecretManager::new());

enum MODE {
    DEV,
    PROD,
}

pub struct SecretManager {
    secrets: HashMap<String, String>,
}
impl SecretManager {
    fn new() -> Self {
        let mut secrets: HashMap<String, String> = HashMap::new();
        let mode = match env::var("MODE") {
            Ok(mode) if mode.to_lowercase() == "prod" => MODE::PROD,
            _ => MODE::DEV,
        };
        match mode {
            MODE::DEV => {
                secrets.insert(
                    "DB_CONNECTION_STRING".to_string(),
                    "host=database:5432 user=postgres password=postgres dbname=postgres".to_string(),
                );
                secrets.insert("PORT".to_string(), "8000".to_string());
                secrets.insert(
                    "FRONTEND_URL".to_string(),
                    "http://localhost:3000".to_string(),
                );
                secrets.insert(
                    "BACKEND_URL".to_string(),
                    "http://localhost:8000".to_string(),
                );
                secrets.insert("BACKEND_DOMAIN".to_string(), "localhost".to_string());
            }
            MODE::PROD => {
                secrets.insert("DB_CONNECTION_STRING".to_string(), env::var("DB_CONNECTION_STRING").unwrap_or_default());
                secrets.insert("PORT".to_string(), env::var("PORT").unwrap_or_default());
                secrets.insert(
                    "FRONTEND_URL".to_string(),
                    env::var("FRONTEND_URL").unwrap_or_default(),
                );
                secrets.insert(
                    "BACKEND_URL".to_string(),
                    env::var("BACKEND_URL").unwrap_or_default(),
                );
                }
        }
        secrets.insert("CODE_SECRET".to_string(), "code_secret".to_string());
        println!("Secrets loaded: {:?}", secrets);
        SecretManager { secrets }
    }

    pub fn get(&self, key: &str) -> String {
        self.secrets.get(key).cloned().unwrap_or_default()
    }
}
