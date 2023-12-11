use axum::http::StatusCode;

pub async fn always_errors() -> Result<(), StatusCode> {
    if true {
        return Err(StatusCode::IM_A_TEAPOT);
    }

    Ok(())
}
