use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;

#[derive(Debug)]
pub enum ResponseError<E = String> {
    Status(StatusCode),
    StatusMessage((StatusCode, E)),
}

impl<E> IntoResponse for ResponseError<E>
where
    E: ToString,
{
    fn into_response(self) -> axum::response::Response {
        #[derive(Serialize)]
        struct ErrorResponse {
            error: String,
        }

        match self {
            Self::Status(code) => code.into_response(),
            Self::StatusMessage((code, msg)) => (
                code,
                Json(ErrorResponse {
                    error: msg.to_string(),
                }),
            )
                .into_response(),
        }
    }
}
