use mongodb::bson::doc;

use crate::response::{Error, Response, Result};

use super::{player, Session};

pub fn set_cloak(session: &Session, cloak: String) -> Result {
    match player::player(session, session.local_player.id.clone())? {
        Response::Player(crate::response::PlayerResponse::Player { cloaks, .. }) => {
            if cloaks.contains(&cloak) || cloak.is_empty() {
                session
                    .database
                    .players
                    .update_one(
                        doc! {
                            "uuid": session.local_player.id.clone(),
                        },
                        doc! {
                            "$set": {
                                "cloak": cloak,
                            },
                        },
                    )
                    .run()?;
                Ok(Response::Success)
            } else {
                Err(Error::SomethingWentWrong)
            }
        }
        _ => Err(Error::SomethingWentWrong),
    }
}
