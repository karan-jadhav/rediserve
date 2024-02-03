use crate::{
    models::{
        api_input_data::{ApiInput, ApiInputValue},
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

use crate::{models::api_response::ApiResponse, state::AppState};

pub async fn command_route_handler(
    Extension(app_state): Extension<Arc<AppState>>,
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
                    return Json(ApiResponse::from(Err(ApiError::NoCommand)));
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
                return Json(ApiResponse::from(Err(ApiError::NoCommand)));
            }
        }
    }

    if command_str.is_empty() {
        return Json(ApiResponse::from(Err(ApiError::NoCommand)));
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

    return Json(ApiResponse::from(result));
}

pub fn redis_routes() -> Router {
    Router::new()
        .route("/", post(command_route_handler))
        .route("/*path_segments", get(command_route_handler))
        .route("/*path_segments", post(command_route_handler))
}
