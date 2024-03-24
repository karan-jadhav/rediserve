use crate::{
    models::{
        api_input_data::{ApiInput, ApiInputValue, ExtractEncoding},
        response_builder::ResponseBuilder,
        ApiError, Argument, Command,
    },
    services::CommandService,
};
use axum::{
    extract::Json,
    extract::{Path, Query},
    response::IntoResponse,
    routing::{get, post},
    Extension, Router,
};

use std::{collections::HashMap, sync::Arc};

use crate::state::AppState;

pub async fn command_route_handler(
    Extension(app_state): Extension<Arc<AppState>>,
    encoding: ExtractEncoding,
    path_segments: Option<Path<String>>,
    params: Query<HashMap<String, String>>,
    payload: ApiInput,
) -> impl IntoResponse {
    let mut command_str = String::new();
    let mut arguements: Vec<Argument> = Vec::new();

    let path_segments_present = if let Some(path) = path_segments {
        let path_vaues: Vec<String> = path
            .split("/")
            .filter(|s| !s.is_empty())
            .map(|s| s.trim().to_string())
            .collect();

        if !path_vaues.is_empty() {
            command_str = path_vaues[0].to_string();

            if path_vaues.len() > 1 {
                arguements.extend(path_vaues.iter().skip(1).map(|s| Argument::from(s)));
            }
        }
        true
    } else {
        false
    };

    match payload.0 {
        ApiInputValue::Single(command_value) => {
            if path_segments_present {
                arguements.push(Argument(command_value));
            }
        }
        ApiInputValue::List(command_value_list) => {
            if !path_segments_present {
                if command_value_list.is_empty() {
                    return Json(ResponseBuilder::error(ApiError::NoCommand));
                } else {
                    // remove new line character
                    command_str = command_value_list[0]
                        .to_string()
                        .trim_matches('\"')
                        .to_string();
                    arguements.extend(command_value_list.iter().skip(1).map(|s| Argument::from(s)));
                }
            }
        }
        ApiInputValue::None => {
            if !path_segments_present {
                // return Json(ApiResponse::from(Err(ApiError::NoCommand)));
                return Json(ResponseBuilder::error(ApiError::NoCommand));
            }
        }
    }

    if command_str.is_empty() {
        // return Json(ApiResponse::from(Err(ApiError::NoCommand)));
        return Json(ResponseBuilder::error(ApiError::NoCommand));
    }

    for (key, value) in params.iter() {
        if !key.starts_with("_") {
            arguements.extend(vec![Argument::from(key), Argument::from(value)]);
        }
    }

    let command = Command {
        name: command_str,
        args: arguements,
    };

    let con = app_state.redis_pool.get().await.unwrap();

    let result = CommandService::process_command(command, con).await;

    let response = ResponseBuilder::new(encoding.into_inner()).build(result);

    return response.into();
}

pub fn redis_routes() -> Router {
    Router::new()
        .route("/", post(command_route_handler))
        .route("/*path_segments", get(command_route_handler))
        .route("/*path_segments", post(command_route_handler))
}

#[cfg(test)]
mod tests {

    use axum::http::StatusCode;
    use axum_test::TestServer;

    use super::redis_routes;
    use crate::utils::add_layers;
    use crate::utils::app_setup::app_setup;
    use rand::Rng;

    #[tokio::test]
    async fn test_path_command() {
        let (config, app_state) = app_setup();

        let routes = redis_routes();

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
            .get(format!("/set/{}/{}", random_key, random_value).as_str())
            .add_query_param("_token", &token)
            .await;

        response.assert_status(StatusCode::OK);

        response.assert_json(&serde_json::json!({
            "result": "OK"
        }));

        let response = server
            .get(format!("/get/{}", random_key).as_str())
            .add_query_param("_token", &token)
            .await;

        response.assert_status(StatusCode::OK);

        response.assert_json(&serde_json::json!({
            "result": random_value
        }));
    }

    #[tokio::test]
    async fn test_post_command() {
        let (config, app_state) = app_setup();

        let routes = redis_routes();

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
            .post("/")
            .json(&serde_json::json!(["set", random_key, random_value]))
            .add_query_param("_token", &token)
            .await;

        response.assert_status(StatusCode::OK);

        response.assert_json(&serde_json::json!({
            "result": "OK"
        }));

        let response = server
            .post("/")
            .json(&serde_json::json!(["get", random_key]))
            .add_query_param("_token", &token)
            .await;

        response.assert_status(StatusCode::OK);

        response.assert_json(&serde_json::json!({
            "result": random_value
        }));
    }

    #[tokio::test]
    async fn test_post_command_with_path() {
        let (config, app_state) = app_setup();

        let routes = redis_routes();

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
            .post(format!("/set/{}", random_key).as_str())
            .text(&random_value)
            .add_query_param("_token", &token)
            .await;

        response.assert_status(StatusCode::OK);

        response.assert_json(&serde_json::json!({
            "result": "OK"
        }));

        let response = server
            .get(format!("/get/{}", random_key).as_str())
            .add_query_param("_token", &token)
            .await;

        response.assert_status(StatusCode::OK);

        response.assert_json(&serde_json::json!({
            "result": random_value
        }));
    }
}
