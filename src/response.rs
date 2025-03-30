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
    NonSaturnPlayer,
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
            PlayerResponse::NonSaturnPlayer => write!(f, "@saturn=false"),
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
    InvalidRequest,
    InvalidMethod,
    InvalidParameter,
    ParameterNotFound,
    InvalidSession,
    InvalidHandshake,
    DatabaseError,
    SomethingWentWrong,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidRequest => write!(f, "Invalid request"),
            Error::InvalidMethod => write!(f, "Invalid method"),
            Error::InvalidParameter => write!(f, "Invalid parameter"),
            Error::ParameterNotFound => write!(f, "Parameter not found"),
            Error::InvalidSession => write!(f, "Invalid session"),
            Error::InvalidHandshake => write!(f, "Invalid handshake"),
            Error::DatabaseError => write!(f, "Database error"),
            Error::SomethingWentWrong => write!(f, "Something went wrong"),
        }
    }
}

impl From<mongodb::error::Error> for Error {
    fn from(e: mongodb::error::Error) -> Self {
        println!("{e}");
        Error::DatabaseError
    }
}

impl std::error::Error for Error {}
