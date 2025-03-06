use std::{
    collections::HashMap,
    io::{self, BufRead, BufReader},
    net::TcpStream,
    sync::Arc,
};

pub struct Session {
    pub session_id: String,
    pub database: Arc<crate::database::Database>,
}

impl Session {
    pub fn new(
        reader: &mut BufReader<TcpStream>,
        database: Arc<crate::database::Database>,
    ) -> io::Result<Self> {
        let mut session_id = String::new();
        reader.read_line(&mut session_id)?;

        Ok(Self {
            session_id,
            database,
        })
    }

    pub fn handle_request(
        &self,
        method: &str,
        params: &HashMap<String, String>,
    ) -> crate::response::Response {
        match method {
            "ping" => crate::response::Response::Pong,

            _ => crate::response::Response::Error(crate::response::Error::InvalidRequest),
        }
    }
}
