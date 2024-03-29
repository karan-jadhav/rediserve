use std::sync::Arc;

use axum::{middleware, Extension, Router};

use crate::{
    cmd::Args,
    config::AppConfig,
    middleware::{check_auth, get_trace_layer},
    state::AppState,
};

pub fn app_setup(args: Args) -> (AppConfig, Arc<AppState>) {
    let config = AppConfig::new(args);
    let app_state = Arc::new(AppState::new(&config));
    (config, app_state)
}

pub fn add_layers(routes: Router, app_state: Arc<AppState>) -> Router {
    return routes
        .layer(get_trace_layer())
        .layer(middleware::from_fn(check_auth))
        .layer(Extension(app_state));
}
