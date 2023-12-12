mod index;

use axum::{routing::get, Router};

pub async fn create_routes() -> Router {
    Router::new().route("/", get(index::index))
}
