use mongodb::bson::doc;

use crate::{methods::Session, response::Result};

pub fn buy_cloak(session: &Session, cloak: String) -> Result {
    let filter = doc! {
        "uuid": session.local_player.id.clone()
    };

    if let Some(doc) = session.database.players.find_one(filter.clone()).run()? {
        if doc.coins >= 100 {
            session
                .database
                .players
                .update_one(
                    filter,
                    doc! { "$inc": { "points": -100 }, "$push": { "cloaks": cloak } },
                )
                .run()?;
            Ok(crate::response::Response::Success)
        } else {
            Err(crate::response::Error::TransactionError(
                "Not enough coins".to_string(),
            ))
        }
    } else {
        Err(crate::response::Error::DatabaseError(
            "Player not found".to_string(),
        ))
    }
}
