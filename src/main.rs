use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use mysql::*;
use serde::Serialize;
use serde::Deserialize;
mod database;
use database::db::get_connection;
mod models;
use sqlx::FromRow;
use tokio_postgres::{NoTls, Error};
use std::env;
use dotenv::dotenv;


#[derive(Debug, FromRow, Serialize)]
struct Musique {
    id: i32,
    uuid: String,
    duree: String,
}

#[derive(Deserialize)]
struct AddPlaylistParams{
    nom_playlist: String,
    id_createur: String,
}

#[derive(Deserialize)]
struct AddMusiqueParams {
    uuid: String,
}

#[derive(Serialize)]
struct ResponseMessage {
    message: String,
}

#[derive(Deserialize)]
struct User {
    nom_utilisateur: String,
    mots_passe: String,
}

#[derive(Serialize)]  // Assurez-vous que la structure est sérialisable
struct Utilisateur {
    id: i32,
    nom_utilisateur: String,
}

/**
* Gestion de la musique
*/
async fn add_musique(params: web::Json<AddMusiqueParams>) -> impl Responder {
    let file_path = format!("./src/musiques/{}", &params.uuid);

    // Obtenir la durée du fichier audio
    let total_duration = get_audio_duration(&file_path);

    if total_duration.is_zero() {
        return HttpResponse::InternalServerError().json(ResponseMessage {
            message: "Impossible d'obtenir la durée de l'audio".to_string(),
        });
    }

    let client = match get_connection().await {
        Ok(client) => client,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ResponseMessage {
                message: "Erreur de connexion à la base de données".to_string(),
            });
        }
    };

    let query = "INSERT INTO musique (uuid, duree) VALUES ($1, $2)";
    if let Err(_) = client.execute(query, &[&params.uuid, &(total_duration.as_secs().to_string())]).await {

        return HttpResponse::InternalServerError().json(ResponseMessage {
            message: "Erreur lors de l'ajout de la musique".to_string(),
        });
    }

    HttpResponse::Ok().json(ResponseMessage {
        message: format!("Musique ajoutée avec succès: {}", &params.uuid),
    })
}

// Fonction pour obtenir la durée du fichier audio
fn get_audio_duration(file_path: &str) -> std::time::Duration {
    match mp3_duration::from_path(file_path) {
        Ok(duration) => {
            //println!("Duration: {} seconds", duration.as_secs());
            duration
        }
        Err(e) => {
            println!("Failed to get duration: {}", e);
            std::time::Duration::new(0, 0)  // Retourne une durée de 0 en cas d'erreur
        }
    }
}


async fn get_musique(uuid: web::Path<String>) -> impl Responder {
    let client = match get_connection().await {
        Ok(client) => client,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ResponseMessage {
                message: "Erreur de connexion à la base de données".to_string(),
            });
        }
    };

    let query = "SELECT id, uuid, duree FROM musique WHERE uuid = $1";
    let row = match client.query_opt(query, &[&uuid.into_inner()]).await {
        Ok(Some(row)) => row,
        Ok(None) => {
            return HttpResponse::NotFound().json(ResponseMessage {
                message: "Musique non trouvée".to_string(),
            });
        }
        Err(_) => {
            return HttpResponse::InternalServerError().json(ResponseMessage {
                message: "Erreur lors de la récupération de la musique".to_string(),
            });
        }
    };

    let musique = Musique {
        id: row.get(0),
        uuid: row.get(1),
        duree: row.get::<_, String>(2) as String,
    };

    HttpResponse::Ok().json(musique)
}


