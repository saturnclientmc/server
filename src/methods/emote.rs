use crate::response::{Response, Result};

use super::Session;

pub fn emote(session: &Session, emote: String, notify: Vec<&str>) -> Result {
    if notify.len() > 0 {
        session.notify(
            &notify,
            &format!("emote@uuid={}@name={emote}", session.local_player.id),
        )?;
    }

    Ok(Response::Success)
}
