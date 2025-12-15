use crate::secrets::SECRET_MANAGER;
use native_tls::TlsConnector;
use tokio::sync::OnceCell;
use tokio_postgres::Client;

pub static DB_CLIENT: OnceCell<Client> = OnceCell::const_new();
pub struct DbService;
pub static DBSERVICE: DbService = DbService;

impl DbService {
    async fn client(&self) -> &'static Client {
        DB_CLIENT
            .get_or_init(|| async {
                let connector = TlsConnector::builder()
                    .build()
                    .expect("failed to build TLS connector");
                let tls = postgres_native_tls::MakeTlsConnector::new(connector);
                let connection_string = SECRET_MANAGER.get("DB_CONNECTION_STRING");
                let (client, connection) = tokio_postgres::connect(connection_string.as_str(), tls)
                    .await
                    .expect("failed to connect to database");

                tokio::spawn(async move {
                    if let Err(err) = connection.await {
                        eprintln!("database connection error: {err}");
                    }
                });

                client
            })
            .await
    }

    pub async fn add_url(
        &self,
        code: &str,
        url: &str,
    ) -> Result<(String, String), tokio_postgres::Error> {
        let client = self.client().await;
        let row = client
            .query_one(
                "INSERT INTO url_table (code, url) VALUES ($1, $2) RETURNING code, url",
                &[&code, &url],
            )
            .await?;
        Ok((row.get("code"), row.get("url")))
    }

    pub async fn get_url_from_code(
        &self,
        code: &str,
    ) -> Result<(String, String), tokio_postgres::Error> {
        let client = self.client().await;
        let row = client
            .query_one("SELECT url FROM url_table WHERE code = $1", &[&code])
            .await?;
        Ok((code.to_string(), row.get("url")))
    }

    pub async fn get_url(&self, url: &str) -> Result<(String, String), tokio_postgres::Error> {
        let client = self.client().await;
        let row = client
            .query_one("SELECT code FROM url_table WHERE url = $1", &[&url])
            .await?;
        Ok((row.get("code"), url.to_string()))
    }

    pub async fn check_db_health(&self) -> bool {
        let client = self.client().await;
        match client.check_connection().await {
            Ok(_) => return true,
            Err(_) => return false,
        };
    }

    pub async fn create_table_if_not_exists(&self) -> Result<(), tokio_postgres::Error> {
        let client = self.client().await;
        client
            .execute(
                "CREATE TABLE IF NOT EXISTS url_table (
                    code VARCHAR(255) PRIMARY KEY,
                    url TEXT NOT NULL
                )",
                &[],
            )
            .await?;
        Ok(())
    }
}
