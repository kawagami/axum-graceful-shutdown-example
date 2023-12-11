use axum::{
    http::status::StatusCode,
    response::{IntoResponse, Response},
};

pub async fn returns_201() -> Response {
    (StatusCode::CREATED, "is 201").into_response()
}
