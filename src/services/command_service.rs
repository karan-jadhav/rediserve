// src/services/command_processor.rs

use deadpool_redis::Connection;

use crate::models::{api_types::RedisResult, ApiError};

/// A service for processing commands and arguments.
pub struct CommandService;

impl CommandService {
    /// Processes a command with its arguments.
    pub async fn process_command(
        command: String,
        args: Vec<&str>,
        mut con: Connection,
    ) -> RedisResult {
        let mut cmd = redis::cmd(&command);

        for arg in args {
            cmd.arg(arg);
        }

        let result: RedisResult = cmd
            .query_async(&mut con)
            .await
            .map_err(ApiError::RedisError);

        return result;
    }
}
