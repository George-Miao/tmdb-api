use thiserror::Error;

#[derive(Debug, Deserialize, Serialize, Error)]
#[error("Body error (status: {status_code}): {status_message}")]
pub struct ServerOtherBodyError {
    pub status_code: u16,
    pub status_message: String,
}

#[derive(Debug, Deserialize, Serialize, Error)]
#[error("Validation error: {}", errors.join(", "))]
pub struct ServerValidationBodyError {
    pub errors: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Error)]
#[serde(untagged)]
pub enum ServerBodyError {
    #[error("{0}")]
    Other(#[from] ServerOtherBodyError),
    #[error("{0}")]
    Validation(#[from] ServerValidationBodyError),
}

impl ServerBodyError {
    pub fn as_other_error(&self) -> Option<&ServerOtherBodyError> {
        match self {
            Self::Other(inner) => Some(inner),
            _ => None,
        }
    }

    pub fn as_validation_error(&self) -> Option<&ServerValidationBodyError> {
        match self {
            Self::Validation(inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Debug, Error)]
#[error("Server error(code: {code}): {body}")]
pub struct ServerError {
    pub code: u16,
    pub body: ServerBodyError,
}

#[cfg(feature = "commands")]
#[derive(Debug, Error)]
pub enum Error {
    #[error("Reqwest error: {0}")]
    Reqwest(reqwest::Error),

    #[error("{0}")]
    Server(ServerError),
}

#[cfg(feature = "commands")]
impl Error {
    pub fn as_reqwest_error(&self) -> Option<&reqwest::Error> {
        match self {
            Self::Reqwest(inner) => Some(inner),
            _ => None,
        }
    }

    pub fn is_reqwest_error(&self) -> bool {
        matches!(self, Self::Reqwest(_))
    }

    pub fn as_server_error(&self) -> Option<&ServerError> {
        match self {
            Self::Server(inner) => Some(inner),
            _ => None,
        }
    }

    pub fn is_server_error(&self) -> bool {
        matches!(self, Self::Server(_))
    }
}

#[cfg(feature = "commands")]
impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::Reqwest(err)
    }
}

#[cfg(feature = "commands")]
impl From<(reqwest::StatusCode, ServerBodyError)> for Error {
    fn from((code, body): (reqwest::StatusCode, ServerBodyError)) -> Self {
        Self::Server(ServerError {
            code: code.as_u16(),
            body,
        })
    }
}
