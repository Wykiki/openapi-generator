use std::collections::HashMap;
use std::sync::RwLock;

use actix_web::{get, post, web, HttpResponse};
use uuid::Uuid;

use super::super::model::{NewPet, Pet};

#[get("/pets")]
pub async fn get_pets(pets_db: web::Data<RwLock<HashMap<String, Pet>>>) -> HttpResponse {
    HttpResponse::Ok().json(
        pets_db
            .read()
            .expect("RwLock poisoning GET /pets")
            .values()
            .clone()
            .collect::<Vec<&Pet>>(),
    )
}

#[get("/pets/{id}")]
pub async fn get_pet(
    path: web::Path<String>,
    pets_db: web::Data<RwLock<HashMap<String, Pet>>>,
) -> HttpResponse {
    let pets = pets_db.read().expect("RwLock poisoning GET /pets/{id}");
    let pet = pets.get(&path.to_string());
    match pet {
        Some(pet) => HttpResponse::Ok().json(pet),
        None => HttpResponse::NotFound().finish(),
    }
}

#[post("/pets")]
pub async fn add_pet(
    new_pet: web::Json<NewPet>,
    pets_db: web::Data<RwLock<HashMap<String, Pet>>>,
) -> HttpResponse {
    let mut pets = pets_db.write().expect("RwLock poisoning POST /pets");
    let id = Uuid::new_v4().to_string();
    let pet = Pet::new(new_pet.name.clone(), new_pet.tag.clone(), id.clone());
    pets.insert(id.clone(), pet);
    HttpResponse::Ok().body(id)
}
