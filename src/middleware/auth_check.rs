use std::sync::Arc;

use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
    Extension,
};

use crate::state::AppState;

pub async fn check_auth(
    Extension(app_state): Extension<Arc<AppState>>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let user_token = app_state.token.as_ref();

    let token = if let Some(token) = user_token {
        token
    } else {
        return Ok(next.run(req).await);
    };

    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    // check if it starts with "Bearer"
    if !auth_header.starts_with("Bearer") {
        return Err(StatusCode::UNAUTHORIZED);
    } else {
        // check if the token is valid
        let auth_token = auth_header.trim_start_matches("Bearer").trim();
        if auth_token != token {
            return Err(StatusCode::UNAUTHORIZED);
        } else {
            return Ok(next.run(req).await);
        }
    }
}
