use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpResponse};

use super::route;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(route::get_pets);
}
