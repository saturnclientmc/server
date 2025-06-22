use mongodb::bson::doc;

use crate::{methods::Session, response::Result};

pub fn buy_cloak(session: &Session, cloak: String) -> Result {
    let filter = doc! {
        "uuid": session.local_player.id.clone(),
        "points": { "$gte": 100 }
    };

    if let Some(doc) = session.database.players.find_one(filter).run()? {
        println!("Found matching document: {:?}", doc);
        Ok(crate::response::Response::Success)
    } else {
        Err(crate::response::Error::TransactionError(
            "Not enough coins".to_string(),
        ))
    }
}