async fn get_all_musiques() -> impl Responder {
    let client = match get_connection().await {
        Ok(client) => client,
        Err(error) => {
            return HttpResponse::InternalServerError().json(ResponseMessage {
                message: error.to_string(),
            });
        }
    };

    let query = "SELECT id, uuid, duree FROM musique";
    let rows = match client.query(query, &[]).await {
        Ok(rows) => rows,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ResponseMessage {
                message: "Erreur lors de la récupération des musiques".to_string(),
            });
        }
    };

    let musiques: Vec<Musique> = rows
    .iter()
    .map(|row| Musique {
        id: row.get::<_, i32>(0),  // Assurez-vous que le type correspond (i32 pour l'ID)
        uuid: row.get::<_, String>(1),  // Le type attendu pour UUID est String
        duree: row.get::<_, String>(2),  // La durée peut être un String, ou un autre type selon la table
    })
    .collect();

    

    HttpResponse::Ok().json(musiques)
}

async fn supprimer_musique(uuid: web::Path<String>) -> impl Responder {
    let client = match get_connection().await {
        Ok(client) => client,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ResponseMessage {
                message: "Erreur de connexion à la base de données".to_string(),
            });
        }
    };

    let query = "DELETE FROM musique WHERE uuid = $1";
    if let Err(_) = client.execute(query, &[&uuid.into_inner()]).await {
        return HttpResponse::InternalServerError().json(ResponseMessage {
            message: "Erreur lors de la suppression de la musique".to_string(),
        });
    }

    HttpResponse::Ok().json(ResponseMessage {
        message: "Musique supprimée avec succès".to_string(),
    })
}

async fn add_playlist(
    params: web::Json<AddPlaylistParams>,
) -> impl Responder {
    let client = match get_connection().await {
        Ok(client) => client,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ResponseMessage {
                message: "Erreur de connexion à la base de données".to_string(),
            });
        }
    };
    let query = "INSERT INTO playlist (nom_playlist, id_createur, nombre_morceaux) VALUES ($1, $2, 0)";
    if let Err(_) = client.execute(query, &[&params.nom_playlist, &params.id_createur]).await {
        return HttpResponse::InternalServerError().json(ResponseMessage {
            message: "Erreur lors de l'ajout de la playlist".to_string(),
        });
    }

    HttpResponse::Ok().json(ResponseMessage {
        message: format!("Playlist '{}' ajoutée avec succès", &params.nom_playlist),
    })
}

async fn supprimer_playlist(
    id: web::Path<i32>,
) -> impl Responder {
    let client = match get_connection().await {
        Ok(client) => client,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ResponseMessage {
                message: "Erreur de connexion à la base de données".to_string(),
            });
        }
    };
    let query = "DELETE FROM playlist WHERE id = $1";
    if let Err(_) = client.execute(query, &[&id.into_inner()]).await {
        return HttpResponse::InternalServerError().json(ResponseMessage {
            message: "Erreur lors de la suppression de la playlist".to_string(),
        });
    }

    HttpResponse::Ok().json(ResponseMessage {
        message: "Playlist supprimée avec succès".to_string(),
    })
}


// Fonction pour ajouter une musique à une playlist
#[derive(Deserialize)]
struct AddMusiqueToPlaylistParams {
    id_musique: i32,
    id_playlist: i32,
}
async fn add_musique_to_playlist(
    params: web::Json<AddMusiqueToPlaylistParams>,
) -> impl Responder {
    let client = match get_connection().await {
        Ok(client) => client,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ResponseMessage {
                message: "Erreur de connexion à la base de données".to_string(),
            });
        }
    };
    let query = "UPDATE musique SET id_playlist = $1 WHERE id = $2";
    if let Err(_) = client.execute(query, &[&params.id_playlist, &params.id_musique]).await {
        return HttpResponse::InternalServerError().json(ResponseMessage {
            message: "Erreur lors de l'ajout de la musique à la playlist".to_string(),
        });
    }

    let query_update = "UPDATE playlist SET nombre_morceaux = nombre_morceaux + 1 WHERE id = $1";
    if let Err(_) = client.execute(query_update, &[&params.id_playlist]).await {
        return HttpResponse::InternalServerError().json(ResponseMessage {
            message: "Erreur lors de la mise à jour du nombre de morceaux".to_string(),
        });
    }

    HttpResponse::Ok().json(ResponseMessage {
        message: "Musique ajoutée à la playlist avec succès".to_string(),
    })
}

