pub mod assets;
pub mod io;
pub mod methods;
pub mod parser;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

use io::Database;
use methods::handle_request;
use mongodb::sync::Client;

const MONGO_URI: &str = "mongodb://admin:admin@localhost/";

fn main() -> std::io::Result<()> {
    let client = Client::with_uri_str(MONGO_URI).unwrap();
    let database = Arc::new(Database::new(&client));
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    println!("Server listening on port 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn({
                    let database = Arc::clone(&database);
                    move || handle_client(stream, database)
                });
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }

    Ok(())
}

pub fn handle_client(mut stream: TcpStream, database: Arc<Database>) {
    let mut buffer = [0; 512];

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                let recv = String::from_utf8_lossy(&buffer[..n]).to_string();
                match parser::parse(&recv) {
                    Ok((method, params)) => {
                        match handle_request(Arc::clone(&database), method, params) {
                            Ok(response) => {
                                stream.write_all(response.to_string().as_bytes()).unwrap()
                            }
                            Err(e) => eprintln!("Error handling request: {}", e),
                        }
                    }
                    Err(e) => eprintln!("Error parsing request: {}", e),
                };
            }
            Err(e) => {
                eprintln!("Error reading from client: {}", e);
                break;
            }
        }
    }

    println!("Client disconnected");
}
