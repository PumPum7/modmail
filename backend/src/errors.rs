use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde_json::json;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum AppError {
    Anyhow(anyhow::Error),
    Diesel(diesel::result::Error),
    R2D2(diesel::r2d2::PoolError),
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Anyhow(err) => write!(f, "{}", err),
            AppError::Diesel(err) => write!(f, "{}", err),
            AppError::R2D2(err) => write!(f, "{}", err),
        }
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Anyhow(err)
    }
}

impl From<diesel::result::Error> for AppError {
    fn from(err: diesel::result::Error) -> Self {
        AppError::Diesel(err)
    }
}

impl From<diesel::r2d2::PoolError> for AppError {
    fn from(err: diesel::r2d2::PoolError) -> Self {
        AppError::R2D2(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AppError::Anyhow(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            AppError::Diesel(err) => match err {
                diesel::result::Error::NotFound => (StatusCode::NOT_FOUND, "Not found".to_string()),
                _ => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            },
            AppError::R2D2(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Pool error: {}", err),
            ),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
