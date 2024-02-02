use std::sync::Arc;

use axum::Json;
use deadpool_redis::Pool;

use super::{api_response::ApiResponse, ApiError};

pub type JsonValue = serde_json::Value;
pub type RedisValue = redis::Value;
pub type RedisArguement<T> = redis::Arg<T>;
pub type RedisResponse = Result<RedisValue, ApiError>;
pub type JsonResponse = Json<ApiResponse>;

pub type SharedRedisPool = Arc<Pool>;
