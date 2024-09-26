
// use minimp3::{Decoder, Frame};
// use std::fs::File;
// use std::io::BufReader;
// use std::process::Command;
// use std::str;
// use std::path::Path;
// use magic::{Cookie, flags};

// async fn get_musics() -> impl Responder {
//     let file_path = "./src/musiques/JAAH_SLT_Tuff_(AUDIO).mp3";
    
//     // Appel système pour obtenir la durée du fichier MP3
//     let output = Command::new("ffprobe")
//         .arg("-v")
//         .arg("error")
//         .arg("-show_entries")
//         .arg("format=duration")
//         .arg("-of")
//         .arg("default=noprint_wrappers=1:nokey=1")
//         .arg(file_path)
//         .output()
//         .expect("Failed to execute command");

//     let duration_str = str::from_utf8(&output.stdout).expect("Invalid UTF-8 output");
//     let duration_sec: f64 = duration_str.trim().parse().expect("Failed to parse duration");
//     let duration_ms = (duration_sec * 1000.0) as u64;

//     println!("Duration: {} ms", duration_ms);

//     println!("Opening file");

//     // Connexion à la base de données
//     let pool = get_connection();
//     let mut conn = pool.get_conn().expect("Failed to get connection");

//     // Requête pour obtenir les musiques
//     let result: Vec<Musique> = conn
//         .query_map(
//             "SELECT id, nom, duree FROM musique",
//             |(id, nom, duree)| Musique { id, nom, duree },
//         )
//         .expect("Erreur lors de l'exécution de la requête");

//     // Réponse en JSON
//     HttpResponse::Ok().json(result)
// }

// // use actix_web::{web, App, HttpServer, HttpResponse, Responder};
// // use std::process::Command;
// // use std::str;

// async fn get_audio_duration() -> impl Responder {
//     let file_path = "./src/musiques/JAAH_SLT_Tuff_(AUDIO).mp3";

//     // Ouvrir le fichier
//     let file = File::open(file_path).expect("Failed to open file");
//     let mut decoder = Decoder::new(BufReader::new(file));

//     let mut total_duration = 0;

//     // Lire les frames pour calculer la durée totale
//     while let Ok(Frame { sample_rate, data, .. }) = decoder.next_frame() {
//         total_duration += data.len() as u64 * 1000 / sample_rate as u64;
//     }

//     // Retourner la durée en JSON
//     HttpResponse::Ok().json(serde_json::json!({
//         "file_path": file_path,
//         "duration_ms": total_duration,
//     }))
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .route("/duration", web::get().to(get_audio_duration))
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }


// // #[actix_web::main]
// // async fn main() -> std::io::Result<()> {
// //     dotenv().ok();
// //     HttpServer::new(|| {
// //         App::new()
// //             .route("/musiques", web::get().to(get_musics))
// //     })
// //     .bind("127.0.0.1:8080")?
// //     .run()
// //     .await
// // }

// use actix_web::{web, App, HttpResponse, HttpServer, Responder};
// use std::fs;
// use std::fs::File;

// use rodio::{Decoder, OutputStream, Sink};
// use std::fs::File;
// use std::io::BufReader;

// fn play_audio(file_path: &str) {
//     // Démarrer le flux audio
//     let (_stream, stream_handle) = OutputStream::try_default().unwrap();
//     let sink = Sink::try_new(&stream_handle).unwrap();

//     // Ouvrir le fichier audio
//     let file = File::open(file_path).expect("Failed to open audio file");
//     let source = Decoder::new(BufReader::new(file)).expect("Failed to decode audio");

//     // Lire l'audio
//     sink.append(source);
//     sink.sleep_until_end();
// }

// fn main() {
//     let audio_path = "./src/musiques/JAAH_SLT_Tuff_(AUDIO).mp3";
//     play_audio(audio_path);
// }

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

#[derive(Debug, FromRow, Serialize)]
struct Musique {
    id: u32,
    uuid: String,
    duree: u64,
}

#[derive(Deserialize)]
struct AddMusiqueParams {
    uuid: String,
}

#[derive(Serialize)]
struct ResponseMessage {
    message: String,
}

async fn add_musique(params: web::Json<AddMusiqueParams>) -> impl Responder {
    let directory = "./src/musiques/";
    let file_path = format!("{}{}", directory, &params.uuid);

    println!("File path: {}", file_path);
    let file = File::open(&file_path).expect("Failed to open file");

    // Obtenir la durée du fichier audio
    let total_duration = get_audio_duration(&file_path);

    let pool = get_connection();
    let mut conn = pool.get_conn().expect("Failed to get connection");

    // Insertion de la musique dans la base de données
    conn.exec_drop(
        "INSERT INTO musique (uuid, duree) VALUES (:uuid, :duree)",
        params! {
            "uuid" => &params.uuid,
            "duree" => total_duration.as_secs(),
        },
    ).expect("Failed to insert musique");

    HttpResponse::Ok().json(ResponseMessage { message: format!("Musique ajoutée avec succès: {}", &params.uuid) })
}

