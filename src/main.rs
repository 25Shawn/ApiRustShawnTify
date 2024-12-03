use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use mysql::*;
use serde::Serialize;
use serde::Deserialize;
mod database;
use database::db::get_connection;
mod models;
use sqlx::FromRow;
use std::env;
use dotenv::dotenv;
use actix_multipart::Multipart;
use futures::StreamExt; // For stream extensions like `next()`
use tokio::fs::File; // For file handling
use tokio::io::AsyncWriteExt;
use actix_cors::Cors;
use std::path::Path;
use actix_files as fs;


#[derive(Debug, FromRow, Serialize)]
struct Musique {
    id: i32,
    uuid: String,
    duree: String,
    image: Option<String>,
    image_url: Option<String>,

}

#[derive(Deserialize)]
struct AddPlaylistParams{
    nom_playlist: String,
    id_createur: i32,
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

async fn add_musique(mut payload: Multipart) -> impl Responder {
    let save_path_audio = "./src/musiques/";
    let save_path_images = "./src/images/";

    // Créer les répertoires si nécessaires
    for path in [save_path_audio, save_path_images] {
        if !Path::new(path).exists() {
            if let Err(_) = std::fs::create_dir_all(path) {
                return HttpResponse::InternalServerError().json(ResponseMessage {
                    message: format!("Erreur lors de la création du répertoire {}", path),
                });
            }
        }
    }

    let mut audio_filename: Option<String> = None;
    let mut image_filename: Option<String> = None;

    // Traiter chaque partie du multipart
    while let Some(item) = payload.next().await {
        let mut field = match item {
            Ok(field) => field,
            Err(_) => {
                return HttpResponse::BadRequest().json(ResponseMessage {
                    message: "Erreur lors du traitement du fichier".to_string(),
                });
            }
        };

        // Obtenir le nom du fichier
        let filename = field
            .content_disposition()
            .get_filename()
            .unwrap_or_default()
            .to_string();

        // Déterminer le chemin du fichier en fonction de son type
        let filepath = if filename.ends_with(".mp3") {
            audio_filename = Some(filename.clone());
            format!("{}{}", save_path_audio, filename)
        } else if filename.ends_with(".png") || filename.ends_with(".jpg") || filename.ends_with(".jpeg") {
            image_filename = Some(filename.clone());
            format!("{}{}", save_path_images, filename)
        } else {
            return HttpResponse::BadRequest().json(ResponseMessage {
                message: format!("Type de fichier non supporté : {}", filename),
            });
        };

        // Sauvegarder le fichier localement
        let mut file = match File::create(&filepath).await {
            Ok(f) => f,
            Err(_) => {
                return HttpResponse::InternalServerError().json(ResponseMessage {
                    message: format!("Erreur lors de la sauvegarde du fichier : {}", filename),
                });
            }
        };

        while let Some(chunk) = field.next().await {
            let data = match chunk {
                Ok(data) => data,
                Err(_) => {
                    return HttpResponse::InternalServerError().json(ResponseMessage {
                        message: format!("Erreur lors de l'écriture du fichier : {}", filename),
                    });
                }
            };
            if let Err(_) = file.write_all(&data).await {
                return HttpResponse::InternalServerError().json(ResponseMessage {
                    message: format!("Erreur lors de l'écriture du fichier : {}", filename),
                });
            }
        }
    }

    // Vérifier si les deux fichiers ont été téléchargés
    if let (Some(audio), Some(image)) = (audio_filename, image_filename) {
        // Calculer la durée du fichier audio
        let audio_path = format!("{}{}", save_path_audio, audio);
        let total_duration = get_audio_duration(&audio_path);
        if total_duration.is_zero() {
            return HttpResponse::InternalServerError().json(ResponseMessage {
                message: "Impossible d'obtenir la durée de l'audio".to_string(),
            });
        }

        // Insérer dans la base de données
        let client = match get_connection().await {
            Ok(client) => client,
            Err(_) => {
                return HttpResponse::InternalServerError().json(ResponseMessage {
                    message: "Erreur de connexion à la base de données".to_string(),
                });
            }
        };

        let query = "INSERT INTO musique (uuid, duree, image) VALUES ($1, $2, $3)";
        let duree_str = total_duration.as_secs().to_string();

        if let Err(e) = client.execute(query, &[&audio, &duree_str, &image]).await {
            return HttpResponse::InternalServerError().json(ResponseMessage {
                message: format!("Erreur lors de l'insertion dans la base de données : {}", e),
            });
        }

        HttpResponse::Ok().json(ResponseMessage {
            message: "Musique ajoutée avec succès".to_string(),
        })
    } else {
        HttpResponse::BadRequest().json(ResponseMessage {
            message: "Les fichiers audio et image sont requis".to_string(),
        })
    }
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

#[derive(Serialize, Deserialize)]
struct MusiqueAffcherPlaylist {
    id: i32,
    image: String,
    uuid: String,
    duree: String,  // Durée en format chaîne (ex: "06:31")
}

async fn get_musiques_playlist(id: web::Path<i32> ) -> impl Responder {
    let client = match get_connection().await {
        Ok(client) => client,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ResponseMessage {
                message: "Erreur de connexion à la base de données".to_string(),
            });
        }
    };

