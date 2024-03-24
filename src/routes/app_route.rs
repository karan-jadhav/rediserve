use axum::{body::Body, routing::get, Router};

use super::{pipeline_routes, redis_routes, transaction_routes};

pub fn app_routes() -> Router {
    return Router::new()
        .route(
            "/",
            get(|| async { Body::from(serde_json::json!({"status": "working",}).to_string()) }),
        )
        .merge(redis_routes())
        .merge(pipeline_routes())
        .merge(transaction_routes());
}

#[cfg(test)]
mod tests {
    use axum::http::{header, HeaderValue, StatusCode};
    use axum_test::TestServer;
    use clap::Parser;

    use super::app_routes;
    use crate::cmd::Args;
    use crate::utils::add_layers;
    use crate::utils::app_setup::app_setup;

    #[tokio::test]
    async fn test_unauthorization() {
        let args = Args::parse();

        let (_, app_state) = app_setup(args);

        let routes = app_routes();

        let app = add_layers(routes, app_state);

        let server = match TestServer::new(app) {
            Ok(server) => server,
            Err(e) => panic!("Error setting up test server: {}", e),
        };

        let response = server.get("/").await;

        response.assert_status(StatusCode::UNAUTHORIZED);
        response.assert_json(&serde_json::json!({
            "error": "Unauthorized access"
        }));
    }

    #[tokio::test]
    async fn test_query_authorization() {
        let args = Args::parse();

        let (config, app_state) = app_setup(args);

        let routes = app_routes();

        let app = add_layers(routes, app_state);

        let token = config.token.unwrap();

        let server = match TestServer::new(app) {
            Ok(server) => server,
            Err(e) => panic!("Error setting up test server: {}", e),
        };

        let response = server.get("/").add_query_param("_token", token).await;

        response.assert_status(StatusCode::OK);

        response.assert_json(&serde_json::json!({
            "status": "working"
        }));
    }

    #[tokio::test]
    async fn test_header_authorization() {
        let args = Args::parse();

        let (config, app_state) = app_setup(args);

        let routes = app_routes();

        let app = add_layers(routes, app_state);

        let token = config.token.unwrap();

        let server = match TestServer::new(app) {
            Ok(server) => server,
            Err(e) => panic!("Error setting up test server: {}", e),
        };

        let response = server
            .get("/")
            .add_header(
                header::AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
            )
            .await;

        response.assert_status(StatusCode::OK);

        response.assert_json(&serde_json::json!({
            "status": "working"
        }));
    }
}
