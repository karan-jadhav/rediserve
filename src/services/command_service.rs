use deadpool_redis::Connection;

use crate::models::{api_types::RedisResult, ApiError, Arguement, Command};

pub struct CommandService;

impl CommandService {
    pub async fn process_command(
        command: Command,
        arguments: Vec<Arguement>,
        mut con: Connection,
    ) -> RedisResult {
        let mut cmd = redis::cmd(command.as_ref());

        for arg in arguments {
            cmd.arg(arg);
        }

        let result: RedisResult = cmd
            .query_async(&mut con)
            .await
            .map_err(ApiError::RedisError);

        return result;
    }
}