    let query = "SELECT id, image, uuid, duree FROM musique WHERE id_playlist = $1";
    let rows = match client.query(query, &[&id.into_inner()]).await {
        Ok(rows) => rows,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ResponseMessage {
                message: "Erreur lors de la récupération des musiques de la playlist".to_string(),
            });
        }
    };

    let mut musiques: Vec<MusiqueAffcherPlaylist> = Vec::new();
    for row in rows {
        let id: i32 = row.get(0);
        let image: String = row.get(1);
        let uuid: String = row.get(2);
        let duree_str: String = row.get(3);  // Durée sous forme de chaîne

        // Créer une instance de Musique et l'ajouter à la liste
        let musique = MusiqueAffcherPlaylist {
            id,
            image,
            uuid,
            duree: duree_str,
        };
        musiques.push(musique);
    }

    HttpResponse::Ok().json(musiques)  // Renvoie la liste des musiques
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

    let query = "SELECT uuid FROM musique WHERE uuid = $1";
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

    let uuid: String = row.get(0);
    let file_url = format!("http://127.0.0.1:8080/musiques/{}", uuid);

    HttpResponse::Ok().json(serde_json::json!({
        "url": file_url
    }))
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

    let query = "SELECT id, uuid, duree, image FROM musique";
    let rows = match client.query(query, &[]).await {
        Ok(rows) => rows,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ResponseMessage {
                message: "Erreur lors de la récupération des musiques".to_string(),
            });
        }
    };

    // Mapper les résultats de la requête vers un vecteur de musiques avec URL d'image
    let musiques: Vec<Musique> = rows
        .iter()
        .map(|row| {
            let image = row.get::<_, Option<String>>(3); // Option<String> pour gérer les images NULL
            let image_url = image.as_ref().map(|img| format!("https://apirustshawntify.onrender.com/images/{}", img));
            Musique {
                id: row.get::<_, i32>(0),
                uuid: row.get::<_, String>(1),
                duree: row.get::<_, String>(2),
                image,
                image_url, // Ajouter l'URL d'image ici
            }
        })
        .collect();

    // Retourner les musiques avec les URL d'images
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
#[derive(Deserialize)]
struct QueryParams {
    id_user: i32,
}

