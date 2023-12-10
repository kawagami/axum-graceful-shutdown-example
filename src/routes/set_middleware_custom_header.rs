use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};

use super::read_middleware_custom_header::HeaderMessage;

pub async fn set_middleware_custom_header(
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let headers = request.headers();

    let message = headers
        .get("testtest")
        .ok_or_else(|| StatusCode::BAD_REQUEST)?;

    let message = message
        .to_str()
        .map_err(|_err| StatusCode::BAD_REQUEST)?
        .to_owned();
    let extensions = request.extensions_mut();

    extensions.insert(HeaderMessage(message));

    Ok(next.run(request).await)
}
