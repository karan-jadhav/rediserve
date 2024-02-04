use rediserve::routes::app_routes;
use rediserve::{config::AppConfig, state::AppState};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let config = AppConfig::new();

    let app_state = Arc::new(AppState::new(config.redis_url));

    let app = app_routes(app_state);

    let addr = format!("0.0.0.0:{}", config.server_port);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("Server running on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}
