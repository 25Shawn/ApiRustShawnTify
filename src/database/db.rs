use tokio_postgres::{Client, Error};
use postgres_native_tls::MakeTlsConnector;
use native_tls::TlsConnector;
use dotenv::dotenv;
use std::env;
use tokio;

pub async fn get_connection() -> Result<Client, Error> {
    dotenv().ok();

    let pg_user = env::var("PG_USER").expect("PG_USER must be set");
    let pg_password = env::var("PG_PASSWORD").expect("PG_PASSWORD must be set");
    let pg_host = env::var("PG_HOST").expect("PG_HOST must be set");
    let pg_port = env::var("PG_PORT").expect("PG_PORT must be set");
    let pg_database = env::var("PG_DATABASE").expect("PG_DATABASE must be set");

    let url = format!(
        "postgres://{}:{}@{}:{}/{}?sslmode=require", pg_user, pg_password, pg_host, pg_port, pg_database
    );
    
    let tls_connector = TlsConnector::new().expect("Failed to create TLS connector");
    let tls = MakeTlsConnector::new(tls_connector);

    
    let (client, connection) = tokio_postgres::connect(&url, tls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(client)
}
