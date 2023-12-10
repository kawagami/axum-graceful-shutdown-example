use axum::{extract::Query, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct QueryParams {
    id: u32,
    message: String,
}

pub async fn get_handler(Query(query): Query<QueryParams>) -> Json<QueryParams> {
    Json(query)
}
