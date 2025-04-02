pub mod player;
mod set_cloak;
mod set_hat;

use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    net::TcpStream,
    sync::{mpsc, Arc},
};

use serde::{Deserialize, Serialize};

use crate::{
    parser::ParamMap,
    response::{PlayerResponse, Response, Result},
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
        let (token_send, token_recv) =
            mpsc::channel::<std::result::Result<String, crate::response::Error>>();

        let reader_clone = reader
            .get_ref()
            .try_clone()
            .map_err(|_| crate::response::Error::InvalidHandshake)?;
        std::thread::spawn(move || {
            let mut reader = BufReader::new(reader_clone);
            let mut session_token = String::new();
            if reader.read_line(&mut session_token).is_ok() {
                token_send.send(Ok(session_token))
            } else {
                token_send.send(Err(crate::response::Error::InvalidHandshake))
            }
        });

        match token_recv.recv_timeout(std::time::Duration::from_secs(20)) {
            Ok(Ok(session_token)) => {
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

                println!("[MOJANG] {} successfully logged on", &local_player.name);

                let session = Self {
                    session_token,
                    database,
                    local_player,
                };

                // Capture the player
                match player::login(&session) {
                    Ok(player) => Ok((session, player)),
                    Err(e) => Err(e),
                }
            }
            Err(mpsc::RecvTimeoutError::Timeout) => Err(crate::response::Error::Timeout),
            _ => Err(crate::response::Error::InvalidHandshake),
        }
    }

    pub fn handle_request(&self, method: &str, params: &HashMap<String, String>) -> Result {
        match method {
            "ping" => Ok(Response::Pong),

            "set_cloak" => set_cloak::set_cloak(self, params.parse_param("cloak")?),

            "set_hat" => set_hat::set_hat(self, params.parse_param("hat")?),

            "player" => player::player(self, params.parse_param("uuid")?),

            "players" => {
                let mut players: Vec<PlayerResponse> = Vec::new();
                for uuid in params.parse_param::<String>("uuids")?.split("$") {
                    match player::player(self, uuid.to_string())? {
                        Response::Player(p) => {
                            players.push(p);
                        }
                        _ => {}
                    }
                }
                Ok(Response::Players(players))
            }

            _ => Err(crate::response::Error::InvalidRequest),
        }
    }
}
