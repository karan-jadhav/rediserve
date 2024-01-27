use std::sync::Arc;

use axum::{extract::Path, routing::get, Extension, Json, Router};

use crate::{
    models::{
        api_response::ApiResponse,
        api_types::{JsonResponse, RedisResult},
    },
    services::CommandService,
    state::AppState,
};

pub async fn redis_routes_handler(
    Path((command, args)): Path<(String, String)>,
    Extension(app_state): Extension<Arc<AppState>>,
) -> JsonResponse {
    let command = command.to_lowercase();

    let arg_segments: Vec<&str> = args
        .split('/')
        .filter(|s| !s.is_empty())
        .map(|s| s.trim())
        .collect();

    let con = app_state.redis_pool.get().await.unwrap();

    let result: RedisResult = CommandService::process_command(command, arg_segments, con).await;

    Json(ApiResponse::from(result))
}

pub fn redis_routes() -> Router {
    Router::new().route("/:command/*args", get(redis_routes_handler))
}
