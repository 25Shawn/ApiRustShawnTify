
use dotenv::dotenv;

use mysql::{Pool, Opts, OptsBuilder};
use std::env;

pub fn get_connection() -> Pool {
    // Charger les variables d'environnement
    dotenv::dotenv().ok();

    // Lire l'URL de connexion à partir de l'environnement
    let url = env::var("URL_DB").expect("DB_URL must be set");

    // Créer des options de connexion à partir de l'URL
    let opts = Opts::from_url(&url).expect("Failed to parse database URL");

    // Créer un pool de connexions
    Pool::new(opts).expect("Failed to create pool")
}