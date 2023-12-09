use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MirrorJson {
    message: String,
}

#[derive(Serialize)]
pub struct MirrorJsonResponse {
    message: String,
    message_from_axum: String,
}

pub async fn post_handler(Json(body): Json<MirrorJson>) -> Json<MirrorJsonResponse> {
    Json(MirrorJsonResponse {
        message: body.message,
        message_from_axum: "from axum response".to_owned(),
    })
}
