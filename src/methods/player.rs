use mongodb::bson::doc;

use crate::{
    database::Player,
    response::{Response, Result},
};

use super::Session;

pub fn player(session: &Session, uuid: String) -> Result {
    // First, try to find the existing player
    match session
        .database
        .players
        .find(doc! { "uuid": uuid.clone() })
        .run()?
        .deserialize_current()
    {
        // Player exists, return their data
        Ok(player) => Ok(Response::Player {
            cloak: player.cloak,
            uuid,
            cloaks: player.cloaks,
            hats: player.hats,
            hat: player.hat,
        }),

        // Player doesn't exist, create a new player
        Err(_) => {
            let player = Player {
                uuid: uuid.clone(),
                cloaks: Vec::new(),
                cloak: "".to_string(),
                hats: Vec::new(),
                hat: String::new(),
            };

            // Insert the new player
            session.database.players.insert_one(player.clone()).run()?;

            // Return the newly created player's data
            Ok(Response::Player {
                cloak: player.cloak,
                uuid,
                cloaks: player.cloaks,
                hats: player.hats,
                hat: player.hat,
            })
        }
    }
}

pub fn create(session: &Session) -> Result {
    let uuid = session.local_player.id.clone();

    let player = Player {
        uuid: uuid.clone(),
        cloaks: Vec::new(),
        cloak: "".to_string(),
        hats: Vec::new(),
        hat: String::new(),
    };

    session.database.players.insert_one(player.clone()).run()?;

    Ok(Response::Player {
        cloak: player.cloak,
        uuid,
        cloaks: player.cloaks,
        hats: player.hats,
        hat: player.hat,
    })
}