// Fonction pour supprimer une musique d'une playlist
#[derive(Deserialize)]
struct RemoveMusiqueFromPlaylistParams {
    id_musique: i32,
    id_playlist: i32,
}
async fn remove_musique_from_playlist(params: web::Json<RemoveMusiqueFromPlaylistParams>) -> impl Responder {
    let conn = match get_connection().await {
        Ok(conn) => conn,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ResponseMessage {
                message: "Erreur de connexion à la base de données".to_string(),
            });
        }
    };

    let query = "UPDATE musique SET id_playlist = NULL WHERE id = $1";
    if let Err(_) = conn.execute(query, &[&params.id_musique]).await {
        return HttpResponse::InternalServerError().json(ResponseMessage {
            message: "Erreur lors de la suppression de la musique de la playlist".to_string(),
        });
    }

    let query = "UPDATE playlist SET nombre_morceaux = nombre_morceaux - 1 WHERE id = $1";
    if let Err(_) = conn.execute(query, &[&params.id_playlist]).await {
        return HttpResponse::InternalServerError().json(ResponseMessage {
            message: "Erreur lors de la mise à jour du nombre de morceaux".to_string(),
        });
    }

    HttpResponse::Ok().json(ResponseMessage {
        message: "Musique supprimée de la playlist avec succès".to_string(),
    })
}

#[derive(Serialize)]
struct Playlist {
    id: i32,
    nom_playlist: String,
    id_createur: i32,
    nombre_morceaux: i32,
}

// Fonction pour récupérer toutes les playlists
async fn get_all_playlists() -> impl Responder {
    // Connexion à la base de données
    let conn = match get_connection().await {
        Ok(conn) => conn,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ResponseMessage {
                message: "Erreur de connexion à la base de données".to_string(),
            });
        }
    };

    let query = "SELECT id, nom_playlist, id_createur, nombre_morceaux FROM playlist";

    // Exécution de la requête pour récupérer les playlists
    match conn.query(query, &[]).await {
        Ok(rows) => {
            // Mapper les résultats de la requête (rows) dans une liste de playlists
            let playlists: Vec<Playlist> = rows
                .iter()
                .map(|row| Playlist {
                    id: row.get("id"),
                    nom_playlist: row.get("nom_playlist"),
                    id_createur: row.get("id_createur"),
                    nombre_morceaux: row.get("nombre_morceaux"),
                })
                .collect();

            // Retourner la liste des playlists sérialisée en JSON
            HttpResponse::Ok().json(playlists)
        }
        Err(_) => {
            HttpResponse::InternalServerError().json(ResponseMessage {
                message: "Erreur lors de la récupération des playlists".to_string(),
            })
        }
    }
}
async fn get_playlist(id: web::Path<i32>) -> impl Responder {
    let conn = match get_connection().await {
        Ok(conn) => conn,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ResponseMessage {
                message: "Erreur de connexion à la base de données".to_string(),
            });
        }
    };

    let query = "SELECT id, nom_playlist, id_createur, nombre_morceaux FROM playlist WHERE id = $1";
    
    // Extraire l'id de `web::Path`
    let id_value = id.into_inner();

    // Passer l'id_value directement comme référence d'un type compatible avec `ToSql`
    match conn.query(query, &[&(id_value)]).await {
        Ok(rows) => {
            // Si la playlist existe, on récupère les données de la première ligne
            if let Some(row) = rows.iter().next() {
                // Mapper les données dans la structure `Playlist`
                let playlist = Playlist {
                    id: row.get("id"),
                    nom_playlist: row.get("nom_playlist"),
                    id_createur: row.get("id_createur"),
                    nombre_morceaux: row.get("nombre_morceaux"),
                };
                
                // Retourner la réponse sous forme de JSON
                HttpResponse::Ok().json(playlist)
            } else {
                // Si aucune playlist n'est trouvée
                HttpResponse::NotFound().json(ResponseMessage {
                    message: "Playlist non trouvée".to_string(),
                })
            }
        }
        Err(_) => {
            HttpResponse::InternalServerError().json(ResponseMessage {
                message: "Erreur lors de la récupération de la playlist".to_string(),
            })
        }
    }
}




