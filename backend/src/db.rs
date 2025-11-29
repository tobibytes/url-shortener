use std::sync::Mutex;
use postgres::Client;
use once_cell::sync::Lazy;
use native_tls::TlsConnector;
use crate::secrets::SECRET_MANAGER;

pub static DBSERVICE: Lazy<Mutex<DbService>> = Lazy::new(|| {
    Mutex::new(DbService::new().expect("failed to initialize database client"))
});
pub struct DbService { client: Client }
impl DbService {
    pub fn new() -> Result<Self, postgres::Error> {
        let connector =  TlsConnector::builder().build().unwrap();
        let tls = postgres_native_tls::MakeTlsConnector::new(connector);
        let connection_string = SECRET_MANAGER.get("DB_CONNECTION_STRING");
        let client = Client::connect(connection_string.as_str(), tls)?;
        Ok(DbService { client })
    }

    pub fn add_url(&mut self, code: String, url: String) -> Result<(String, String), postgres::Error> {
        let res = self.client.query_one("INSERT INTO url_table (code, url) VALUES ($1, $2) RETURNING code, url", &[&code, &url]);
        match res {
            Ok(row) => Ok((row.get("code"), row.get("url"))),
            Err(err) => Err(err),
        }
    }
    pub fn get_url_from_code(&mut self, code: String) -> Result<(String, String), postgres::Error> {
        let row_res = self.client.query_one("SELECT url FROM url_table WHERE code = $1", &[&code]);
        match row_res {
            Ok(row) => Ok((code, row.get("url"))),
            Err(err) => Err(err),
        }
    }
    pub fn get_url(&mut self, url: String) -> Result<(String, String), postgres::Error> {
        let row_res = self.client.query_one("SELECT code FROM url_table WHERE url = $1", &[&url]);
        match row_res {
            Ok(row) => Ok((row.get("code"), url)),
            Err(err) => Err(err),
        }
    }
}
