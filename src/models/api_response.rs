use crate::utils::redis_value_to_json;
use serde::{Deserialize, Serialize};

use super::{
    api_types::{JsonValue, RedisValue},
    ApiError,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse {
    pub result: Option<JsonValue>,
    pub error: Option<String>,
}

// impl From<RedisResult<RedisValue>> for ApiResponse {
impl From<Result<RedisValue, ApiError>> for ApiResponse {
    fn from(result: Result<RedisValue, ApiError>) -> Self {
        match result {
            Ok(redis_value) => ApiResponse {
                result: Some(redis_value_to_json(redis_value)),
                error: None,
            },
            Err(err) => ApiResponse {
                result: None,
                // error: Some(ApiError::from(err).to_string()),
                error: match err {
                    ApiError::RedisError(redis_error) => {
                        Some(format!("ERR {}", redis_error.detail().unwrap()))
                    }
                    _ => Some(err.to_string()),
                },
            },
        }
    }
}
