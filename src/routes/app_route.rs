use std::sync::Arc;

use axum::{routing::get, Extension, Router};

use crate::{middleware::get_trace_layer, state::AppState};

use super::{pipeline_routes, redis_routes, transaction_routes};

pub fn app_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .merge(redis_routes())
        .merge(pipeline_routes())
        .merge(transaction_routes())
        .layer(Extension(app_state))
        .layer(get_trace_layer())
}