async fn add_user(user: web::Json<User>) -> impl Responder {
    let conn = match get_connection().await {
        Ok(conn) => conn,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ResponseMessage {
                message: "Erreur de connexion à la base de données".to_string(),
            });
        }
    };

    let query = "INSERT INTO utilisateur (nom_utilisateur, mots_passe) VALUES ($1, $2)";
    if let Err(_) = conn.execute(query, &[&user.nom_utilisateur, &user.mots_passe]).await {
        return HttpResponse::InternalServerError().json(ResponseMessage {
            message: "Erreur lors de l'ajout de l'utilisateur".to_string(),
        });
    }

    HttpResponse::Ok().json(ResponseMessage {
        message: "Utilisateur ajouté avec succès".to_string(),
    })
}


async fn connexion_user(user: web::Json<User>) -> impl Responder {
    let conn = match get_connection().await {
        Ok(conn) => conn,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ResponseMessage {
                message: "Erreur de connexion à la base de données".to_string(),
            });
        }
    };

    let query = "SELECT id, nom_utilisateur FROM utilisateur WHERE nom_utilisateur = $1 AND mots_passe = $2";
    
    match conn.query(query, &[&user.nom_utilisateur, &user.mots_passe]).await {
        Ok(rows) => {
            if let Some(row) = rows.iter().next() {
                // Extraire les données du `Row` et les mapper dans un struct `Utilisateur`
                let utilisateur = Utilisateur {
                    id: row.get("id"),
                    nom_utilisateur: row.get("nom_utilisateur"),
                };
                // Sérialiser et retourner la réponse JSON
                HttpResponse::Ok().json(utilisateur)
            } else {
                HttpResponse::Unauthorized().json(ResponseMessage {
                    message: "Nom d'utilisateur ou mot de passe incorrect".to_string(),
                })
            }
        }
        Err(_) => HttpResponse::InternalServerError().json(ResponseMessage {
            message: "Erreur lors de la connexion de l'utilisateur".to_string(),
        }),
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server_handle = start_server();
    server_handle.await?;
    Ok(())
}

fn start_server() -> actix_web::dev::Server {
    dotenv().ok();

    // Récupérer le port et le convertir en u16
    let port: u16 = env::var("PG_PORT")
    .unwrap_or_else(|_| "5432".to_string())
        .parse()
        .expect("PG_PORT doit être un entier valide");

    // Lancer votre application sur ce port
    println!("Lancement du serveur sur le port {}", port);
    HttpServer::new(|| {
        App::new()
            // Définition des routes
            .route("/addMusique", web::post().to(add_musique))
            .route("/musiques", web::get().to(get_all_musiques))
            .route("/musique/{uuid}", web::get().to(get_musique))
            .route("/supprimer/{uuid}", web::delete().to(supprimer_musique))
            .route("/addPlaylist", web::post().to(add_playlist))
            .route("/playlist", web::get().to(get_all_playlists))
            .route("/playlist/{id}", web::get().to(get_playlist))
            .route("/supprimerPlaylist/{id}", web::delete().to(supprimer_playlist))
            .route("/addMusiqueToPlaylist", web::post().to(add_musique_to_playlist))
            .route("/removeMusiqueFromPlaylist", web::post().to(remove_musique_from_playlist))
            .route("/addUser", web::post().to(add_user))
            .route("/user", web::get().to(connexion_user))
    })
    .bind(("0.0.0.0", port)) // Adresse et port
    .expect("Échec de la liaison à l'adresse")
    .run()
}








