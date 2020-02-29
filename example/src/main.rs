use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use std::collections::HashMap;
use std::sync::RwLock;

mod app;
mod model;
mod route;

use app::config;
use model::Pet;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let pets_db: web::Data<RwLock<HashMap<String, Pet>>> =
        web::Data::new(RwLock::new(HashMap::new()));
    HttpServer::new(move || {
        App::new()
            .app_data(pets_db.clone())
            .wrap(
                Cors::new()
                    .allowed_origin("All")
                    .send_wildcard()
                    // .allowed_methods(vec!["GET", "POST"])
                    // .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    // .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600)
                    .finish(),
            )
            .configure(config)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
