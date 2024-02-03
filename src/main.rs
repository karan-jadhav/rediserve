use axum::{extract::Extension, routing::get};
use axum::{Json, Router};
use futures::future::join_all;
use rediserve::models::api_response::ApiResponse;
use rediserve::models::api_types::{JsonResponse, RedisResponse};
use rediserve::models::{Argument, Command};
use rediserve::routes::{pipeline_routes, redis_routes};

use rediserve::services::CommandService;
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
        .merge(pipeline_routes())
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
    let command_str = "set";
    let argument1 = Argument::from(&"koo".to_owned());
    let argument2 = Argument::from(&"dar".to_owned());
    let command1 = Command {
        name: command_str.to_string(),
        args: vec![argument1, argument2],
    };

    let command_str = "get";
    let argument1 = Argument::from(&"koo".to_owned());
    let command2 = Command {
        name: command_str.to_string(),
        args: vec![argument1],
    };

    let command_list = vec![command1, command2];

    // let mut con = app_state.redis_pool.get().await.unwrap();

    let futures: Vec<_> = command_list
        .into_iter()
        .map(|command| {
            let pool = app_state.redis_pool.clone();
            async move {
                let con = pool.get().await.unwrap();
                let result = CommandService::process_command(command, con).await;
                return result;
            }
        })
        .collect();

    let results: Vec<RedisResponse> = join_all(futures).await;

    println!("{:?}", results);

    return Json(ApiResponse::from("result"));
}
