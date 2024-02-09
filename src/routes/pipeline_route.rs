use axum::{response::IntoResponse, routing::post, Json};
use std::sync::Arc;

use axum::{Extension, Router};

use crate::{
    models::{
        api_input_data::ExtractEncoding, api_types::RedisResponse,
        multi_api_input_data::MultiApiInput, response_builder::ResponseBuilder, Argument, Command,
    },
    services::CommandService,
    state::AppState,
};

pub async fn pipeline_route_handler(
    Extension(app_state): Extension<Arc<AppState>>,
    encoding: ExtractEncoding,
    payload: MultiApiInput,
) -> impl IntoResponse {
    // loop through the payload and process the data

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

    let result: Vec<RedisResponse> =
        CommandService::process_pipeline(command_list, app_state.redis_pool.clone()).await;

    let response = ResponseBuilder::new(encoding.into_inner()).build_pipeline(result);

    return Json(response);
}
pub fn pipeline_routes() -> Router {
    Router::new().route("/pipeline", post(pipeline_route_handler))
}
