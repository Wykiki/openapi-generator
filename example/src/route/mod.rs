use actix_web::{get, web, App, HttpServer, Responder};

mod pet;

pub use pet::get_pets;

#[get("/{id}/{name}")]
pub async fn index(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}
