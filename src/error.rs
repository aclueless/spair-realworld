//! Error type for error handling

/// Define all possible errors
#[derive(thiserror::Error, Clone, Debug)]
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
    UnprocessableEntity(types::ErrorInfo),

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

impl From<spair::ResponsedError<types::ErrorInfo>> for Error {
    fn from(e: spair::ResponsedError<types::ErrorInfo>) -> Self {
        match e {
            spair::ResponsedError::FetchError(spair::FetchError::DeserializeJsonError(_)) => {
                Self::DeserializeError
            }
            spair::ResponsedError::ApiError(e) => {
                match (e.data, e.status) {
                    (_, spair::StatusCode::UNAUTHORIZED) => Self::Unauthorized,
                    (_, spair::StatusCode::FORBIDDEN) => Self::Forbidden,
                    (_, spair::StatusCode::NOT_FOUND) => Self::NotFound,
                    (_, spair::StatusCode::INTERNAL_SERVER_ERROR) => Self::InternalServerError,
                    (Ok(e), _) => Self::UnprocessableEntity(e),
                    (Err(spair::FetchError::DeserializeJsonError(_)), _) => Self::DeserializeError,
                    _ => Self::RequestError,
                }
            }
            _ => Self::RequestError,
        }
    }
}
