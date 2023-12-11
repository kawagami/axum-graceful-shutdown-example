use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Data {
    message: String,
    count: u32,
    username: String,
}

pub async fn get_json() -> Json<Data> {
    let data = Data {
        message: "String".to_owned(),
        count: 456,
        username: "String".to_owned(),
    };

    Json(data)
}
