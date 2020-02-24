use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Pet {
    name: String,
    tag: String,
    id: i64,
}

impl Pet {
    pub fn new<S: Into<String>>(name: S, tag: S, id: i64) -> Self {
        Pet {
            name: name.into(),
            tag: tag.into(),
            id: id,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewPet {
    name: String,
    tag: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    code: i64,
    message: String,
}
