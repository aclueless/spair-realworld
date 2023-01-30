//! Error type for error handling

use types::ErrorInfo;
use thiserror::Error as ThisError;

/// Define all possible errors
#[derive(ThisError, Clone, Debug, PartialEq, Eq)]
pub enum Error {
    /// 401
    #[error("Unauthorized")]
    Unauthorized,

    /// 403
    #[error("Forbidden")]
    Forbidden,

    /// 404
    #[error("Not Found")]
    NotFound,

    /// 422
    #[error("Unprocessable Entity: {0:?}")]
    UnprocessableEntity(ErrorInfo),

    /// 500
    #[error("Internal Server Error")]
    InternalServerError,

    /// serde deserialize error
    #[error("Deserialize Error")]
    DeserializeError,

    /// request error
    #[error("Http Request Error")]
    RequestError,
}

impl Error {
    pub fn from_status_code(status: u16, reported_error: Error) -> Self {
        match status {
            401 => Error::Unauthorized,
            403 => Error::Forbidden,
            404 => Error::NotFound,
            500 => Error::InternalServerError,
            422 => reported_error,
            _ => Error::RequestError,
        }
    }
}
