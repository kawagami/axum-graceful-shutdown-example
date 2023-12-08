// use super::structs::User;

use super::structs::{AuthBody, AuthError, AuthPayload, Claims, User, KEYS};

use jsonwebtoken::{encode, Header};

use axum::{
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

pub fn app() -> Router {
    Router::new()
        .route("/hello", get(hello))
        .route("/hello2", post(post_json))
        .route("/jwt", post(jwt))
        .route("/jwt_protected", get(jwt_protected))
}

async fn hello() -> impl IntoResponse {
    "Hello, I'm a Rust/Axum HTTP server!"
}

async fn post_json(Json(payload): Json<User>) -> impl IntoResponse {
    let result = User {
        id: payload.id,
        username: payload.username,
    };
    Json(result)
}

async fn jwt(Json(payload): Json<AuthPayload>) -> Result<Json<AuthBody>, AuthError> {
    // Check if the user sent the credentials
    if payload.client_id.is_empty() || payload.client_secret.is_empty() {
        return Err(AuthError::MissingCredentials);
    }
    // Here you can check the user credentials from a database
    if payload.client_id != "kawa" || payload.client_secret != "gami" {
        return Err(AuthError::WrongCredentials);
    }
    let claims = Claims {
        sub: "b@b.com".to_owned(),
        company: "ACME".to_owned(),
        // Mandatory expiry time as UTC timestamp
        exp: 2000000000, // May 2033
    };
    // Create the authorization token
    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation)?;

    // Send the authorized token
    Ok(Json(AuthBody::new(token)))
}

async fn jwt_protected(claims: Claims) -> Result<String, AuthError> {
    // Send the protected data to the user
    Ok(format!(
        "Welcome to the protected area :)\nYour data:\n{claims}",
    ))
}
