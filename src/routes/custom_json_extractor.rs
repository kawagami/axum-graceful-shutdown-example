use axum::{
    async_trait,
    extract::{FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json, RequestExt,
};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct RequestUser {
    pub username: String,
    #[validate(length(min = 5, max = 10, message = "min 5 max 10"))]
    pub password: String,
}

#[async_trait]
impl<S> FromRequest<S> for RequestUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request(request: Request, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(user) = request
            .extract::<Json<RequestUser>, _>()
            .await
            .map_err(|error| (StatusCode::IM_A_TEAPOT, format!("{}", error)))?;

        // 於此攔截 error
        if let Err(error) = user.validate() {
            return Err((StatusCode::BAD_REQUEST, format!("{}", error)));
        }

        Ok(user)
    }
}

pub async fn custom_json_extractor(user: RequestUser) -> Response {
    (
        StatusCode::CREATED,
        format!(
            "username => {}\npassword => {}",
            user.username, user.password
        ),
    )
        .into_response()
}