// Fonction pour obtenir la durée du fichier audio
fn get_audio_duration(file_path: &str) -> std::time::Duration {
    match mp3_duration::from_path(file_path) {
        Ok(duration) => {
            println!("Duration: {} seconds", duration.as_secs());
            duration
        }
        Err(e) => {
            println!("Failed to get duration: {}", e);
            std::time::Duration::new(0, 0)  // Retourne une durée de 0 en cas d'erreur
        }
    }
}

// Fonction pour obtenir toutes les musiques
async fn get_all_musiques() -> impl Responder {
    let musiques = web::block(|| getMusiques()).await;

    match musiques {
        Ok(musiques) => HttpResponse::Ok().json(musiques),
        Err(_) => HttpResponse::InternalServerError().json(ResponseMessage {
            message: "Erreur lors de la récupération des musiques".to_string(),
        }),
    }
}

// Fonction pour obtenir une musique par UUID
async fn get_musique(uuid: web::Path<String>) -> impl Responder {
    println!("UUID: {}", uuid);
    let musique = web::block(move || get_musique_inner(&uuid)).await;

    match musique {
        Ok(Some(musique)) => HttpResponse::Ok().json(musique),
        Ok(None) => HttpResponse::NotFound().json(ResponseMessage {
            message: "Musique non trouvée".to_string(),
        }),
        Err(_) => HttpResponse::InternalServerError().json(ResponseMessage {
            message: "Erreur lors de la récupération de la musique".to_string(),
        }),
    }
}

// Fonction interne pour récupérer une musique de la base de données
fn get_musique_inner(uuid: &str) -> Option<(i32, String, String)> {  // Utiliser String pour uuid et String pour duree
    let pool = get_connection();
    let mut conn = pool.get_conn().expect("Failed to get connection");

    println!("UUID: {}", uuid);

    // Utiliser exec_first pour obtenir un seul résultat
    let result: Option<(i32, Vec<u8>, String)> = conn.exec_first(
        "SELECT id, uuid, duree FROM musique WHERE uuid = ?",
        (uuid,),
    ).expect("Erreur lors de l'exécution de la requête");

    // Convertir les résultats dans le format souhaité
    let converted_result = result.map(|(id, uuid_bytes, duree)| {
        let uuid_string = String::from_utf8(uuid_bytes).unwrap_or_else(|_| "Invalid UUID".to_string());
        let duree_string = duree;  // Assurez-vous que duree est de type String
        (id, uuid_string, duree_string)
    });

    println!("Result: {:?}", converted_result);
    converted_result
} 

fn getMusiques() -> Vec<Musique> {
    let pool = get_connection();
    let mut conn = pool.get_conn().expect("Failed to get connection");

    let result: Vec<Musique> = conn
        .query_map(
            "SELECT id, uuid, duree FROM musique",
            |(id, uuid, duree)| Musique { id, uuid, duree },
        )
        .expect("Erreur lors de l'exécution de la requête");

    result
}

// Fonction pour supprimer une musique par UUID
async fn supprimer_musique(uuid: web::Path<String>) -> impl Responder {
    let pool = get_connection();
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ResponseMessage {
                message: "Erreur de connexion à la base de données".to_string(),
            });
        }
    };

    // Tentative de suppression de la musique avec l'UUID fourni
    let result = conn.exec_drop(
        "DELETE FROM musique WHERE uuid = ?",
        (uuid.into_inner(),),
    );

    match result {
        Ok(_) => HttpResponse::Ok().json(ResponseMessage {
            message: "Musique supprimée avec succès".to_string(),
        }),
        Err(_) => HttpResponse::InternalServerError().json(ResponseMessage {
            message: "Erreur lors de la suppression de la musique".to_string(),
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
    HttpServer::new(|| {
        App::new()
            .route("/addMusique", web::post().to(add_musique))
            .route("/musiques", web::get().to(get_all_musiques)) 
            .route("/musique/{uuid}", web::get().to(get_musique))
            .route("/supprimer/{uuid}", web::delete().to(supprimer_musique))
    })
    .bind("127.0.0.1:8080")
    .expect("Failed to bind to address")
    .run()
}







