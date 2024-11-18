use dotenv::dotenv;
use std::env;
use tokio_postgres::{NoTls, Client, Error};

pub async fn get_connection() -> Result<Client, Error> {
    // Charger les variables d'environnement
    dotenv().ok();

    // Lire les variables d'environnement
    let user = env::var("PG_USER").expect("PG_USER must be set");
    let password = env::var("PG_PASSWORD").unwrap_or_default(); // Si le mot de passe est vide, on le laisse vide
    let host = env::var("PG_HOST").expect("PG_HOST must be set");
    let port = env::var("PG_PORT").expect("PG_PORT must be set");
    let database = env::var("PG_DATABASE").expect("PG_DATABASE must be set");

    // Construire l'URL de connexion PostgreSQL
    let url = format!(
        "postgres://{}:{}@{}:{}/{}",
        user, password, host, port, database
    );

    // Se connecter à PostgreSQL
    let (client, connection) = tokio_postgres::connect(&url, NoTls).await?;

    // Lancer la connexion dans un thread séparé
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    Ok(client)
}



// pub async fn get_connection() -> Result<Client, Error> {
//     // Charger les variables d'environnement depuis le fichier .env
//     dotenv().ok();

//     // Lire les variables d'environnement à partir du fichier .env
//     let user = env::var("PG_USER").expect("PG_USER must be set");
//     let host = env::var("PG_HOST").expect("PG_HOST must be set");
//     let database = env::var("PG_DATABASE").expect("PG_DATABASE must be set");
//     let password = env::var("PG_PASSWORD").expect("PG_PASSWORD must be set");
//     let port = env::var("PG_PORT")
//         .expect("PG_PORT must be set")
//         .parse::<u16>()
//         .expect("Invalid PG_PORT");

//     // Construire l'URL de connexion PostgreSQL
//     let connection_string = format!(
//         "postgresql://{}:{}@{}:{}/{}",
//         user, password, host, port, database
//     );

//     // Connexion à la base de données PostgreSQL
//     let (client, connection) = tokio_postgres::connect(&connection_string, NoTls).await?;

//     // Lancer la connexion dans un thread asynchrone
//     tokio::spawn(connection);

//     Ok(client)
// }
