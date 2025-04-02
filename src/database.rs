use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Player {
    pub uuid: String,
    pub cloaks: Vec<String>,
    pub cloak: String,
    pub hats: Vec<String>,
    pub hat: String,
    pub online: bool,
}

pub struct Database {
    pub players: mongodb::sync::Collection<Player>,
}

impl Database {
    pub fn new(client: &mongodb::sync::Client) -> Self {
        let db = client.database("saturnclient");
        Self {
            players: db.collection("players"),
        }
    }
}
