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

#[cfg(test)]
mod tests {

    use axum::http::StatusCode;
    use axum_test::TestServer;

    use super::pipeline_routes;
    use crate::utils::add_layers;
    use crate::utils::app_setup::app_setup;
    use rand::Rng;

    #[tokio::test]
    async fn test_pipeline_route() {
        let (config, app_state) = app_setup();

        let routes = pipeline_routes();

        let app = add_layers(routes, app_state);

        let token = config.token.unwrap();

        let server = match TestServer::new(app) {
            Ok(server) => server,
            Err(e) => panic!("Error setting up test server: {}", e),
        };

        let random_key: String = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();

        let random_value: String = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();

        let response = server
            .post("/pipeline")
            .json(&serde_json::json!([
                ["SET", random_key, random_value],
                ["GET", random_key]
            ]))
            .add_query_param("_token", &token)
            .await;

        response.assert_status(StatusCode::OK);
    }
}
