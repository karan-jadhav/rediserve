use std::sync::Arc;

use deadpool_redis::{Config, Runtime};

use crate::{config::AppConfig, models::api_types::SharedRedisPool};

#[derive(Clone)]
pub struct AppState {
    pub redis_pool: SharedRedisPool,
    pub token: Option<String>,
}

impl AppState {
    pub fn new(app_config: &AppConfig) -> Self {
        let cfg = Config::from_url(&app_config.redis_url);

        let pool = cfg
            .create_pool(Some(Runtime::Tokio1))
            .expect("Failed to create pool");

        let shared_pool = Arc::new(pool);

        AppState {
            redis_pool: shared_pool,
            token: app_config.token.clone(),
        }
    }
}
