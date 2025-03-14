use mongodb::bson::doc;

use crate::response::{Response, Result};

use super::Session;

pub fn set_cloak(session: &Session, cloak: String) -> Result {
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
}
