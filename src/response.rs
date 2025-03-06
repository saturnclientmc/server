#[derive(Debug, Clone)]
pub enum Response {
    Pong,
    Error(Error),
}

impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Response::Pong => write!(f, "Pong"),
            Response::Error(error) => write!(f, "!{}", error),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    InvalidRequest,
    InvalidMethod,
    InvalidParameter,
    ParameterNotFound,
    InvalidSessionToken,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidRequest => write!(f, "Invalid request"),
            Error::InvalidMethod => write!(f, "Invalid method"),
            Error::InvalidParameter => write!(f, "Invalid parameter"),
            Error::ParameterNotFound => write!(f, "Parameter not found"),
            Error::InvalidSessionToken => write!(f, "Invalid session token"),
        }
    }
}
