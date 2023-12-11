use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct RequestUser {
    username: String,
    password: Option<String>,
}

pub async fn validate_with_serde(Json(user): Json<RequestUser>) -> Json<RequestUser> {
    Json(RequestUser {
        username: user.username,
        password: user.password,
    })
}
