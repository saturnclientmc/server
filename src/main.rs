use std::{
    net::{TcpListener, TcpStream},
    sync::Arc,
    time::{Duration, Instant},
};

use openssl::{pkey::Private, rsa::Rsa};

use crate::methods::SocketMap;

pub mod cosmetics;
pub mod database;
pub mod encryption;
pub mod methods;
pub mod parser;
pub mod response;

fn main() -> Result<(), response::Error> {
    let rsa = Rsa::generate(2048).map_err(|e| {
        response::Error::EncryptionError(format!("Failed to generate RSA keys: {}", e))
    })?;

    let client =
        mongodb::sync::Client::with_uri_str("mongodb://admin:admin@10.7.1.21/").map_err(|e| {
            response::Error::DatabaseError(format!("Failed to connect to MongoDB: {}", e))
        })?;
    let database = Arc::new(database::Database::new(&client));
    let sockets = SocketMap::default();

    let listener = TcpListener::bind("0.0.0.0:8080").map_err(|e| {
        response::Error::NetworkError(format!("Failed to bind to port 8080: {}", e))
    })?;

    println!("Server listening on port 8080");

    for stream_result in listener.incoming() {
        let stream = match stream_result {
            Ok(stream) => stream,
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
                continue;
            }
        };

        println!("New connection");
        let database = Arc::clone(&database);
        let sockets = Arc::clone(&sockets);
        let rsa = rsa.clone();

        std::thread::spawn(move || {
            if let Err(e) = handle_client(stream, rsa, database, sockets) {
                eprintln!("Client error: {}", e);
            }
        });
    }

    Ok(())
}

fn handle_client(
    stream: TcpStream,
    rsa: Rsa<Private>,
    database: Arc<database::Database>,
    sockets: SocketMap,
) -> Result<(), response::Error> {
    let mut stream = encryption::handshake(stream, rsa)?;
    let mut last_activity = Instant::now();

    match methods::Session::new(stream.try_clone()?, database, sockets) {
        Ok((session, res)) => {
            stream.send(res)?;

            loop {
                // Check for inactivity
                if last_activity.elapsed() > Duration::from_secs(60) {
                    println!(
                        "[MOJANG] {} inactive for too long",
                        session.local_player.name
                    );
                    methods::player::logout(&session)?;
                    println!("[MOJANG] {} went offline", session.local_player.name);
                    break;
                }

                match stream.read()? {
                    None => {
                        // Client disconnected
                        println!("[MOJANG] {} disconnected", session.local_player.name);
                        methods::player::logout(&session)?;
                        println!("[MOJANG] {} went offline", session.local_player.name);
                        break;
                    }
                    Some(request_string) => {
                        last_activity = Instant::now();
                        let (method, params) = parser::parse(&request_string)?;
                        let response = session.handle_request(&method, &params);

                        stream.send(match response {
                            Ok(response) => response.to_string(),
                            Err(e) => format!("!{e}"),
                        })?;
                    }
                }
            }
        }
        Err(e) => {
            stream.send(format!("!{e}"))?;
            println!("Disconnected");
            stream.close()?;
        }
    }

    Ok(())
}
