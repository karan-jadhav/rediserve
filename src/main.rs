use axum::{extract::Extension, routing::get};
use axum::{Json, Router};
use rediserve::models::api_response::ApiResponse;
use rediserve::models::api_types::{JsonResponse, RedisResponse};
use rediserve::models::ApiError;
use rediserve::routes::redis_routes::redis_routes;
use rediserve::{config::AppConfig, state::AppState};
use std::sync::Arc;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let config = AppConfig::new();

    let app_state = Arc::new(AppState::new(config.redis_url));

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/redis", get(set_redis_key))
        .merge(redis_routes())
        .layer(Extension(app_state))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    let addr = format!("0.0.0.0:{}", config.server_port);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("Server running on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}

async fn set_redis_key(Extension(app_state): Extension<Arc<AppState>>) -> JsonResponse {
    // let error = ApiError::InvalidToken;

    // return Json(ApiResponse::from(Err(error)));

    let mut con = app_state.redis_pool.get().await.unwrap();

    let result: RedisResponse = redis::cmd("get")
        // .arg("koo")
        // .arg("dar")
        .query_async(&mut con)
        .await
        .map_err(ApiError::RedisError);

    return Json(ApiResponse::from(result));
}
