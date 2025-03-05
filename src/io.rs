use mongodb::sync::{Client, Collection};
use serde::{Deserialize, Serialize};

pub enum Response {
    Pong,
}

impl Response {
    pub fn to_string(&self) -> String {
        match self {
            Response::Pong => "pong".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub uuid: String,
    pub cloaks: Vec<String>,
}

pub struct Database {
    pub players: Collection<Player>,
}

impl Database {
    pub fn new(client: &Client) -> Self {
        let db = client.database("saturn");
        Self {
            players: db.collection("players"),
        }
    }
}
