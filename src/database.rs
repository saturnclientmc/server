pub struct Player {
    pub uuid: String,
    pub cloaks: Vec<String>,
    pub cloak: String,
}

pub struct Database {
    pub players: mongodb::sync::Collection<Player>,
}

impl Database {
    pub fn new(client: &mongodb::sync::Client) -> Self {
        let db = client.database("saturn");
        Self {
            players: db.collection("players"),
        }
    }
}
