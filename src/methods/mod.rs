mod player;
mod set_cloak;

use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    net::TcpStream,
    sync::Arc,
};

use serde::{Deserialize, Serialize};

use crate::{
    parser::ParamMap,
    response::{Response, Result},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct LocalPlayer {
    pub id: String,
    pub name: String,
}

pub struct Session {
    pub session_token: String,
    pub database: Arc<crate::database::Database>,
    pub local_player: LocalPlayer,
}

impl Session {
    pub fn new(
        reader: &mut BufReader<TcpStream>,
        database: Arc<crate::database::Database>,
    ) -> Result<(Self, Response)> {
        // Read the session token
        let mut session_token = String::new();
        reader
            .read_line(&mut session_token)
            .map_err(|_| crate::response::Error::InvalidHandshake)?;

        // Validate session id
        let response = minreq::get("https://api.minecraftservices.com/minecraft/profile")
            .with_header("Authorization", &format!("Bearer {session_token}"))
            .send()
            .map_err(|_| crate::response::Error::InvalidSession)?;

        // If the session is invalid, return an error
        if response.status_code != 200 {
            return Err(crate::response::Error::InvalidSession);
        }

        // Parse the player data
        let local_player: LocalPlayer = response
            .json()
            .map_err(|_| crate::response::Error::InvalidSession)?;

        let uuid = local_player.id.clone();

        let session = Self {
            session_token,
            database,
            local_player,
        };

        let player = player::player(&session, uuid)?;

        Ok((session, player))
    }

    pub fn handle_request(&self, method: &str, params: &HashMap<String, String>) -> Result {
        match method {
            "ping" => Ok(Response::Pong),

            "set_cloak" => set_cloak::set_cloak(self, params.parse_param("cloak")?),

            "player" => player::player(self, params.parse_param("uuid")?),

            _ => Err(crate::response::Error::InvalidRequest),
        }
    }
}
