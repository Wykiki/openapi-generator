use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pet {
    name: String,
    tag: String,
    id: String,
}

impl Pet {
    pub fn new<S: Into<String>>(name: S, tag: S, id: S) -> Self {
        Pet {
            name: name.into(),
            tag: tag.into(),
            id: id.into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewPet {
    pub name: String,
    pub tag: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    code: i64,
    message: String,
}
