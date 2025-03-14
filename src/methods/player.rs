use mongodb::bson::doc;

use crate::response::{Response, Result};

use super::Session;

pub fn player(session: &Session, uuid: String) -> Result {
    let player = session
        .database
        .players
        .find(doc! {
            "uuid": uuid.clone(),
        })
        .run()?
        .deserialize_current()
        .unwrap();

    Ok(Response::Player {
        cloak: player.cloak,
        uuid,
    })
}
