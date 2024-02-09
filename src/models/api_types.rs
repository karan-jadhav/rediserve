use std::sync::Arc;

use deadpool_redis::Pool;

use super::ApiError;

pub type JsonValue = serde_json::Value;
pub type RedisValue = redis::Value;

pub type RedisResponse = Result<RedisValue, ApiError>;

pub type SharedRedisPool = Arc<Pool>;
