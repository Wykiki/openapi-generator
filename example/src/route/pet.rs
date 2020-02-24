use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

use super::super::model::{Error, NewPet, Pet};

#[get("/pets")]
pub async fn get_pets() -> HttpResponse {
    HttpResponse::Ok().json(vec![Pet::new("Fido", "Dog", 1)])
}
