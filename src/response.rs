pub type Result<T = Response> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum PlayerResponse {
    Player {
        uuid: String,
        cloaks: Vec<String>,
        cloak: String,
        hats: Vec<String>,
        hat: String,
    },
    NonSaturnPlayer(String),
}
impl std::fmt::Display for PlayerResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayerResponse::Player {
                uuid,
                cloaks,
                cloak,
                hats,
                hat,
            } => write!(
                f,
                "player@cloak={cloak}@uuid={uuid}@cloaks={}@hats={}@hat={hat}@saturn=true",
                cloaks.join("$"),
                hats.join("$"),
            ),
            PlayerResponse::NonSaturnPlayer(uuid) => write!(f, "@saturn=false@uuid={uuid}"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Response {
    Pong,
    Success,
    Player(PlayerResponse),
    Players(Vec<PlayerResponse>),
}

impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Response::Pong => write!(f, "Pong"),
            Response::Success => write!(f, "Success"),
            Response::Player(player) => write!(f, "{player}"),
            Response::Players(players) => {
                write!(
                    f,
                    "{}",
                    players
                        .iter()
                        .map(|p| format!("{p}"))
                        .collect::<Vec<String>>()
                        .join("\n")
                )
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    InvalidRequest(String),
    InvalidMethod(String),
    InvalidParameter { param: String, reason: String },
    ParameterNotFound(String),
    InvalidSession(String),
    InvalidHandshake(String),
    DatabaseError(String),
    NetworkError(String),
    Timeout(String),
    AuthenticationError(String),
    ValidationError(String),
    EncryptionError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidRequest(msg) => write!(f, "Invalid request: {}", msg),
            Error::InvalidMethod(method) => write!(f, "Invalid method: {}", method),
            Error::InvalidParameter { param, reason } => write!(f, "Invalid parameter '{}': {}", param, reason),
            Error::ParameterNotFound(param) => write!(f, "Required parameter not found: {}", param),
            Error::InvalidSession(details) => write!(f, "Invalid session: {}", details),
            Error::InvalidHandshake(details) => write!(f, "Handshake failed: {}", details),
            Error::DatabaseError(details) => write!(f, "Database error: {}", details),
            Error::NetworkError(details) => write!(f, "Network error: {}", details),
            Error::Timeout(operation) => write!(f, "Operation timed out: {}", operation),
            Error::AuthenticationError(details) => write!(f, "Authentication failed: {}", details),
            Error::ValidationError(details) => write!(f, "Validation failed: {}", details),
            Error::EncryptionError(details) => write!(f, "Encryption error: {}", details),
        }
    }
}

impl From<mongodb::error::Error> for Error {
    fn from(e: mongodb::error::Error) -> Self {
        println!("{e}");
        Error::DatabaseError(e.to_string())
    }
}

impl std::error::Error for Error {}
