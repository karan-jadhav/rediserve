use std::sync::Arc;

use axum::{middleware, routing::get, Extension, Router};

use crate::{
    middleware::{check_auth, get_trace_layer},
    state::AppState,
};

use super::{pipeline_routes, redis_routes, transaction_routes};

pub fn app_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .merge(redis_routes())
        .merge(pipeline_routes())
        .merge(transaction_routes())
        .layer(get_trace_layer())
        .layer(middleware::from_fn(check_auth))
        .layer(Extension(app_state))
}
