use axum::{http::StatusCode, response::IntoResponse};

pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let tup = (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", self.0));
        tup.into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
