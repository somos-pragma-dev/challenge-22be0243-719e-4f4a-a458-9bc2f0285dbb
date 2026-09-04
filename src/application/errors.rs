use actix_web::{error, HttpResponse, ResponseError};

#[derive(Debug)]
pub enum AppError {
    NotFound,
    InternalError,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            AppError::NotFound => HttpResponse::NotFound().finish(),
            AppError::InternalError => HttpResponse::InternalServerError().finish(),
        }
    }
}