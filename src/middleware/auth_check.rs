use std::sync::Arc;

use axum::{
    body::Body,
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
    Extension,
};

use crate::state::AppState;

pub async fn check_auth(
    Extension(app_state): Extension<Arc<AppState>>,
    req: Request<Body>,
    next: Next,
) -> Result<Response<Body>, StatusCode> {
    let user_token = match app_state.token.as_ref() {
        Some(token) => token,
        None => return Ok(next.run(req).await),
    };

    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .filter(|header| header.starts_with("Bearer "))
        .map(|header| header.trim_start_matches("Bearer "));

    if let Some(auth_token) = auth_header {
        if auth_token == user_token {
            return Ok(next.run(req).await);
        }
    }

    // If Authorization header is not present or invalid, check for _token query parameter
    let query_params = req.uri().query().unwrap_or("");
    let query_token = url::form_urlencoded::parse(query_params.as_bytes())
        .find(|(key, _)| key == "_token")
        .map(|(_, value)| value.into_owned());

    if query_token == Some(user_token.to_string()) {
        return Ok(next.run(req).await);
    }

    // Err(StatusCode::UNAUTHORIZED)
    // return response {"error": "Unauthorized access"} with status code 401
    return Ok(Response::builder()
        .status(StatusCode::UNAUTHORIZED)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(r#"{"error": "Unauthorized access"}"#))
        .unwrap());
}
