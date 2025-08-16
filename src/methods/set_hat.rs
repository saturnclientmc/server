use mongodb::bson::doc;

use crate::response::{Error, Response, Result};

use super::{player, Session};

pub fn set_hat(session: &Session, hat: String, notify: Vec<&str>) -> Result {
    match player::player(session, session.local_player.id.clone())? {
        Response::Player(crate::response::PlayerResponse::Player { hats, .. }) => {
            if !hats.contains(&hat) && !hat.is_empty() {
                return Err(Error::ValidationError(format!(
                    "Player does not own hat: {}",
                    hat
                )));
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
                            "hat": hat.clone(),
                        },
                    },
                )
                .run()
                .map_err(|e| Error::DatabaseError(format!("Failed to update hat: {}", e)))?;

            if notify.len() > 0 {
                session.notify(
                    &notify,
                    &format!("update_hat@uuid={}@hat={hat}", session.local_player.id),
                )?;
            }

            Ok(Response::Success)
        }
        _ => Err(Error::ValidationError(
            "Invalid player data received".to_string(),
        )),
    }
}
