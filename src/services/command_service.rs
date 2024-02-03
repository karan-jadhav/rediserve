use std::sync::Arc;

use deadpool_redis::{Connection, Pool};
use futures::future::join_all;

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

    pub async fn process_pipeline(
        commands: Vec<Command>,
        redis_pool: Arc<Pool>,
    ) -> Vec<RedisResponse> {
        let futures: Vec<_> = commands
            .into_iter()
            .map(|command| {
                let pool = redis_pool.clone();
                async move {
                    let con = pool.get().await.unwrap();

                    let result = Self::process_command(command, con).await;
                    return result;
                }
            })
            .collect();

        let results: Vec<RedisResponse> = join_all(futures).await;

        return results;
    }

    pub async fn process_transaction(commands: Vec<Command>, mut con: Connection) -> RedisResponse {
        let mut transaction_pipeline = redis::pipe();
        // make the transaction atomic for transactional integrity
        transaction_pipeline.atomic();

        for command in commands {
            transaction_pipeline.cmd(command.name.as_ref());
            for arg in command.args {
                transaction_pipeline.arg(arg);
            }
        }

        let result: RedisResponse = transaction_pipeline
            .query_async(&mut con)
            .await
            .map_err(ApiError::RedisError);

        return result;
    }
}
