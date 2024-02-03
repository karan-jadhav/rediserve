use deadpool_redis::Connection;

use crate::models::{api_types::RedisResponse, ApiError, Command};

pub struct CommandService;

impl CommandService {
    pub async fn process_command(command: Command, mut con: Connection) -> RedisResponse {
        let mut cmd = redis::cmd(command.as_ref());

        for arg in command.args.iter() {
            cmd.arg(arg);
        }

        let result: RedisResponse = cmd
            .query_async(&mut con)
            .await
            .map_err(ApiError::RedisError);

        return result;
    }
}