// Fonction pour récupérer toutes les playlists
async fn get_all_playlists(id_user: web::Query<QueryParams>) -> impl Responder {
    // Connexion à la base de données
    let conn = match get_connection().await {
        Ok(conn) => conn,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ResponseMessage {
                message: "Erreur de connexion à la base de données".to_string(),
            });
        }
    };

    let query = "SELECT id, nom_playlist, id_createur, nombre_morceaux FROM playlist WHERE id_createur = $1";
    let id_value = id_user.into_inner();

    // Exécution de la requête pour récupérer les playlists
    match conn.query(query, &[&id_value.id_user]).await {
        Ok(rows) => {
            // Transformer les résultats en un tableau de playlists
            let playlists: Vec<Playlist> = rows.iter().map(|row| Playlist {
                id: row.get("id"),
                nom_playlist: row.get("nom_playlist"),
                id_createur: row.get("id_createur"),
                nombre_morceaux: row.get("nombre_morceaux"),
            }).collect();

            HttpResponse::Ok().json(playlists) // Retourner un tableau JSON
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


async fn connexion_user(user: web::Query<User>) -> impl Responder {
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
                
                
                HttpResponse::Ok().json(utilisateur)
            } else {
                println!("Nom d'utilisateur ou mot de passe incorrect");
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
    if let Err(e) = start_server().await {
        eprintln!("Erreur lors du démarrage du serveur : {}", e);
    }
    Ok(())
}

pub async fn start_server() -> std::io::Result<()> {
    dotenv().ok();

    let port: u16 = env::var("PG_PORT")
        .unwrap_or_else(|_| "5432".to_string())
        .parse()
        .expect("PG_PORT doit être un entier valide");

    println!("Lancement du serveur sur le port {}", port);

    let server = HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600),
            )
            .route("/addMusique", web::post().to(add_musique))
            .route("/musiques", web::get().to(get_all_musiques))
            .route("/musique/{uuid}", web::get().to(get_musique))
            .route("/musiquePlaylist/{id}", web::get().to(get_musiques_playlist))
            .route("/supprimer/{uuid}", web::delete().to(supprimer_musique))
            .route("/addPlaylist", web::post().to(add_playlist))
            .route("/playlist", web::get().to(get_all_playlists))
            .route("/playlist/{id}", web::get().to(get_playlist))
            .route("/supprimerPlaylist/{id}", web::delete().to(supprimer_playlist))
            .route("/addMusiqueToPlaylist", web::post().to(add_musique_to_playlist))
            .route("/removeMusiqueFromPlaylist", web::post().to(remove_musique_from_playlist))
            .route("/addUser", web::post().to(add_user))
            .route("/user", web::get().to(connexion_user))
            .service(fs::Files::new("/musiques", "./src/musiques").show_files_listing())
            .service(fs::Files::new("/images", "./src/images").show_files_listing())
    });
    server.bind(format!("0.0.0.0:{}", port))?.run().await
}

////////////
//  Pour le localhost
////////////

// pub async fn start_server() -> std::io::Result<()> {
//     dotenv().ok();

//     HttpServer::new(|| {
//         App::new()
//         .wrap(Cors::default().allow_any_origin().allow_any_method().allow_any_header())
//             .route("/addMusique", web::post().to(add_musique))
//             .route("/musiques", web::get().to(get_all_musiques))
//             .route("/musique/{uuid}", web::get().to(get_musique))
//             .route("/musiquePlaylist/{id}", web::get().to(get_musiques_playlist))
//             .route("/supprimer/{uuid}", web::delete().to(supprimer_musique))
//             .route("/addPlaylist", web::post().to(add_playlist))
//             .route("/playlist", web::get().to(get_all_playlists))
//             .route("/playlist/{id}", web::get().to(get_playlist))
//             .route("/supprimerPlaylist/{id}", web::delete().to(supprimer_playlist))
//             .route("/addMusiqueToPlaylist", web::post().to(add_musique_to_playlist))
//             .route("/removeMusiqueFromPlaylist", web::post().to(remove_musique_from_playlist))
//             .route("/addUser", web::post().to(add_user))
//             .route("/user", web::get().to(connexion_user))
//             .service(fs::Files::new("/musiques", "./src/musiques").show_files_listing())
//             .service(fs::Files::new("/images", "./src/images").show_files_listing())
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }
