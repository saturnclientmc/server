use mongodb::bson::doc;

use crate::{
    database::Player,
    response::{Response, Result},
};

use super::Session;

pub fn player(session: &Session, uuid: String) -> Result {
    // Try to find the player with the given UUID
    let mut cursor = session
        .database
        .players
        .find(doc! { "uuid": uuid.clone() })
        .run()?;

    // Check if there's a document in the cursor
    Ok(match cursor.next() {
        Some(Ok(player)) => {
            if player.online {
                Response::Player(crate::response::PlayerResponse::Player {
                    cloak: player.cloak,
                    uuid,
                    cloaks: player.cloaks,
                    hats: player.hats,
                    hat: player.hat,
                })
            } else {
                crate::response::Response::Player(crate::response::PlayerResponse::NonSaturnPlayer(
                    uuid,
                ))
            }
        }

        _ => crate::response::Response::Player(crate::response::PlayerResponse::NonSaturnPlayer(
            uuid,
        )),
    })
}

pub fn create(session: &Session) -> Result {
    let uuid = session.local_player.id.clone();

    let player = Player {
        uuid: uuid.clone(),
        cloaks: Vec::new(),
        cloak: "".to_string(),
        hats: Vec::new(),
        hat: String::new(),
        online: true,
    };

    session.database.players.insert_one(player.clone()).run()?;

    Ok(Response::Player(crate::response::PlayerResponse::Player {
        cloak: player.cloak,
        uuid,
        cloaks: player.cloaks,
        hats: player.hats,
        hat: player.hat,
    }))
}

pub fn login(session: &Session) -> Result {
    let uuid = session.local_player.id.clone();

    // Try to find the player with the given UUID
    let mut cursor = session
        .database
        .players
        .find(doc! { "uuid": uuid.clone() })
        .run()?;

    // Check if there's a document in the cursor
    match cursor.next() {
        Some(Ok(mut player)) => {
            // Update player's online status
            player.online = true;
            session
                .database
                .players
                .update_one(
                    doc! { "uuid": uuid.clone() },
                    doc! { "$set": { "online": true } },
                )
                .run()?;

            // Player exists, deserialize and return their data
            Ok(Response::Player(crate::response::PlayerResponse::Player {
                cloak: player.cloak,
                uuid,
                cloaks: player.cloaks,
                hats: player.hats,
                hat: player.hat,
            }))
        }

        // No document found or deserialization error
        _ => create(session),
    }
}

pub fn logout(session: &Session) -> Result {
    let uuid = session.local_player.id.clone();

    // Update player's online status to false
    session
        .database
        .players
        .update_one(
            doc! { "uuid": uuid.clone() },
            doc! { "$set": { "online": false } },
        )
        .run()?;

    Ok(Response::Player(
        crate::response::PlayerResponse::NonSaturnPlayer(uuid),
    ))
}
