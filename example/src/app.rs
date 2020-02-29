use actix_web::web;

use super::route;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(route::get_pets)
        .service(route::get_pet)
        .service(route::add_pet);
}
