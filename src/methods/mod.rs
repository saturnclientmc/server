use std::{collections::HashMap, sync::Arc};

use crate::io::{Database, Response};

pub fn handle_request(
    database: Arc<Database>,
    method: String,
    params: HashMap<String, String>,
) -> Result<Response, String> {
    match method.as_str() {
        "ping" => Ok(Response::Pong),
        _ => Err("Invalid request: ".to_string()),
    }
}
