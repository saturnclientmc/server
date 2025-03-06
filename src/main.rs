use std::{
    io::{BufRead, BufReader, Write},
    net::TcpListener,
    sync::Arc,
};

pub mod database;
pub mod methods;
pub mod parser;
pub mod response;

fn main() -> std::io::Result<()> {
    let client = mongodb::sync::Client::with_uri_str("mongodb://admin:admin@localhost/").unwrap();
    let database = Arc::new(database::Database::new(&client));

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        println!("New connection");
        let database = Arc::clone(&database);

        std::thread::spawn({
            move || {
                let mut stream = stream.unwrap();
                let mut reader = BufReader::new(stream.try_clone().unwrap());
                let session = methods::Session::new(&mut reader, database).unwrap();

                loop {
                    let mut request_string = String::new();
                    match reader.read_line(&mut request_string).unwrap() {
                        0 => break,
                        _ => {
                            let (method, params) = parser::parse(&request_string).unwrap();
                            let response = session.handle_request(&method, &params);
                            stream.write_all(response.to_string().as_bytes()).unwrap();
                        }
                    };
                }
            }
        });
    }

    Ok(())
}
