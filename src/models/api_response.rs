use crate::utils::redis_value_to_json;
use serde::{Deserialize, Serialize};

use super::{
    api_types::{JsonValue, RedisResponse, RedisValue},
    ApiError,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<JsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

// impl From<RedisResult<RedisValue>> for ApiResponse {
// impl From<Result<RedisValue, ApiError>> for ApiResponse {
impl From<RedisResponse> for ApiResponse {
    // fn from(result: Result<RedisValue, ApiError>) -> Self {
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

impl From<&str> for ApiResponse {
    fn from(result: &str) -> Self {
        ApiResponse {
            result: Some(JsonValue::String(result.to_string())),
            error: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PipelineApiResponse(pub Vec<ApiResponse>);

impl From<Vec<RedisResponse>> for PipelineApiResponse {
    fn from(results: Vec<RedisResponse>) -> Self {
        let mut response_list = vec![];

        for res in results {
            match res {
                Ok(redis_value) => response_list.push(ApiResponse {
                    result: Some(redis_value_to_json(redis_value)),
                    error: None,
                }),
                Err(err) => {
                    response_list.push(match err {
                        ApiError::RedisError(redis_error) => ApiResponse {
                            result: None,
                            error: Some(format!("ERR {}", redis_error.detail().unwrap())),
                        },
                        _ => ApiResponse {
                            result: None,
                            error: Some(err.to_string()),
                        },
                    });
                }
            }
        }

        PipelineApiResponse(response_list)
    }
}

// impl From<Vec<RedisResponse>> for ApiResponse {
//     fn from(results: Vec<RedisResponse>) -> Self {
//         let mut result = vec![];
//         let mut error = vec![];

//         for res in results {
//             match res {
//                 Ok(redis_value) => result.push(redis_value_to_json(redis_value)),
//                 Err(err) => {
//                     error.push(match err {
//                         ApiError::RedisError(redis_error) => {
//                             format!("ERR {}", redis_error.detail().unwrap())
//                         }
//                         _ => err.to_string(),
//                     });
//                 }
//             }
//         }

//         if error.is_empty() {
//             ApiResponse {
//                 result: Some(JsonValue::Array(result)),
//                 error: None,
//             }
//         } else {
//             ApiResponse {
//                 result: Some(JsonValue::Array(result)),
//                 error: Some(error.join("\n")),
//             }
//         }
//     }
// }
