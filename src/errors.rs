use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error, serde::Deserialize, serde::Serialize)]
pub enum MyError {
    #[display(fmt = "internal error")]
    InternalError,

    #[display(fmt = "bad request")]
    BadClientData,

    #[display(fmt = "timeout")]
    Timeout,

    #[display(fmt = "authentication error")]
    AuthError,

    #[display(fmt = "token decoding error")]
    DecodeError,

    #[display(fmt = "token expired")]
    TokenExpirationError,

    #[display(fmt = "unauthorized")]
    Unauthorized
}

impl error::ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            MyError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::BadClientData => StatusCode::BAD_REQUEST,
            MyError::Timeout => StatusCode::GATEWAY_TIMEOUT,
            MyError::AuthError => StatusCode::NOT_FOUND,
            MyError::DecodeError => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::TokenExpirationError => StatusCode::UNAUTHORIZED,
            MyError::Unauthorized => StatusCode::UNAUTHORIZED
        }
    }
}
