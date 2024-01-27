use redis::RedisError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Redis error: {0}")]
    RedisError(#[from] RedisError),
    #[error("invalid token")]
    InvalidToken,
}
