use std::{
    net::TcpListener,
    sync::Arc,
    time::{Duration, Instant},
};

use openssl::rsa::Rsa;

pub mod database;
pub mod encryption;
pub mod methods;
pub mod parser;
pub mod response;

fn main() -> std::io::Result<()> {
    let rsa = Rsa::generate(2048).expect("Failed to generate RSA keys");

    let client = mongodb::sync::Client::with_uri_str("mongodb://admin:admin@localhost/").unwrap();
    let database = Arc::new(database::Database::new(&client));

    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();

    for stream in listener.incoming() {
        println!("New connection");
        let database = Arc::clone(&database);
        let rsa = rsa.clone();

        std::thread::spawn({
            move || {
                let mut stream = encryption::handshake(stream.unwrap(), rsa);
                let mut last_activity = Instant::now();

                match methods::Session::new(stream.clone(), database) {
                    Ok((session, res)) => {
                        stream.send(res);

                        loop {
                            // Check for inactivity
                            if last_activity.elapsed() > Duration::from_secs(60) {
                                println!(
                                    "[MOJANG] {} inactive for too long",
                                    session.local_player.name
                                );
                                methods::player::logout(&session).unwrap();
                                println!("[MOJANG] {} went offline", session.local_player.name);
                                break;
                            }

                            match stream.read() {
                                None => {
                                    // Client disconnected
                                    println!("[MOJANG] {} disconnected", session.local_player.name);
                                    methods::player::logout(&session).unwrap();
                                    println!("[MOJANG] {} went offline", session.local_player.name);
                                    break;
                                }
                                Some(request_string) => {
                                    last_activity = Instant::now();
                                    let (method, params) = parser::parse(&request_string).unwrap();
                                    let response = session.handle_request(&method, &params);

                                    stream.send(match response {
                                        Ok(response) => response.to_string(),
                                        Err(e) => format!("!{e}"),
                                    });
                                }
                            };
                        }
                    }
                    Err(e) => {
                        stream.send(format!("!{e}"));
                        println!("Disconnected");
                        stream.close();
                    }
                }
            }
        });
    }

    Ok(())
}
