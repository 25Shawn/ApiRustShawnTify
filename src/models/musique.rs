use serde::Serialize;

#[derive(Serialize)]
pub struct Musique {
    pub id: i32,
    pub uuid: String,
    pub duree: String,
}