use mongodb::bson::doc;

use crate::response::{Error, Response, Result};

use super::{player, Session};

pub fn set_cloak(session: &Session, cloak: String) -> Result {
    match player::player(session, session.local_player.id.clone())? {
        Response::Player(crate::response::PlayerResponse::Player { cloaks, .. }) => {
            if !cloaks.contains(&cloak) && !cloak.is_empty() {
                return Err(Error::ValidationError(
                    format!("Player does not own cloak: {}", cloak)
                ));
            }

            session
                .database
                .players
                .update_one(
                    doc! {
                        "uuid": session.local_player.id.clone(),
                    },
                    doc! {
                        "$set": {
                            "cloak": cloak.clone(),
                        },
                    },
                )
                .run()
                .map_err(|e| Error::DatabaseError(
                    format!("Failed to update cloak: {}", e)
                ))?;

            Ok(Response::Success)
        }
        _ => Err(Error::ValidationError("Invalid player data received".to_string())),
    }
}
