mod index;
mod mirror_body_json;
mod mirror_body_string;
mod mirror_user_agent;
mod path_variables;
mod query_params;

use axum::{
    http::Method,
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

pub fn create_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    Router::new()
        .route("/", get(index::get_handler).post(index::post_handler))
        .route("/mirror_body_string", get(mirror_body_string::get_handler))
        .route("/mirror_body_json", post(mirror_body_json::post_handler))
        .route("/path_variables/:id", post(path_variables::post_handler))
        .route("/query_params", get(query_params::get_handler))
        .route("/mirror_user_agent", post(mirror_user_agent::post_handler))
        .layer(cors)
}
