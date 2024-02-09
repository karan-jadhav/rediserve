use crate::utils::redis_value_to_json;
use serde::Serialize;

use super::{
    api_types::{JsonValue, RedisResponse, RedisValue},
    ApiError,
};

/// Represents the standard response structure of an API call.
/// It can optionally contain a result or an error message, but not both.
#[derive(Debug, Serialize)]
pub struct ApiResponse {
    // The result of the API call, serialized as JSON. Skipped during serialization if None.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<JsonValue>,

    // Error message string, if the API call resulted in an error. Skipped during serialization if None.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Represents a collection of ApiResponse, typically used for pipeline operations.
#[derive(Debug, Serialize)]
pub struct PipelineApiResponse(pub Vec<ApiResponse>);

/// Enumerates possible response types for a transaction API call.
/// It can either be a list of ApiResponse for successful transactions,
/// or a single ApiResponse for an error.
#[derive(Serialize, Debug)]
pub enum TransactionApiResponseType {
    TransactionResponse(Vec<ApiResponse>),
    TransactionError(ApiResponse),
}

/// Wrapper around the TransactionApiResponseType to provide a unified interface.
#[derive(Debug, Serialize)]
pub struct TransactionApiResponse(pub TransactionApiResponseType);

/// A builder for creating ApiResponse objects, potentially with different encodings.
pub struct ResponseBuilder {
    // Optional encoding specification for the response.
    encoding: String,
}

impl ResponseBuilder {
    /// Constructs a new ResponseBuilder with an optional encoding.
    pub fn new(encoding: String) -> ResponseBuilder {
        ResponseBuilder { encoding }
    }

    /// Builds an ApiResponse from a Redis operation result. Utilizes the specified encoding
    /// for converting Redis values to JSON format.
    pub fn build(&self, result: Result<RedisValue, ApiError>) -> ApiResponse {
        match result {
            Ok(redis_value) => {
                let json_value = redis_value_to_json(redis_value, self.encoding.as_str());
                ApiResponse {
                    result: Some(json_value),
                    error: None,
                }
            }
            Err(api_error) => ApiResponse {
                result: None,
                error: match api_error {
                    ApiError::RedisError(redis_error) => match redis_error.detail() {
                        Some(detail) => Some(format!("ERR {}", detail)),
                        None => Some("ERR".to_string()),
                    },
                    _ => Some(api_error.to_string()),
                },
            },
        }
    }

    /// Builds a PipelineApiResponse from a vector of Redis operation results.
    /// This is typically used for pipeline operations where multiple commands are sent in a batch.
    pub fn build_pipeline(&self, result: Vec<Result<RedisValue, ApiError>>) -> PipelineApiResponse {
        let mut response_list = vec![];

        for res in result {
            response_list.push(self.build(res));
        }

        PipelineApiResponse(response_list)
    }

    /// Builds a TransactionApiResponse from a transaction result.
    /// A successful transaction will contain a list of ApiResponse objects,
    /// whereas a failed transaction will contain a single ApiResponse with an error.
    pub fn build_transaction(&self, result: RedisResponse) -> TransactionApiResponse {
        match result {
            Ok(redis_values) => {
                let mut response_list = vec![];
                match redis_values {
                    RedisValue::Bulk(items) => {
                        for item in items {
                            response_list.push(self.build(Ok(item)));
                        }
                    }
                    _ => {
                        response_list.push(self.build(Ok(redis_values)));
                    }
                }
                TransactionApiResponse(TransactionApiResponseType::TransactionResponse(
                    response_list,
                ))
            }
            Err(api_error) => {
                let response = self.build(Err(api_error));
                TransactionApiResponse(TransactionApiResponseType::TransactionError(response))
            }
        }
    }

    /// Creates an ApiResponse indicating an error, based solely on the ApiError provided.
    /// This is useful for generating error responses without a specific Redis value.
    pub fn error(error: ApiError) -> ApiResponse {
        ApiResponse {
            result: None,
            error: Some(error.to_string()),
        }
    }

    /// Creates an ApiResponse with a successful result, represented as a string.
    /// This can be used for simple string responses or messages mostly for testing purposes.
    pub fn from_str(result: &str) -> ApiResponse {
        ApiResponse {
            result: Some(JsonValue::String(result.to_string())),
            error: None,
        }
    }
}
