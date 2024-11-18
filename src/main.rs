use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use std::thread;
use std::fs;
use rodio::{Decoder, OutputStream, Sink};
use mysql::*;
use mysql::prelude::*;
use mysql::{params, prelude::*};
use serde::Serialize;
use serde::Deserialize;
use std::env;
use dotenv::dotenv;
mod database;
use database::db::get_connection;
mod models;
use models::musique::Musique as MusiqueModel;
use sqlx::FromRow;
use chrono::NaiveTime;
use std::convert::TryInto;
use tokio_postgres::{Client, NoTls, Error};


#[derive(Debug, FromRow, Serialize)]
struct Musique {
    id: i32,
    uuid: String,
    duree: String,
}

#[derive(Deserialize)]
struct AddPlaylistParams{
    nom_playlist: String,
    id_createur: i32,
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

// async fn add_musique(params: web::Json<AddMusiqueParams>) -> impl Responder {
//     let directory = "./src/musiques/";
//     let file_path = format!("{}{}", directory, &params.uuid);

//     //println!("File path: {}", file_path);
//     let file = File::open(&file_path).expect("Failed to open file");

//     // Obtenir la durée du fichier audio
//     let total_duration = get_audio_duration(&file_path);

//     let pool = get_connection();
//     let mut conn = pool.get_conn().expect("Failed to get connection");

//     // Insertion de la musique dans la base de données
//     conn.exec_drop(
//         "INSERT INTO musique (uuid, duree) VALUES (:uuid, :duree)",
//         params! {
//             "uuid" => &params.uuid,
//             "duree" => total_duration.as_secs(),
//         },
//     ).expect("Failed to insert musique");

//     HttpResponse::Ok().json(ResponseMessage { message: format!("Musique ajoutée avec succès: {}", &params.uuid) })
// }

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

// Fonction pour obtenir toutes les musiques
// async fn get_all_musiques() -> impl Responder {
//     let musiques = web::block(|| getMusiques()).await;

//     match musiques {
//         Ok(musiques) => HttpResponse::Ok().json(musiques),
//         Err(_) => HttpResponse::InternalServerError().json(ResponseMessage {
//             message: "Erreur lors de la récupération des musiques".to_string(),
//         }),
//     }
// }

// Fonction pour obtenir une musique par UUID
// async fn get_musique(uuid: web::Path<String>) -> impl Responder {
//     //println!("UUID: {}", uuid);
//     let musique = web::block(move || get_musique_inner(&uuid)).await;

//     match musique {
//         Ok(Some(musique)) => HttpResponse::Ok().json(musique),
//         Ok(None) => HttpResponse::NotFound().json(ResponseMessage {
//             message: "Musique non trouvée".to_string(),
//         }),
//         Err(_) => HttpResponse::InternalServerError().json(ResponseMessage {
//             message: "Erreur lors de la récupération de la musique".to_string(),
//         }),
//     }
// }
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

// Fonction interne pour récupérer une musique de la base de données
// fn get_musique_inner(uuid: &str) -> Option<(i32, String, String)> {  // Utiliser String pour uuid et String pour duree
//     let pool = get_connection();
//     let mut conn = pool.get_conn().expect("Failed to get connection");

//     println!("UUID: {}", uuid);

//     // Utiliser exec_first pour obtenir un seul résultat
//     let result: Option<(i32, Vec<u8>, String)> = conn.exec_first(
//         "SELECT id, uuid, duree FROM musique WHERE uuid = ?",
//         (uuid,),
//     ).expect("Erreur lors de l'exécution de la requête");

//     // Convertir les résultats dans le format souhaité
//     let converted_result = result.map(|(id, uuid_bytes, duree)| {
//         let uuid_string = String::from_utf8(uuid_bytes).unwrap_or_else(|_| "Invalid UUID".to_string());
//         let duree_string = duree;  // Assurez-vous que duree est de type String
//         (id, uuid_string, duree_string)
//     });

//     println!("Result: {:?}", converted_result);
//     converted_result
// } 
async fn get_all_musiques() -> impl Responder {
    let client = match get_connection().await {
        Ok(client) => client,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ResponseMessage {
                message: "Erreur de connexion à la base de données".to_string(),
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
            id: row.get(0),
            uuid: row.get(1),
            duree: row.get::<_, String>(2) as String,
        })
        .collect();

    HttpResponse::Ok().json(musiques)
}

// fn getMusiques() -> Vec<Musique> {
//     let pool = get_connection();
//     let mut conn = pool.get_conn().expect("Failed to get connection");

//     let result: Vec<Musique> = conn
//         .query_map(
//             "SELECT id, uuid, duree FROM musique",
//             |(id, uuid, duree)| Musique { id, uuid, duree },
//         )
//         .expect("Erreur lors de l'exécution de la requête");

//     result
// }

// Fonction pour supprimer une musique par UUID
// async fn supprimer_musique(uuid: web::Path<String>) -> impl Responder {
//     let pool = get_connection();
//     let mut conn = match pool.get_conn() {
//         Ok(conn) => conn,
//         Err(_) => {
//             return HttpResponse::InternalServerError().json(ResponseMessage {
//                 message: "Erreur de connexion à la base de données".to_string(),
//             });
//         }
//     };

//     // Tentative de suppression de la musique avec l'UUID fourni
//     let result = conn.exec_drop(
//         "DELETE FROM musique WHERE uuid = ?",
//         (uuid.into_inner(),),
//     );

//     match result {
//         Ok(_) => HttpResponse::Ok().json(ResponseMessage {
//             message: "Musique supprimée avec succès".to_string(),
//         }),
//         Err(_) => HttpResponse::InternalServerError().json(ResponseMessage {
//             message: "Erreur lors de la suppression de la musique".to_string(),
//         }),
//     }
// }

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


// Gestion des playlists

// Fonction pour ajouter une playlist
// async fn add_playlist(params: web::Json<AddPlaylistParams>) -> impl Responder {
//     let pool = get_connection();
//     let mut conn = match pool.get_conn() {
//         Ok(conn) => conn,
//         Err(_) => {
//             return HttpResponse::InternalServerError().json(ResponseMessage {
//                 message: "Erreur de connexion à la base de données".to_string(),
//             });
//         }
//     };

//     let result = conn.exec_drop(
//         "INSERT INTO playlist (nom_playlist, id_createur, nombre_morceaux) VALUES (:nom_playlist, :id_createur, 0)",
//         params! {
//             "nom_playlist" => &params.nom_playlist,
//             "id_createur" => &params.id_createur,
//         },
//     );

//     match result {
//         Ok(_) => HttpResponse::Ok().json(ResponseMessage {
//             message: format!("Playlist '{}' ajoutée avec succès", &params.nom_playlist),
//         }),
//         Err(_) => HttpResponse::InternalServerError().json(ResponseMessage {
//             message: "Erreur lors de l'ajout de la playlist".to_string(),
//         }),
//     }
// }
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
// Fonction pour supprimer une playlist
// async fn supprimer_playlist(id: web::Path<u32>) -> impl Responder {
//     let pool = get_connection();
//     let mut conn = match pool.get_conn() {
//         Ok(conn) => conn,
//         Err(_) => {
//             return HttpResponse::InternalServerError().json(ResponseMessage {
//                 message: "Erreur de connexion à la base de données".to_string(),
//             });
//         }
//     };

//     let result = conn.exec_drop(
//         "DELETE FROM playlist WHERE id = ?",
//         (id.into_inner(),),
//     );

//     match result {
//         Ok(_) => HttpResponse::Ok().json(ResponseMessage {
//             message: "Playlist supprimée avec succès".to_string(),
//         }),
//         Err(_) => HttpResponse::InternalServerError().json(ResponseMessage {
//             message: "Erreur lors de la suppression de la playlist".to_string(),
//         }),
//     }
// }
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

// async fn add_musique_to_playlist(params: web::Json<AddMusiqueToPlaylistParams>) -> impl Responder {
//     let pool = get_connection();
//     let mut conn = match pool.get_conn() {
//         Ok(conn) => conn,
//         Err(_) => {
//             return HttpResponse::InternalServerError().json(ResponseMessage {
//                 message: "Erreur de connexion à la base de données".to_string(),
//             });
//         }
//     };

//     let result = conn.exec_drop(
//         "UPDATE musique SET id_playlist = :id_playlist WHERE id = :id_musique",
//         params! {
//             "id_playlist" => &params.id_playlist,
//             "id_musique" => &params.id_musique,
//         },
//     );

//     if let Ok(_) = result {
//         // Mise à jour du nombre de morceaux dans la playlist
//         conn.exec_drop(
//             "UPDATE playlist SET nombre_morceaux = nombre_morceaux + 1 WHERE id = :id_playlist",
//             params! {
//                 "id_playlist" => &params.id_playlist,
//             },
//         ).expect("Erreur lors de la mise à jour du nombre de morceaux");

//         HttpResponse::Ok().json(ResponseMessage {
//             message: "Musique ajoutée à la playlist avec succès".to_string(),
//         })
//     } else {
//         HttpResponse::InternalServerError().json(ResponseMessage {
//             message: "Erreur lors de l'ajout de la musique à la playlist".to_string(),
//         })
//     }
// }
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


// async fn remove_musique_from_playlist(params: web::Json<RemoveMusiqueFromPlaylistParams>) -> impl Responder {
//     let pool = get_connection();
//     let mut conn = match pool.get_conn() {
//         Ok(conn) => conn,
//         Err(_) => {
//             return HttpResponse::InternalServerError().json(ResponseMessage {
//                 message: "Erreur de connexion à la base de données".to_string(),
//             });
//         }
//     };

//     let result = conn.exec_drop(
//         "UPDATE musique SET id_playlist = NULL WHERE id = :id_musique AND id_playlist = :id_playlist",
//         params! {
//             "id_musique" => &params.id_musique,
//             "id_playlist" => &params.id_playlist,
//         },
//     );

//     if let Ok(_) = result {
//         // Mise à jour du nombre de morceaux dans la playlist
//         conn.exec_drop(
//             "UPDATE playlist SET nombre_morceaux = nombre_morceaux - 1 WHERE id = :id_playlist",
//             params! {
//                 "id_playlist" => &params.id_playlist,
//             },
//         ).expect("Erreur lors de la mise à jour du nombre de morceaux");

//         HttpResponse::Ok().json(ResponseMessage {
//             message: "Musique supprimée de la playlist avec succès".to_string(),
//         })
//     } else {
//         HttpResponse::InternalServerError().json(ResponseMessage {
//             message: "Erreur lors de la suppression de la musique de la playlist".to_string(),
//         })
//     }
// }

// #[derive(Serialize)]
// struct Playlist {
//     id: u32,
//     nom_playlist: String,
//     id_createur: u32,
//     nombre_morceaux: u32,
// }

// // Fonction pour récupérer toutes les playlists
// async fn get_all_playlists() -> impl Responder {
//     let pool = get_connection(); // Fonction qui récupère la connexion MySQL
//     let mut conn = pool.get_conn().expect("Failed to get connection");

//     // Requête SQL pour obtenir toutes les playlists
//     let playlists: Vec<Playlist> = conn
//         .query_map(
//             "SELECT id, nom_playlist, id_createur, nombre_morceaux FROM playlist",
//             |(id, nom_playlist, id_createur, nombre_morceaux)| Playlist {
//                 id,
//                 nom_playlist,
//                 id_createur,
//                 nombre_morceaux,
//             },
//         )
//         .expect("Failed to fetch playlists");

//     // Retourne la réponse JSON contenant toutes les playlists
//     HttpResponse::Ok().json(playlists)
// }

// async fn get_playlist(id: web::Path<u32>) -> impl Responder {
//     let pool = get_connection();  // Fonction qui récupère la connexion à la base de données MySQL
//     let mut conn = match pool.get_conn() {
//         Ok(conn) => conn,
//         Err(_) => {
//             return HttpResponse::InternalServerError().json(ResponseMessage {
//                 message: "Erreur de connexion à la base de données".to_string(),
//             });
//         }
//     };

//     // Requête SQL pour obtenir la playlist avec l'ID spécifié
//     let result: Option<(u32, String, u32, u32)> = conn.exec_first(
//         "SELECT id, nom_playlist, id_createur, nombre_morceaux FROM playlist WHERE id = :id",
//         params! {
//             "id" => id.into_inner(), // Le paramètre est passé correctement ici
//         },
//     ).expect("Erreur lors de l'exécution de la requête");

//     // Convertir le résultat en Playlist si trouvé
//     match result {
//         Some((id, nom_playlist, id_createur, nombre_morceaux)) => {
//             let playlist = Playlist {
//                 id,
//                 nom_playlist,
//                 id_createur,
//                 nombre_morceaux,
//             };
//             HttpResponse::Ok().json(playlist)
//         },
//         None => HttpResponse::NotFound().json(ResponseMessage {
//             message: "Playlist non trouvée".to_string(),
//         }),
//     }
// }

// async fn add_user(user: web::Json<User>) -> impl Responder {
//     let pool = get_connection();
//     let mut conn = match pool.get_conn() {
//         Ok(conn) => conn,
//         Err(_) => {
//             return HttpResponse::InternalServerError().json(ResponseMessage {
//                 message: "Erreur de connexion à la base de données".to_string(),
//             });
//         }
//     };

//     let result = conn.exec_drop(
//         "INSERT INTO utilisateur (nom_utilisateur, mots_passe) VALUES (:nom_utilisateur, :mots_passe)",
//         params! {
//             "nom_utilisateur" => &user.nom_utilisateur,
//             "mots_passe" => &user.mots_passe,
//         },
//     );

//     match result {
//         Ok(_) => HttpResponse::Ok().json(ResponseMessage {
//             message: format!("L'utilisateur '{}' a été ajouté avec succès", &user.nom_utilisateur),
//         }),
//         Err(_) => HttpResponse::InternalServerError().json(ResponseMessage {
//             message: "Erreur lors de l'ajout de l'utilisateur".to_string(),
//         }),
//     }
// }

// async fn connexion_user(user: web::Json<User>) -> impl Responder {
//     let pool = get_connection();
//     let mut conn = match pool.get_conn() {
//         Ok(conn) => conn,
//         Err(_) => {
//             return HttpResponse::InternalServerError().json(ResponseMessage {
//                 message: "Erreur de connexion à la base de données".to_string(),
//             });
//         }
//     };

//     // Utilisation de exec_map pour récupérer l'utilisateur
//     let result: Result<Vec<(i32, String, String)>, _> = conn.exec_map(
//         "SELECT id, nom_utilisateur, mots_passe FROM utilisateur WHERE nom_utilisateur = :nom_utilisateur AND mots_passe = :mots_passe",
//         params! {
//             "nom_utilisateur" => &user.nom_utilisateur,
//             "mots_passe" => &user.mots_passe,
//         },
//         |(id, nom_utilisateur, mots_passe)| (id, nom_utilisateur, mots_passe),
//     );

//     match result {
//         Ok(users) if !users.is_empty() => {
//             // L'utilisateur existe
//             HttpResponse::Ok().json(ResponseMessage {
//                 message: "Connexion réussie".to_string(),
//             })
//         },
//         Ok(_) => {
//             // Aucun utilisateur trouvé
//             HttpResponse::Ok().json(ResponseMessage {
//                 message: "Connexion échouée".to_string(),
//             })
//         },
//         Err(_) => {
//             // Erreur lors de la requête
//             HttpResponse::InternalServerError().json(ResponseMessage {
//                 message: "Erreur lors de la requête".to_string(),
//             })
//         },
//     }
// }



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server_handle = start_server();
    server_handle.await?;
    Ok(())
}

fn start_server() -> actix_web::dev::Server {
    HttpServer::new(|| {
        App::new()
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
    .bind("127.0.0.1:8080")
    .expect("Failed to bind to address")
    .run()
}







