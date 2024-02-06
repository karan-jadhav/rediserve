use std::sync::Arc;

use crate::{config::AppConfig, routes::app_routes, state::AppState};

pub async fn start_server() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let config = AppConfig::new();

    let app_state = Arc::new(AppState::new(&config));

    let app = app_routes(app_state);

    let addr = format!("0.0.0.0:{}", config.server_port);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("Server running on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    use crate::{config::AppConfig, routes::app_routes, state::AppState};

    #[tokio::test]
    async fn test_app_routes() {
        let app_state = Arc::new(AppState::new(&AppConfig::new()));

        let app = app_routes(app_state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/set/foo/bar")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
