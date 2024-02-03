use std::sync::Arc;

use axum::{routing::post, Extension, Json, Router};

use crate::{
    models::{
        api_response::TransactionApiResponse,
        api_types::{RedisResponse, TransactionJsonResponse},
        multi_api_input_data::MultiApiInput,
        Argument, Command,
    },
    services::CommandService,
    state::AppState,
};

pub async fn transaction_route_handler(
    Extension(app_state): Extension<Arc<AppState>>,
    payload: MultiApiInput,
) -> TransactionJsonResponse {
    let mut command_list: Vec<Command> = vec![];

    for data in payload.0 {
        if data.is_empty() {
            continue;
        }

        let command_str = data[0].to_string().trim_matches('\"').to_string();
        if data.len() > 1 {
            let arguments = data.iter().skip(1).map(|s| Argument::from(s)).collect();
            let command = Command {
                name: command_str,
                args: arguments,
            };
            command_list.push(command);
        } else {
            let command = Command {
                name: command_str,
                args: vec![],
            };
            command_list.push(command);
        }
    }

    let con = app_state.redis_pool.get().await.unwrap();

    let result: RedisResponse = CommandService::process_transaction(command_list, con).await;

    let response = TransactionApiResponse::from(result);

    return Json(response);
}

pub fn transaction_routes() -> Router {
    Router::new().route("/multi-exec", post(transaction_route_handler))
}
