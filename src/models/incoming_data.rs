use axum::{
    async_trait,
    body::{Body, Bytes},
    extract::{FromRequest, Request},
    http::header::HeaderName,
    http::HeaderValue,
    response::Response,
};
use redis::ToRedisArgs;
use serde::Deserialize;

use super::api_types::JsonValue;

#[derive(Debug, Deserialize)]
pub struct Command(pub String);

impl AsRef<str> for Command {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Deserialize)]
pub struct Arguement(pub JsonValue);

impl From<&JsonValue> for Arguement {
    fn from(arg: &JsonValue) -> Self {
        match arg {
            JsonValue::String(string) => Arguement(JsonValue::String(string.to_string())),
            JsonValue::Number(number) => {
                // if float then convert to string else NumberArg
                if let Some(int) = number.as_i64() {
                    Arguement(JsonValue::Number(int.into()))
                } else {
                    Arguement(JsonValue::String(number.to_string()))
                }
            }
            JsonValue::Bool(boolean) => Arguement(JsonValue::Bool(*boolean)),
            _ => Arguement(JsonValue::Null),
        }
    }
}
impl From<&String> for Arguement {
    fn from(arg: &String) -> Self {
        // check for types
        if let Ok(int) = arg.parse::<i64>() {
            Arguement(JsonValue::Number(int.into()))
        } else if let Ok(boolean) = arg.parse::<bool>() {
            Arguement(JsonValue::Bool(boolean))
        } else {
            Arguement(JsonValue::String(arg.to_string()))
        }
    }
}

impl ToRedisArgs for Arguement {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        match &self.0 {
            JsonValue::String(string) => string.write_redis_args(out),
            JsonValue::Number(number) => {
                // if float then convert to string else NumberArg
                if let Some(int) = number.as_i64() {
                    int.write_redis_args(out)
                } else {
                    number.to_string().write_redis_args(out)
                }
            }
            JsonValue::Null => "".write_redis_args(out),
            JsonValue::Bool(boolean) => boolean.write_redis_args(out),
            JsonValue::Object(_) => "".write_redis_args(out),
            JsonValue::Array(_) => "".write_redis_args(out),
        }
    }
}

#[derive(Deserialize, Debug)]
pub enum ApiInputValue {
    Single(JsonValue),
    List(Vec<JsonValue>),
    None,
}

pub struct ApiInput(pub ApiInputValue);

#[async_trait]
impl<S> FromRequest<S> for ApiInput
where
    Bytes: FromRequest<S>,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        // check content type
        let content_type = req.headers().get(HeaderName::from_static("content-type"));

        match content_type {
            None => {
                return Ok(ApiInput(ApiInputValue::None));
            }
            Some(content_type) => {
                if content_type == HeaderValue::from_static("application/json") {
                    let body = Bytes::from_request(req, state).await.map_err(|_| {
                        Response::builder()
                            .status(400)
                            .body(Body::from("invalid body"))
                            .unwrap()
                    })?;

                    // parse json from bytes

                    let deserialized = serde_json::from_slice(&body)
                        .map_err(|e| {
                            Response::builder()
                                .status(400)
                                .body(Body::from(format!("invalid json: {}", e)))
                                .unwrap()
                        })
                        .map(|s| ApiInput(ApiInputValue::List(s)))?;

                    return Ok(deserialized);
                } else {
                    let body = Bytes::from_request(req, state).await.map_err(|_| {
                        Response::builder()
                            .status(400)
                            .body(Body::from("invalid body"))
                            .unwrap()
                    })?;

                    let deserialized: ApiInput = String::from_utf8(body.to_vec())
                        .map_err(|_| {
                            Response::builder()
                                .status(400)
                                .body(Body::from("invalid utf8"))
                                .unwrap()
                        })
                        .map(|s| ApiInput(ApiInputValue::Single(JsonValue::String(s))))?;

                    return Ok(deserialized);
                }
            }
        }
    }
}
