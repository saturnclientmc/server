mod buy;
pub mod player;
mod set_cloak;
mod set_hat;

use std::{
    collections::HashMap,
    sync::{mpsc, Arc},
};

use serde::{Deserialize, Serialize};

use crate::{
    encryption::ETcp,
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
        mut stream: ETcp,
        database: Arc<crate::database::Database>,
    ) -> Result<(Self, Response)> {
        let (token_send, token_recv) =
            mpsc::channel::<std::result::Result<String, crate::response::Error>>();

        std::thread::spawn(move || match stream.read() {
            Ok(Some(session_token)) => token_send.send(Ok(session_token)),
            _ => token_send.send(Err(crate::response::Error::InvalidHandshake(
                "Failed to read session token".to_string(),
            ))),
        });

        match token_recv.recv_timeout(std::time::Duration::from_secs(20)) {
            Ok(Ok(session_token)) => {
                // Validate session id
                let response = minreq::get("https://api.minecraftservices.com/minecraft/profile")
                    .with_header("Authorization", &format!("Bearer {session_token}"))
                    .send()
                    .map_err(|_| {
                        crate::response::Error::InvalidSession(
                            "Failed to validate session".to_string(),
                        )
                    })?;

                // If the session is invalid, return an error
                if response.status_code != 200 {
                    return Err(crate::response::Error::InvalidSession(format!(
                        "Invalid session status code: {}",
                        response.status_code
                    )));
                }

                // Parse the player data
                let local_player: LocalPlayer = response.json().map_err(|_| {
                    crate::response::Error::InvalidSession(
                        "Failed to parse player data".to_string(),
                    )
                })?;

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
            Err(mpsc::RecvTimeoutError::Timeout) => Err(crate::response::Error::Timeout(
                "Session handshake timed out".to_string(),
            )),
            _ => Err(crate::response::Error::InvalidHandshake(
                "Failed to receive handshake response".to_string(),
            )),
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
                let uuids = params.parse_param::<String>("uuids")?;
                if uuids.is_empty() {
                    return Err(crate::response::Error::InvalidParameter {
                        param: "uuids".to_string(),
                        reason: "UUIDs list cannot be empty".to_string(),
                    });
                }
                for uuid in uuids.split("$") {
                    if uuid.is_empty() {
                        return Err(crate::response::Error::InvalidParameter {
                            param: "uuids".to_string(),
                            reason: "UUID cannot be empty".to_string(),
                        });
                    }
                    match player::player(self, uuid.to_string())? {
                        Response::Player(p) => players.push(p),
                        _ => {}
                    }
                }
                Ok(Response::Players(players))
            }

            "buy_cloak" => buy::buy_cloak(self, params.parse_param("cloak")?),

            _ => Err(crate::response::Error::InvalidMethod(method.to_string())),
        }
    }
}
