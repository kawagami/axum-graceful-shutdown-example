mod v0;

use axum::Router;

pub fn app() -> Router {
    Router::new().nest("/v0", v0::app())
}
