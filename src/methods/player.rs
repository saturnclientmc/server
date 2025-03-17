use mongodb::bson::doc;

use crate::{
    database::Player,
    response::{Response, Result},
};

use super::Session;

pub fn player(session: &Session, uuid: String) -> Result {
    let player = session
        .database
        .players
        .find(doc! {
            "uuid": uuid.clone(),
        })
        .run()?
        .deserialize_current()?;

    Ok(Response::Player {
        cloak: player.cloak,
        uuid,
        cloaks: player.cloaks,
    })
}

pub fn create(session: &Session) -> Result {
    let uuid = session.local_player.id.clone();

    let player = Player {
        cloak: "".to_string(),
        cloaks: Vec::new(),
        uuid: uuid.clone(),
    };

    session.database.players.insert_one(player.clone()).run()?;

    Ok(Response::Player {
        cloak: player.cloak,
        uuid,
        cloaks: player.cloaks,
    })
}
