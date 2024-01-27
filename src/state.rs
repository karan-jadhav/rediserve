use std::sync::Arc;

use deadpool_redis::{Config, Runtime};

use crate::models::api_types::SharedRedisPool;

#[derive(Clone)]
pub struct AppState {
    pub redis_pool: SharedRedisPool,
}

impl AppState {
    pub fn new(redis_url: String) -> Self {
        let cfg = Config::from_url(redis_url);

        let pool = cfg
            .create_pool(Some(Runtime::Tokio1))
            .expect("Failed to create pool");

        let shared_pool = Arc::new(pool);

        AppState {
            redis_pool: shared_pool,
        }
    }
}
