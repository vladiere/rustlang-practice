use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use tracing::debug;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Serialize, Clone, Debug, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    // LoginFail,

    // -- Auth errors.
    // AuthFailNoAuthTokenCookie,
    // AuthFailTokenWrongFormat,
    // AuthFailCtxNotInRequestExt,
    // -- Model errors.
    PostDeleteFailIdNotFound { id: i64 },
    PostCreateError,
    PostFindByIdNotFound { id: i64 },

    // -- Config errors.
    ConfigMissingEnv(&'static str),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        debug!("{:<12} - {self:?}", "INTO_RESPONSE");

        // Create a placeholder Axum response.
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Insert the Error into the response.
        response.extensions_mut().insert(self);

        response
    }
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        #[allow(unreachable_patterns)]
        match self {
            // ---- Login.
            // Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),
            // ---- Auth.
            // Self::AuthFailNoAuthTokenCookie
            // | Self::AuthFailTokenWrongFormat
            // | Self::AuthFailCtxNotInRequestExt => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),
            // ---- Model.
            Self::PostDeleteFailIdNotFound { .. } => {
                (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS)
            }
            Self::PostCreateError => (StatusCode::NOT_ACCEPTABLE, ClientError::INVALID_PARAMS),
            Self::PostFindByIdNotFound { .. } => {
                (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS)
            }
            // ---- Fallback
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    // LOGIN_FAIL,
    // NO_AUTH,
    EMPTY_PARAMS,
    INVALID_PARAMS,
    SERVICE_ERROR,
}

// // region: ---- Error boilerplate
// impl std::fmt::Display for Error {
//     fn fmt(&self, fmt: &mut std::fmt::Formatter) -> core::result::Result<(), std::fmt::Error> {
//         write!(fmt, "{self:?}")
//     }
// }
//
// impl std::error::Error for Error {}
// // endregion: ---- Error boilerplate
