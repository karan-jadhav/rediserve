use std::sync::Arc;

use axum::{response::IntoResponse, routing::post, Extension, Json, Router};

use crate::{
    models::{
        api_input_data::ExtractEncoding, api_types::RedisResponse,
        multi_api_input_data::MultiApiInput, response_builder::ResponseBuilder, Argument, Command,
    },
    services::CommandService,
    state::AppState,
};

pub async fn transaction_route_handler(
    Extension(app_state): Extension<Arc<AppState>>,
    encoding: ExtractEncoding,
    payload: MultiApiInput,
) -> impl IntoResponse {
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

    let response = ResponseBuilder::new(encoding.into_inner()).build_transaction(result);

    return Json(response);
}

pub fn transaction_routes() -> Router {
    Router::new().route("/multi-exec", post(transaction_route_handler))
}
