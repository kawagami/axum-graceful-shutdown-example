use axum::{routing::get, Router};

pub fn create_routes() -> Router {
    // Router::new().merge(api::app())
    Router::new().route("/", get(|| async { "this is ver2" }))
}
